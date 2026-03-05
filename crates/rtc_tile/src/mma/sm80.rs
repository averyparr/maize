use rtc_types::{
    ty::{BF16, F16, F32, F64, I32, IntoFuncArgs, V},
    val::Val,
};

use crate::mma::{SyncMMAOp, WarpRetF16_16x8, WarpRetF32_16x8, WarpRetF64_8x8};

pub struct Sm80MmaF16F16_16x8x16;
pub struct Sm80MmaF16F32_16x8x16;
pub struct Sm80MmaBf16F32_16x8x8;
pub struct Sm80MmaBf16F32_16x8x16;
pub struct Sm80MmaTf32F32_16x8x4;
pub struct Sm80MmaTf32F32_16x8x8;
pub struct SM80MmaF64F64_8x8x4;

impl SyncMMAOp for Sm80MmaF16F16_16x8x16 {
    type AFrag = V<F16, 8>;
    type BFrag = V<F16, 4>;
    type CFrag = V<F16, 4>;

    type Args = (
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
    );

    type Ret = WarpRetF16_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k16.row.col.f16.f16";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3, a4, a5, a6, a7] = a.elements();
        let [b0, b1, b2, b3] = b.elements();
        let [c0, c1, c2, c3] = c.elements();
        (
            Val::from_elements([a0, a1]),
            Val::from_elements([a2, a3]),
            Val::from_elements([a4, a5]),
            Val::from_elements([a6, a7]),
            Val::from_elements([b0, b1]),
            Val::from_elements([b2, b3]),
            Val::from_elements([c0, c1]),
            Val::from_elements([c2, c3]),
        )
    }
}

impl SyncMMAOp for Sm80MmaF16F32_16x8x16 {
    type AFrag = V<F16, 8>;
    type BFrag = V<F16, 4>;
    type CFrag = V<F32, 4>;

    type Args = (
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        V<F16, 2>,
        F32,
        F32,
        F32,
        F32,
    );
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k16.row.col.f32.f32";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        let a = ret.into_accessor();
        Val::from_elements([a.d0, a.d1, a.d2, a.d3])
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0, a1, a2, a3, a4, a5, a6, a7] = a.elements();
        let [b0, b1, b2, b3] = b.elements();
        let [c0, c1, c2, c3] = c.elements();
        (
            Val::from_elements([a0, a1]),
            Val::from_elements([a2, a3]),
            Val::from_elements([a4, a5]),
            Val::from_elements([a6, a7]),
            Val::from_elements([b0, b1]),
            Val::from_elements([b2, b3]),
            c0,
            c1,
            c2,
            c3,
        )
    }
}

impl SyncMMAOp for Sm80MmaBf16F32_16x8x8 {
    type AFrag = V<BF16, 4>;
    type BFrag = V<BF16, 2>;
    type CFrag = V<F32, 4>;

    type Args = (I32, I32, I32, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k8.row.col.bf16";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a01, a23] = unsafe { a.bitcast() }.elements();
        let [b01] = unsafe { b.bitcast() }.elements();
        let [c0, c1, c2, c3] = c.elements();
        (a01, a23, b01, c0, c1, c2, c3)
    }
}

impl SyncMMAOp for Sm80MmaBf16F32_16x8x16 {
    type AFrag = V<BF16, 8>;
    type BFrag = V<BF16, 4>;
    type CFrag = V<F32, 4>;

    type Args = (I32, I32, I32, I32, I32, I32, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k16.row.col.bf16";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a01, a23, a45, a67] = unsafe { a.bitcast() }.elements();
        let [b01, b23] = unsafe { b.bitcast() }.elements();
        let [c0, c1, c2, c3] = c.elements();
        (a01, a23, a45, a67, b01, b23, c0, c1, c2, c3)
    }
}

impl SyncMMAOp for Sm80MmaTf32F32_16x8x4 {
    type AFrag = V<F32, 2>;
    type BFrag = V<F32, 1>;
    type CFrag = V<F32, 4>;

    type Args = (I32, I32, I32, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k4.row.col.tf32";

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

impl SyncMMAOp for Sm80MmaTf32F32_16x8x8 {
    type AFrag = V<F32, 4>;
    type BFrag = V<F32, 2>;
    type CFrag = V<F32, 4>;

    type Args = (I32, I32, I32, I32, I32, I32, F32, F32, F32, F32);
    type Ret = WarpRetF32_16x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m16n8k8.row.col.tf32";

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

impl SyncMMAOp for SM80MmaF64F64_8x8x4 {
    type AFrag = V<F64, 1>;
    type BFrag = V<F64, 1>;
    type CFrag = V<F64, 2>;

    type Args = (F64, F64, F64, F64);
    type Ret = WarpRetF64_8x8;

    const INTRINSIC_NAME: &str = "llvm.nvvm.mma.m8n8k4.row.col.f64";

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag> {
        Self::Ret::unpack(ret)
    }

    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a> {
        let [a0] = a.elements();
        let [b0] = b.elements();
        let [c0, c1] = c.elements();
        (a0, b0, c0, c1)
    }
}
