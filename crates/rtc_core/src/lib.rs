mod codegen;
mod primitives;
mod ty;
mod val;

mod test {
    use inkwell::{OptimizationLevel, targets::FileType};

    use crate::{
        codegen::{
            func_with_args::create_func,
            target::{PTXOptions, SM, TargetMachine},
        },
        ty::{F32, F64, M, R, Void},
        val::C,
    };

    use super::*;

    #[test]
    fn empty_test() {
        let func = create_func::<(R<F32>, R<F32>, M<F64>), Void>();
        let (a_ptr, b_ptr, mut c_ptr) = func.get_args();

        let a = a_ptr.load();
        let b = b_ptr.load();
        let c = a + b;
        let mut c_stor = c.with_storage();
        let mut c_mut = c_stor.get_mut();
        let const_float = func.const_val(0.0);
        c_mut.store(b + c + const_float);

        let t = func.const_val(3.0);
        let c_f64 = c_stor.get() + t;
        let e = (c_f64 + C(5.0)).float_cast::<F64>();
        c_ptr.store(e);

        let to_jit = func.finalize();
        let out = to_jit.compile(
            TargetMachine::PTX(PTXOptions { sm: SM::SM80 }),
            OptimizationLevel::Aggressive,
            FileType::Assembly,
        );
        let s = str::from_utf8(&out).expect("Should be valid UTF-8");
        println!("{}", s);

        assert!(false);
    }
}
