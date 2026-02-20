use crate::codegen::{Func, new_ptx_device, new_ptx_kernel, target::cuda::SM};

mod codegen;
mod ty;
mod val;

use inkwell::values::AnyValue;
use ty::raw::*;

fn test_inner() {
    let kernel = new_ptx_kernel::<(F64, F64, P<F64>)>();
    let (a, b, c) = kernel.get_args();
    println!("{:?}", a.raw().print_to_string());
    unsafe { c.store_unchecked(a + b) };
    let res = kernel.finalize().compile_asm_optimized(SM::SM90);
    println!("{res}");
    assert!(false);
}

#[test]
fn test() {
    test_inner();
}
