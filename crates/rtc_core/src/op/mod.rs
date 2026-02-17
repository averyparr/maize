use std::ops::{Add, Div, Mul, Sub};

use crate::{
    codegen::{
        func_with_args::create_func,
        intrinsics::{
            UnaryIntrinsic,
            cuda::{Exp2Approx, Exp2Fast},
        },
    },
    traits::vec::BulkOps,
    ty::{
        V, Void,
        primitive::{BF16, F16, F32, F64},
        ptr::{Global, M, R},
    },
    val::Val,
};

pub trait Ex2Like {
    fn ex2(self) -> Self;
    fn ex2_fast(self) -> Self;
}

impl<T> Ex2Like for Val<'_, T>
where
    T: UnaryIntrinsic<Exp2Approx> + UnaryIntrinsic<Exp2Fast>,
{
    fn ex2(self) -> Self {
        self.__intrinsic_ex2_approx()
    }
    fn ex2_fast(self) -> Self {
        self.__intrinsic_exp2_fast()
    }
}

impl<'a, T, const N: usize> Ex2Like for Val<'a, V<T, N>>
where
    T: BulkOps,
    Val<'a, T>: Ex2Like,
    Val<'a, T::BulkT>: Ex2Like,
{
    fn ex2(self) -> Self {
        self.map_bulk(|e| e.ex2(), |b| b.ex2()).get()
    }

    fn ex2_fast(self) -> Self {
        self.map_bulk(|e| e.ex2_fast(), |b| b.ex2_fast()).get()
    }
}

impl Ex2Like for Val<'_, BF16> {
    fn ex2(self) -> Self {
        self.cast::<F32>().ex2().cast()
    }
    fn ex2_fast(self) -> Self {
        self.cast::<F32>().ex2_fast().cast()
    }
}

impl<const N: usize> Ex2Like for Val<'_, V<BF16, N>> {
    fn ex2(self) -> Self {
        self.map_elementwise(|e| e.ex2()).get()
    }
    fn ex2_fast(self) -> Self {
        self.map_elementwise(|e| e.ex2_fast()).get()
    }
}

pub trait FloatLike: Ex2Like + Sized + Add + Sub + Mul + Div {}

fn test_inner() {
    type T = F32; //V<F32, 4>;
    let func = create_func::<(Global<M<&mut T>>, R<&T>), Void>();
    let (mut to_load, other) = func.get_args();
    to_load.store(
        to_load
            .load()
            .__intrinsic_ex2_approx()
            .__intrinsic_log2_approx(),
    );
    let ptx = func
        .finalize()
        .compile_to_ptx(crate::codegen::target::SM::SM90);
    let ptx = String::from_utf8(ptx.to_vec()).expect("Should work");
    println!("{}", ptx);
    assert!(false);
}

#[test]
fn test() {
    test_inner()
}
