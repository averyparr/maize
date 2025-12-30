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
    ty::{F16, F32, F64, Global, M, R, VF, Void},
    val::C,
};

type Fl = F16;

fn main() {
    let func = create_kernel::<(Global<R<VF<Fl, 4>>>, Global<R<VF<Fl, 2>>>, Global<M<Fl>>)>();
    let (a_ptr, b_ptr, mut c_ptr) = func.get_args();

    let bsum = b_ptr.load().float_cast::<VF<F32, 2>>().sum();
    let asum = a_ptr.load().sum().float_cast::<F32>();
    let asum = func.rcp_approx_ftz(func.sqrt_approx_ftz(asum));
    let asum = asum + bsum;

    let mult = func.const_val(3.5);

    c_ptr.store(func.exp2_ftz(mult * bsum + asum).float_cast::<F16>());

    let to_jit = func.finalize();
    // println!("{}", to_jit.as_llvm_ir());
    let out = to_jit.compile(
        TargetMachine::PTX(PTXOptions { sm: SM::SM80 }),
        OptimizationLevel::Aggressive,
        FileType::Assembly,
    );
    let s = str::from_utf8(&out).expect("Should be valid UTF-8");
    println!("{}", s);
}
