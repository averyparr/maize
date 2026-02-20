use crate::codegen::{Func, new_ptx_device, new_ptx_kernel, target::cuda::SM};

mod codegen;
mod ty;
mod val;

use inkwell::values::AnyValue;
use ty::raw::*;

type Fl = F32;

fn test_inner() {
    let kernel = new_ptx_kernel::<(Fl, Fl, M<&mut Fl>)>();
    let (a, b, mut c) = kernel.get_args();
    println!("{:?}", a.raw().print_to_string());
    c.store(a + b);
    let res = kernel.finalize().compile_asm_optimized(SM::SM90);
    println!("{res}");
    assert!(false);
}

#[test]
fn test() {
    test_inner();
}
