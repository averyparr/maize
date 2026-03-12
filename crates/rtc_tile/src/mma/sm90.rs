use rtc_types::{
    ty::{F64, V},
    val::Val,
};

use crate::mma::{IntrinsicSyncMMAOp, WarpRetF64_16x8};

pub struct Sm90MmaF64F64_16x8x4;
pub struct Sm90MmaF64F64_16x8x8;
pub struct Sm90MmaF64F64_16x8x16;

impl IntrinsicSyncMMAOp for Sm90MmaF64F64_16x8x4 {
    type AFrag = V<F64, 2>;
    type BFrag = V<F64, 1>;
    type CFrag = V<F64, 4>;

    type Args = (F64, F64, F64, F64, F64, F64, F64);
    type Ret = WarpRetF64_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k4.row.col.f64";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as rtc_types::ty::IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1] = a.elements();
        let [b0] = b.elements();
        let [c0, c1, c2, c3] = c.elements();
        (a0, a1, b0, c0, c1, c2, c3)
    }
}

impl IntrinsicSyncMMAOp for Sm90MmaF64F64_16x8x8 {
    type AFrag = V<F64, 4>;
    type BFrag = V<F64, 2>;
    type CFrag = V<F64, 4>;

    type Args = (F64, F64, F64, F64, F64, F64, F64, F64, F64, F64);
    type Ret = WarpRetF64_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k8.row.col.f64";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as rtc_types::ty::IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3] = a.elements();
        let [b0, b1] = b.elements();
        let [c0, c1, c2, c3] = c.elements();
        (a0, a1, a2, a3, b0, b1, c0, c1, c2, c3)
    }
}

impl IntrinsicSyncMMAOp for Sm90MmaF64F64_16x8x16 {
    type AFrag = V<F64, 8>;
    type BFrag = V<F64, 4>;
    type CFrag = V<F64, 4>;

    type Args = (
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
        F64,
    );
    type Ret = WarpRetF64_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k16.row.col.f64";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as rtc_types::ty::IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3, a4, a5, a6, a7] = a.elements();
        let [b0, b1, b2, b3] = b.elements();
        let [c0, c1, c2, c3] = c.elements();
        (
            a0, a1, a2, a3, a4, a5, a6, a7, b0, b1, b2, b3, c0, c1, c2, c3,
        )
    }
}
