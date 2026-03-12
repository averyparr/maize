use std::marker::PhantomData;

use maize_core::{
    ty::{E4M3, E5M2, F16, F32, I32, IntoFuncArgs, V, vec::VectorizableTy},
    val::Val,
};

use crate::mma::{IntrinsicSyncMMAOp, WarpRetF16_16x8, WarpRetF32_16x8};

#[deprecated(
    note = "e4m3×e4m3→f32 m16n8k16 MMA not supported: LLVM NVPTX backend lacks instruction selection for this intrinsic"
)]
pub struct Sm89MmaF8F32_16x8x16<AType, BType>(PhantomData<(AType, BType)>);
pub struct Sm89MmaF8F32_16x8x32<AType, BType>(PhantomData<(AType, BType)>);
#[deprecated(
    note = "e4m3×e4m3→f32 m16n8k16 MMA not supported: LLVM NVPTX backend lacks instruction selection for this intrinsic"
)]
pub struct Sm89MmaF8F16_16x8x16<AType, BType>(PhantomData<(AType, BType)>);
pub struct Sm89MmaF8F16_16x8x32<AType, BType>(PhantomData<(AType, BType)>);

trait F8Matmul {
    const INTRINS_16_F32: &str;
    const INTRINS_32_F32: &str;
    const INTRINS_16_F16: &str;
    const INTRINS_32_F16: &str;
}

impl F8Matmul for (E4M3, E4M3) {
    const INTRINS_16_F32: &str = "llvm.nvvm.mma.m16n8k16.row.col.f32.e4m3.e4m3.f32";
    const INTRINS_32_F32: &str = "llvm.nvvm.mma.m16n8k32.row.col.f32.e4m3.e4m3.f32";
    const INTRINS_16_F16: &str = "llvm.nvvm.mma.m16n8k16.row.col.f16.e4m3.e4m3.f16";
    const INTRINS_32_F16: &str = "llvm.nvvm.mma.m16n8k32.row.col.f16.e4m3.e4m3.f16";
}
impl F8Matmul for (E5M2, E4M3) {
    const INTRINS_16_F32: &str = "llvm.nvvm.mma.m16n8k16.row.col.f32.e5m2.e4m3.f32";
    const INTRINS_32_F32: &str = "llvm.nvvm.mma.m16n8k32.row.col.f32.e5m2.e4m3.f32";
    const INTRINS_16_F16: &str = "llvm.nvvm.mma.m16n8k16.row.col.f16.e5m2.e4m3.f16";
    const INTRINS_32_F16: &str = "llvm.nvvm.mma.m16n8k32.row.col.f16.e5m2.e4m3.f16";
}
impl F8Matmul for (E4M3, E5M2) {
    const INTRINS_16_F32: &str = "llvm.nvvm.mma.m16n8k16.row.col.f32.e4m3.e5m2.f32";
    const INTRINS_32_F32: &str = "llvm.nvvm.mma.m16n8k32.row.col.f32.e4m3.e5m2.f32";
    const INTRINS_16_F16: &str = "llvm.nvvm.mma.m16n8k16.row.col.f16.e4m3.e5m2.f16";
    const INTRINS_32_F16: &str = "llvm.nvvm.mma.m16n8k32.row.col.f16.e4m3.e5m2.f16";
}
impl F8Matmul for (E5M2, E5M2) {
    const INTRINS_16_F32: &str = "llvm.nvvm.mma.m16n8k16.row.col.f32.e5m2.e5m2.f32";
    const INTRINS_32_F32: &str = "llvm.nvvm.mma.m16n8k32.row.col.f32.e5m2.e5m2.f32";
    const INTRINS_16_F16: &str = "llvm.nvvm.mma.m16n8k16.row.col.f16.e5m2.e5m2.f16";
    const INTRINS_32_F16: &str = "llvm.nvvm.mma.m16n8k32.row.col.f16.e5m2.e5m2.f16";
}

#[expect(
    deprecated,
    reason = "This is necessary to make them work in the future"
)]
impl<F8A, F8B> IntrinsicSyncMMAOp for Sm89MmaF8F32_16x8x16<F8A, F8B>
where
    (F8A, F8B): F8Matmul,
    F8A: VectorizableTy + Copy,
    F8B: VectorizableTy + Copy,
{
    type AFrag = V<F8A, 8>;
    type BFrag = V<F8B, 4>;
    type CFrag = V<F32, 4>;

    type Args = (I32, I32, I32, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = <(F8A, F8B)>::INTRINS_16_F32;

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1] = unsafe { a.bitcast() }.elements();
        let [b0] = unsafe { b.bitcast() }.elements();
        let [c0, c1, c2, c3] = c.elements();
        (a0, a1, b0, c0, c1, c2, c3)
    }
}

impl<F8A, F8B> IntrinsicSyncMMAOp for Sm89MmaF8F32_16x8x32<F8A, F8B>
where
    (F8A, F8B): F8Matmul,
    F8A: VectorizableTy + Copy,
    F8B: VectorizableTy + Copy,
{
    type AFrag = V<F8A, 16>;
    type BFrag = V<F8B, 8>;
    type CFrag = V<F32, 4>;

    type Args = (I32, I32, I32, I32, I32, I32, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = <(F8A, F8B)>::INTRINS_32_F32;

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3] = unsafe { a.bitcast() }.elements();
        let [b0, b1] = unsafe { b.bitcast() }.elements();
        let [c0, c1, c2, c3] = c.elements();
        (a0, a1, a2, a3, b0, b1, c0, c1, c2, c3)
    }
}

#[expect(
    deprecated,
    reason = "This is necessary to make them work in the future"
)]
impl<F8A, F8B> IntrinsicSyncMMAOp for Sm89MmaF8F16_16x8x16<F8A, F8B>
where
    (F8A, F8B): F8Matmul,
    F8A: VectorizableTy + Copy,
    F8B: VectorizableTy + Copy,
{
    type AFrag = V<F8A, 8>;
    type BFrag = V<F8B, 4>;
    type CFrag = V<F16, 4>;

    type Args = (I32, I32, I32, V<F16, 2>, V<F16, 2>);
    type Ret = WarpRetF16_16x8;

    const INTRINSIC_NAME: &str = <(F8A, F8B)>::INTRINS_16_F16;

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1] = unsafe { a.bitcast() }.elements();
        let [b0] = unsafe { b.bitcast() }.elements();
        let [c0, c1, c2, c3] = c.elements();
        (
            a0,
            a1,
            b0,
            Val::from_elements([c0, c1]),
            Val::from_elements([c2, c3]),
        )
    }
}

impl<F8A, F8B> IntrinsicSyncMMAOp for Sm89MmaF8F16_16x8x32<F8A, F8B>
where
    (F8A, F8B): F8Matmul,
    F8A: VectorizableTy + Copy,
    F8B: VectorizableTy + Copy,
{
    type AFrag = V<F8A, 16>;
    type BFrag = V<F8B, 8>;
    type CFrag = V<F16, 4>;

    type Args = (I32, I32, I32, I32, I32, I32, V<F16, 2>, V<F16, 2>);
    type Ret = WarpRetF16_16x8;

    const INTRINSIC_NAME: &str = <(F8A, F8B)>::INTRINS_32_F16;

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3] = unsafe { a.bitcast() }.elements();
        let [b0, b1] = unsafe { b.bitcast() }.elements();
        let [c0, c1, c2, c3] = c.elements();
        (
            a0,
            a1,
            a2,
            a3,
            b0,
            b1,
            Val::from_elements([c0, c1]),
            Val::from_elements([c2, c3]),
        )
    }
}
