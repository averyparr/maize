use rtc_types::{
    struct_reflect,
    ty::{F16, F32, IntoFuncArgs, V},
    val::Val,
};

use crate::mma::{SyncMMAOp, WarpRetF16_16x8, WarpRetF32_16x8};

pub struct Sm75MmaF16F16_16x8x8;
pub struct Sm75MmaF16F32_16x8x8;

struct_reflect!(
    pub struct RetSm75MmaF16F16_16x8x8 {
        pub(super) d01: V<F16, 2>,
        pub(super) d23: V<F16, 2>,
    } => sm75_f16_f16_16x8x8
);
struct_reflect!(
    pub struct RetSm75MmaF16F32_16x8x8 {
        pub(super) d0: F32,
        pub(super) d1: F32,
        pub(super) d2: F32,
        pub(super) d3: F32,
    } => sm75_f16_f32_16x8x8
);

impl SyncMMAOp for Sm75MmaF16F16_16x8x8 {
    type AFrag = V<F16, 4>;
    type BFrag = V<F16, 2>;
    type CFrag = V<F16, 4>;

    type Args = (V<F16, 2>, V<F16, 2>, V<F16, 2>, V<F16, 2>, V<F16, 2>);

    type Ret = WarpRetF16_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k8.row.col.f16.f16";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3] = a.elements();
        let a01 = Val::from_elements([a0, a1]);
        let a23 = Val::from_elements([a2, a3]);
        let [c0, c1, c2, c3] = c.elements();
        let c01 = Val::from_elements([c0, c1]);
        let c23 = Val::from_elements([c2, c3]);
        (a01, a23, b, c01, c23)
    }
}

impl SyncMMAOp for Sm75MmaF16F32_16x8x8 {
    type AFrag = V<F16, 4>;
    type BFrag = V<F16, 2>;
    type CFrag = V<F32, 4>;

    type Args = (V<F16, 2>, V<F16, 2>, V<F16, 2>, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k8.row.col.f32.f32";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3] = a.elements();
        let a01 = Val::from_elements([a0, a1]);
        let a23 = Val::from_elements([a2, a3]);
        let [c0, c1, c2, c3] = c.elements();
        (a01, a23, b, c0, c1, c2, c3)
    }
}
