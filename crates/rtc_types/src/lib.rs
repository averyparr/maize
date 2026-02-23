use crate::{
    codegen::{Func, if_stmt::ControlFlow, new_ptx_device, new_ptx_kernel, target::cuda::SM},
    ty::cuda::{Global, Shared},
};

mod codegen;
mod ty;
mod val;

use inkwell::values::AnyValue;
use ty::raw::*;

type Fl = F32;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Bool, Global<R<&Fl>>, Global<M<&mut Fl>>, Global<M<&mut Fl>>)>();
    kernel.use_fast_math();
    let (a, b, mut c, mut d) = kernel.get_args();

    let res = a
        .then(|| {
            let ret = d.load();
            d.store(b.load());
            kernel.return_void();
            ret
        })
        .or_else(|| {
            c.store(c.cx().constant(0.0));
            c.cx().constant(0.0)
        });
    d.store(res);
    kernel.cx().func().print_to_stderr();
    let res = kernel.finalize().compile_asm_optimized(SM::SM90);
    println!("{res}");
    assert!(false);
}

#[test]
fn test() {
    test_inner();
}
