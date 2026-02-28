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

type Fl = F32;
type FlAlt = F32;

const VEC_LEN: usize = 4;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Global<R<&Fl>>, Global<R<&U32>>, Global<M<&mut Fl>>)>();
    const LDSM_SIZE: usize = 8 * 8;
    let mut c_shared = kernel
        .intrinsics()
        .alloc_aligned_shared::<[Fl; LDSM_SIZE]>(16);
    kernel.use_fast_math();
    let (a, b, mut c) = kernel.get_args();

    let a = a.load();
    let mut idx_mut = c_shared.index_mut(10);
    idx_mut.store(a);

    kernel.cx().module().print_to_stderr();

    println!("{}", kernel.finalize().compile_asm_quickly(SM::SM90));
    // assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
