use crate::{
    codegen::{Func, loops::Loop, new_ptx_kernel, target::cuda::SM},
    ty::cuda::Global,
    val::Val,
};

pub mod codegen;
pub mod intrinsics;
pub mod ty;
pub mod val;

use ty::raw::*;

type Fl = F16;
type FlAlt = F32;

const VEC_LEN: usize = 4;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Global<R<&Fl>>, Global<R<&U32>>, Global<M<&mut U32>>)>();
    let mut c_shared = kernel.intrinsics().shared_global::<V<U32, 4>>();
    kernel.use_fast_math();
    let (a, b, mut c) = kernel.get_args();

    let mut c_local = c.load();

    let max = b.load();
    for idx in Loop::new(max.const_like(4)..10 * max.copy()) {
        c_local.as_mut().store(idx.copy());
        c_shared.store(Val::splat(c.load()));
    }

    c.store(c_local);

    println!("{}", kernel.finalize().compile_asm_quickly(SM::SM90));
    assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
