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
    let kernel = new_ptx_kernel::<(Global<R<&Fl>>, Global<R<&U32>>, Global<M<&mut Fl>>)>();
    kernel.use_fast_math();
    let (a, b, mut c) = kernel.get_args();

    let mut a = a.load();
    let max = b.load();
    let mut a_prime = a.clone();
    let mut a_prime = a_prime.as_mut();
    for _ in Loop::new(max.const_like(4)..10 * max) {
        let mut a_mut = a.as_mut();
        a_mut.store(a_mut.load() * a_mut.load());
        a_prime.store(a_mut.load() * c.load());
    }

    c.store(a_prime.load());

    println!("{}", kernel.finalize().compile_asm_quickly(SM::SM90));
    // assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
