use rtc_types::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    struct_reflect,
    ty::{F16, F32, F64, IntoFuncArgs, M, R, SizedTy, V, cuda::Global},
    val::Val,
};

pub mod sm75;
pub mod sm80;
pub mod sm89;
pub mod sm90;

pub trait SyncMMAOp {
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
    fn call<'a>(
        a: Val<'a, Self::AFrag>,
        b: Val<'a, Self::BFrag>,
        c: Val<'a, Self::CFrag>,
    ) -> Val<'a, Self::CFrag> {
        let cx = a.cx();
        let args = Self::pack_args(a, b, c);
        let mma_intrinsic = cx.get_intrinsic::<Self::Ret, Self::Args>(Self::INTRINSIC_NAME, true);
        let raw_ret = cx.call_fn(mma_intrinsic, args);
        Self::unpack_args(raw_ret)
    }
    fn zero_accum<'a>(cx: &'a FnCodegen) -> Val<'a, Self::CFrag> {
        Val::zeros(cx)
    }
}

pub fn run_test_sync_mma<MMA: SyncMMAOp>(sm: SM) -> String {
    let kernel = new_ptx_kernel::<(
        Global<R<&MMA::AFrag>>,
        Global<R<&MMA::BFrag>>,
        Global<M<&mut MMA::CFrag>>,
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
        let a = val.as_ref().accessor();
        Val::from_elements([a.d0.load(), a.d1.load(), a.d2.load(), a.d3.load()])
    }
}
impl WarpRetF16_16x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F16, 4>> {
        let a = val.as_ref().accessor();
        let [d0, d1] = a.d01.load().elements();
        let [d2, d3] = a.d23.load().elements();
        Val::from_elements([d0, d1, d2, d3])
    }
}

impl WarpRetF64_8x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F64, 2>> {
        let a = val.as_ref().accessor();
        Val::from_elements([a.d0.load(), a.d1.load()])
    }
}

impl WarpRetF64_16x8 {
    fn unpack(val: Val<'_, Self>) -> Val<'_, V<F64, 4>> {
        let a = val.as_ref().accessor();
        Val::from_elements([a.d0.load(), a.d1.load(), a.d2.load(), a.d3.load()])
    }
}
