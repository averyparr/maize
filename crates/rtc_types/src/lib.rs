use crate::{
    codegen::{Func, new_ptx_device, new_ptx_kernel, target::cuda::SM},
    ty::cuda::{Global, Shared},
};

mod codegen;
mod ty;
mod val;

use inkwell::values::AnyValue;
use ty::raw::*;

type Fl = F32;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Global<R<&Fl>>, R<&Fl>, Shared<R<&Fl>>, Global<M<&mut Fl>>)>();
    kernel.use_fast_math();
    let (a, b, c, mut d) = kernel.get_args();
    d.store(a.load_nc() + b.load() * c.load());
    let res = kernel.finalize().compile_asm_optimized(SM::SM90);
    println!("{res}");
    assert!(false);
}

#[test]
fn test() {
    test_inner();
}
