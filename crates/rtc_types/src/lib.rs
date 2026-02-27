use crate::{
    codegen::{Func, new_ptx_kernel, target::cuda::SM},
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
    let kernel = new_ptx_kernel::<(
        Global<R<&Fl>>,
        Global<R<&Fl>>,
        Global<M<&mut Fl>>,
        Global<M<&mut Fl>>,
        Global<R<&V<FlAlt, VEC_LEN>>>,
        Global<M<&mut V<FlAlt, VEC_LEN>>>,
    )>();
    kernel.use_fast_math();
    let (a, b, mut c, mut d, e, mut f) = kernel.get_args();

    let e = e.load();
    let a = a.load();
    c.store((1.0 / a.sqrt()).log2());
    f.store((a.const_like(2.3) + e.vec_cast().exp2() * a).vec_cast());

    println!("{}", kernel.finalize().compile_asm_quickly(SM::SM90));
    assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
