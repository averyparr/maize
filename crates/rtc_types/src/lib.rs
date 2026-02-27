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
    kernel.use_fast_math();
    let (a, b, mut c) = kernel.get_args();

    let max = b.load();
    for idx in Loop::new(max.const_like(4)..10 * max) {
        c.store(idx);
    }

    println!("{}", kernel.finalize().compile_asm_quickly(SM::SM90));
    // assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
