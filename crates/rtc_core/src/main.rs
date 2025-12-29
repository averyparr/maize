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
    ty::{F32, F64, Global, M, R, VF, Void},
    val::C,
};

type Fl = F32;

fn main() {
    let func = create_kernel::<(Global<R<VF<Fl, 4>>>, Global<R<VF<Fl, 2>>>, Global<M<Fl>>)>();
    let (a_ptr, b_ptr, mut c_ptr) = func.get_args();

    let bsum = b_ptr.load().sum();
    let asum = func.sqrt_approx_ftz(a_ptr.load().sum());
    let asum = func.sqrt_approx_ftz(asum);
    let mult = func.const_val(3.5);
    let asum = func.fmax(asum, func.const_val(2.9));

    func.nanosleep(func.const_val(1000));

    c_ptr.store(mult * bsum + asum);

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
