mod codegen;
mod primitives;
mod ty;
mod val;

use inkwell::{OptimizationLevel, targets::FileType};

use crate::{
    codegen::{
        func_with_args::{create_func, create_kernel},
        target::{PTXOptions, SM, TargetMachine},
    },
    primitives::{MutOps, RefOps},
    ty::{F32, F64, Global, M, R, Void},
    val::C,
};

fn main() {
    let func = create_kernel::<(Global<R<F32>>, Global<R<F32>>, Global<M<F64>>)>();
    let (a_ptr, b_ptr, mut c_ptr) = func.get_args();

    let a = a_ptr.load();
    let b = b_ptr.load();
    let c = a + b;
    let mut c_stor = c.with_storage();
    let mut c_mut = c_stor.get_mut();
    let const_float = func.const_val(5.0);
    c_mut.store(b + c + const_float);

    let c_f64 = a * (c_stor.get() + b);
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
}
