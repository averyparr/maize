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
    let kernel = new_ptx_kernel::<(
        Global<R<&Fl>>,
        Global<R<&Fl>>,
        Global<M<&mut Fl>>,
        Global<M<&mut Fl>>,
    )>();
    kernel.use_fast_math();
    let (a, b, mut c, mut d) = kernel.get_args();

    let should_store = a.load_nc().eq(b.load_nc());
    should_store.branch(|| c.store(a.load_nc()));
    let res = should_store.then(|| a.load_nc()).or_else(|| b.load_nc());
    d.store(res);
    let res = kernel.finalize().compile_asm_quickly(SM::SM90);
    println!("{res}");
    assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
