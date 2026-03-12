use maize_core::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    struct_reflect,
    ty::{
        Addrspace, F16, F32, F64, IntoFuncArgs, M, R, SizedTy, V,
        cuda::{Global, Shared},
    },
    val::Val,
};

use crate::{Tile, WarpCollectiveTileTy, group::warp::Warp};

pub mod sm75;
pub mod sm80;
pub mod sm80_derived;
pub mod sm89;
pub mod sm90;

pub trait IntrinsicSyncMMAOp {
    type AFrag: SizedTy + Copy;
    type BFrag: SizedTy + Copy;
    type CFrag: SizedTy + Copy;

    type Args: IntoFuncArgs;
    type Ret: SizedTy;

    const INTRINSIC_NAME: &str;

    fn unpack_args<'a>(ret: Val<'a, Self::Ret>) -> Val<'a, Self::CFrag>;
    fn pack_args<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> <Self::Args as IntoFuncArgs>::ArgValues<'a>;
}

pub trait SyncMMAOp {
    type FragA: SizedTy + Copy;
    type FragB: SizedTy + Copy;
    type FragC: SizedTy + Copy;

    fn call<'a>(
        a: Val<'a, Self::FragA>,
        b: Val<'a, Self::FragB>,
        c: Val<'a, Self::FragC>,
    ) -> Val<'a, Self::FragC>;

    fn zero_accum<'a>(cx: &'a FnCodegen) -> Val<'a, Self::FragC> {
        Val::zeros(cx)
    }

    fn load_a<'a, TileA>(ptr: Val<'a, R<&Tile<TileA>, Shared>>) -> Val<'a, Self::FragA>
    where
        TileA: WarpCollectiveTileTy<FragT = Self::FragA>,
    {
        let lane = Warp::new(ptr.cx()).lane();
        TileA::collective_load(&ptr, lane)
    }
    fn load_b<'a, TileB>(ptr: Val<'a, R<&Tile<TileB>, Shared>>) -> Val<'a, Self::FragB>
    where
        TileB: WarpCollectiveTileTy<FragT = Self::FragB>,
    {
        let lane = Warp::new(ptr.cx()).lane();
        TileB::collective_load(&ptr, lane)
    }
    fn store_c<'a, TileC, Space: Addrspace>(
        ptr: Val<'a, M<&mut Tile<TileC>, Space>>,
        val: Val<'a, Self::FragC>,
    ) where
        TileC: WarpCollectiveTileTy<FragT = Self::FragC>,
    {
        let lane = Warp::new(ptr.cx()).lane();
        TileC::collective_store(ptr, val, lane);
    }
}

pub trait SmemSyncMMAOp: SyncMMAOp {
    const M: u32;
    const N: u32;
    const K: u32;
}

impl<Intrins: IntrinsicSyncMMAOp> SyncMMAOp for Intrins {
    type FragA = Intrins::AFrag;
    type FragB = Intrins::BFrag;
    type FragC = Intrins::CFrag;
    fn call<'a>(
        a: Val<'a, Intrins::AFrag>,
        b: Val<'a, Intrins::BFrag>,
        c: Val<'a, Intrins::CFrag>,
    ) -> Val<'a, Intrins::CFrag> {
        let cx = a.cx();
        let args = Self::pack_args(a, b, c);
        let mma_intrinsic =
            cx.get_intrinsic::<Intrins::Ret, Intrins::Args>(Self::INTRINSIC_NAME, true);
        let raw_ret = cx.call_fn(mma_intrinsic, args);
        Self::unpack_args(raw_ret)
    }
}

pub fn run_test_sync_mma<MMA: IntrinsicSyncMMAOp>(sm: SM) -> String {
    let kernel = new_ptx_kernel::<(
        R<&MMA::AFrag, Global>,
        R<&MMA::BFrag, Global>,
        M<&mut MMA::CFrag, Global>,
    )>();
    let (a, b, mut d) = kernel.get_args();
    let c = MMA::zero_accum(a.cx());
    let ret = MMA::call(a.load(), b.load(), c);
    d.store(ret);
    let asm = kernel.finalize().compile_asm_quickly(sm);
    assert!(!asm.contains("call"), "{}", asm);
    asm
}

struct_reflect!(
    pub struct WarpRetF16_16x8 {
        pub(super) d01: V<F16, 2>,
        pub(super) d23: V<F16, 2>,
    } => warp_ret_f16_16x8
);
struct_reflect!(
    pub struct WarpRetF32_16x8 {
        pub(super) d0: F32,
        pub(super) d1: F32,
        pub(super) d2: F32,
        pub(super) d3: F32,
    } => warp_ret_f32_16x8
);
struct_reflect!(pub struct WarpRetF64_8x8 {
    pub(super) d0: F64,
    pub(super) d1: F64,
} => warp_ret_f64_8x8);

struct_reflect!(pub struct WarpRetF64_16x8 {
    pub(super) d0: F64,
    pub(super) d1: F64,
    pub(super) d2: F64,
    pub(super) d3: F64,
} => warp_ret_f64_16x8);

impl WarpRetF32_16x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F32, 4>> {
        let a = val.into_accessor();
        Val::from_elements([a.d0, a.d1, a.d2, a.d3])
    }
}
impl WarpRetF16_16x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F16, 4>> {
        let a = val.into_accessor();
        let [d0, d1] = a.d01.elements();
        let [d2, d3] = a.d23.elements();
        Val::from_elements([d0, d1, d2, d3])
    }
}

impl WarpRetF64_8x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F64, 2>> {
        let a = val.into_accessor();
        Val::from_elements([a.d0, a.d1])
    }
}

impl WarpRetF64_16x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F64, 4>> {
        let a = val.into_accessor();
        Val::from_elements([a.d0, a.d1, a.d2, a.d3])
    }
}
