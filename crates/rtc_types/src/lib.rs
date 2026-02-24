use crate::{
    codegen::{Func, new_ptx_kernel, target::cuda::SM},
    ty::cuda::Global,
};

pub mod codegen;
pub mod intrinsics;
pub mod ty;
pub mod val;

use ty::raw::*;

type Fl = F16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        Global<R<&Fl>>,
        Global<R<&Fl>>,
        Global<M<&mut Fl>>,
        Global<M<&mut Fl>>,
        Global<R<&V<Fl, 4>>>,
        Global<M<&mut V<F32, 4>>>,
    )>();
    kernel.use_fast_math();
    let (a, b, mut c, mut d, e, mut f) = kernel.get_args();

    let e = e.load();
    c.store(a.load().log2());
    d.store(b.load().cos());
    f.store(e.vec_cast::<BF16>().exp2().vec_cast());

    kernel.cx().module().print_to_stderr();

    println!("{}", kernel.finalize().compile_asm_quickly(SM::SM90));
    // assert!(false);
}

#[test]
fn test_codegen() {
    test_inner();
}
