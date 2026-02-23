use crate::{
    codegen::{Func, new_ptx_kernel, target::cuda::SM},
    ty::cuda::Global,
};

pub mod codegen;
pub mod intrinsics;
pub mod ty;
pub mod val;

use ty::raw::*;

type Fl = F32;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Bool, Global<R<&Fl>>, Global<M<&mut Fl>>, Global<M<&mut Fl>>)>();
    kernel.use_fast_math();
    let (a, b, c, mut d) = kernel.get_args();

    let res = a.then(|| b.load()).or_else(|| b.load());
    // let res = b.load();
    d.store(res);

    let mut s = res.with_storage();
    s.as_mut().store(c.load());
    d.store(s.as_ref().load());
    d.store(b.load_nc());
    kernel.cx().func().print_to_stderr();
    let res = kernel.finalize().compile_asm_quickly(SM::SM90);
    println!("{res}");
    assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
