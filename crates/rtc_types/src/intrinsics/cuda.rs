use inkwell::{AddressSpace, values::BasicValue};

use crate::{
    codegen::{FnCodegen, Func, new_ptx_kernel, target::cuda::SM},
    intrinsics::{BinaryIntrinsic, IntrinsicsLibrary, UnaryIntrinsic},
    ty::{AddrspacePtr, M, R, SizedTy, cuda::Shared, raw::*},
    val::Val,
};

pub struct CUDA<'a>(pub(crate) &'a FnCodegen);

impl<'a> CUDA<'a> {
    #[expect(unused)]
    fn call_unary_intrinsic<Intrinsic, T: UnaryIntrinsic<Intrinsic>>(
        _: Intrinsic,
        val: Val<'_, T>,
    ) -> Val<'_, T> {
        T::call_intrinsic(val, true)
    }
    #[expect(unused)]
    fn call_binary_intrinsic<Intrinsic, T: BinaryIntrinsic<Intrinsic>>(
        _: Intrinsic,
        lhs: Val<'a, T>,
        rhs: Val<'a, T>,
    ) -> Val<'a, T> {
        T::call_intrinsic(lhs, rhs, true)
    }

    pub fn alloc_aligned_shared<T: SizedTy>(self, align: u32) -> Val<'a, Shared<M<&'a mut T>>> {
        assert!(
            align % T::ALIGN == 0,
            "must be able to align properly to {}",
            std::any::type_name::<T>()
        );
        let ty = T::ty(self.0.ctx());
        let global_val = self.0.module().add_global(
            ty,
            Some(AddressSpace::from(Shared::<M<&mut T>>::ADDRSPACE)),
            "const_sized_shared",
        );
        global_val.set_initializer(&T::undef(self.0.ctx()));
        global_val.set_alignment(T::ALIGN);
        unsafe { Val::new_from_value(self.0, global_val.as_basic_value_enum()) }
    }

    pub fn alloc_shared<T: SizedTy>(self) -> Val<'a, Shared<M<&'a mut T>>> {
        let ty = T::ty(self.0.ctx());
        let global_val = self.0.module().add_global(
            ty,
            Some(AddressSpace::from(Shared::<M<&mut T>>::ADDRSPACE)),
            "const_sized_shared",
        );
        global_val.set_initializer(&T::undef(self.0.ctx()));
        global_val.set_alignment(T::ALIGN);
        unsafe { Val::new_from_value(self.0, global_val.as_basic_value_enum()) }
    }
}

impl<'a> IntrinsicsLibrary for CUDA<'a> {}

macro_rules! impl_unary {
    (
        $intrinsic_name: ident,
        $intrinsic_fn_name: ident
        $(, $tipe: ty=$intrinsic: literal)*
        $(,)?
    ) => {
        pub struct $intrinsic_name;
        $(
            unsafe impl UnaryIntrinsic<$intrinsic_name> for $tipe {
                const INTRINSIC_NAME: &str = $intrinsic;
            }
        )*

        #[cfg(test)]
        mod $intrinsic_fn_name {
            use super::*;
            #[test]
            fn test_compiles() {
                $(
                    let ptx = test_unary_intrinsic::<$tipe, $intrinsic_name>();
                    assert!(! ptx.contains(".extern .func"), "PTX compiles down to\n{}", ptx);
                )*
            }
        }

        impl<T: UnaryIntrinsic<$intrinsic_name>> Val<'_, T>
        {
            pub fn $intrinsic_fn_name(self) -> Self {
                T::call_intrinsic(self, true)
            }
        }
    };
}

macro_rules! impl_binary {
    (
        $intrinsic_name: ident,
        $intrinsic_fn_name: ident
        $(, $tipe: ty=$intrinsic: literal)*
        $(,)?
    ) => {
        pub struct $intrinsic_name;
        $(
            unsafe impl BinaryIntrinsic<$intrinsic_name> for $tipe {
                const INTRINSIC_NAME: &str = $intrinsic;
            }
        )*

        #[cfg(test)]
        mod $intrinsic_fn_name {
            use super::*;
            #[test]
            fn test_compiles() {
                $(
                    let ptx = test_binary_intrinsic::<$tipe, $intrinsic_name>();
                    assert!(! ptx.contains(".extern .func"), "PTX compiles down to\n{}", ptx);
                )*
            }
        }

        impl<T: BinaryIntrinsic<$intrinsic_name>> Val<'_, T>
        {
            pub fn $intrinsic_fn_name(self, rhs: Self) -> Self {
                T::call_intrinsic(self, rhs, true)
            }
        }
    };
}

#[allow(unused)]
fn test_unary_intrinsic<T: UnaryIntrinsic<Intrins> + SizedTy + Copy, Intrins>() -> String {
    let func = new_ptx_kernel::<(M<&mut T>,)>();
    let (mut val,) = func.get_args();
    let out = T::call_intrinsic(val.load(), true);
    val.store(out);
    let ptx = String::from_utf8(func.finalize().compile_asm_quickly(SM::SM90).into()).unwrap();

    return ptx;
}

#[allow(unused)]
fn test_binary_intrinsic<T: BinaryIntrinsic<Intrins> + SizedTy + Copy, Intrins>() -> String {
    let func = new_ptx_kernel::<(R<&T>, R<&T>, M<&mut T>)>();
    let (a, b, mut c) = func.get_args();
    let out = T::call_intrinsic(a.load(), b.load(), true);
    c.store(out);
    let ptx = String::from_utf8(func.finalize().compile_asm_quickly(SM::SM90).into()).unwrap();
    ptx
}

// Unary Intrinsics
impl_unary!(
    Abs,
    __intrinsic_abs,
    crate::ty::F16 = "llvm.nvvm.fabs.f16",
    // F16x2 = "llvm.nvvm.fabs.f16x2",
    BF16 = "llvm.nvvm.fabs.bf16",
    // BF16x2 = "llvm.nvvm.fabs.bf16x2",
    F32 = "llvm.nvvm.fabs.f32",
    F64 = "llvm.nvvm.fabs.f64",
);
impl_unary!(
    AbsFtz,
    __intrinsic_abs_ftz,
    F16 = "llvm.nvvm.fabs.ftz.f16",
    // F16x2 = "llvm.nvvm.fabs.ftz.f16x2",
    F32 = "llvm.nvvm.fabs.ftz.f32",
);

impl_unary!(
    Exp2Approx,
    __intrinsic_ex2_approx,
    F32 = "llvm.nvvm.ex2.approx.f",
    F16 = "llvm.nvvm.ex2.approx.f16",
    // F16x2 = "llvm.nvvm.ex2.approx.f16x2",
);
impl_unary!(
    Exp2ApproxFtz,
    __intrinsic_ex2_approx_ftz,
    F32 = "llvm.nvvm.ex2.approx.ftz.f",
    // For some reason, these are not supported
    // by the LLVM backend, despite being valid PTX
    // + seemingly defined in the NVPTX intrinsics
    // BF16 = "llvm.nvvm.ex2.approx.ftz.bf16",
    // BF16x2 = "llvm.nvvm.ex2.approx.ftz.bf16x2",
);

impl_unary!(
    Log2Approx,
    __intrinsic_log2_approx,
    F32 = "llvm.nvvm.lg2.approx.f",
    F64 = "llvm.nvvm.lg2.approx.d",
);
impl_unary!(
    Log2ApproxFtz,
    __intrinsic_log2_approx_ftz,
    F32 = "llvm.nvvm.lg2.approx.ftz.f",
);

impl_unary!(
    Floor,
    __intrinsic_floor,
    F32 = "llvm.nvvm.floor.f",
    F64 = "llvm.nvvm.floor.d",
);
impl_unary!(
    FloorFtz,
    __intrinsic_floor_ftz,
    F32 = "llvm.nvvm.floor.ftz.f",
);
impl_unary!(
    Ceil,
    __intrinsic_ceil,
    F32 = "llvm.nvvm.ceil.f",
    F64 = "llvm.nvvm.ceil.d",
);
impl_unary!(CeilFtz, __intrinsic_ceil_ftz, F32 = "llvm.nvvm.ceil.ftz.f",);

impl_unary!(
    RsqrtApprox,
    __intrinsic_rsqrt_approx,
    F32 = "llvm.nvvm.rsqrt.approx.f",
    F64 = "llvm.nvvm.rsqrt.approx.d",
);
impl_unary!(
    RsqrtApproxFtz,
    __intrinsic_rsqrt_approx_ftz,
    F32 = "llvm.nvvm.rsqrt.approx.ftz.f",
    F64 = "llvm.nvvm.rsqrt.approx.ftz.d",
);

impl_unary!(
    SqrtRn,
    __intrinsic_sqrt_rn,
    F32 = "llvm.nvvm.sqrt.rn.f",
    F64 = "llvm.nvvm.sqrt.rn.d",
);
impl_unary!(
    SqrtRz,
    __intrinsic_sqrt_rz,
    F32 = "llvm.nvvm.sqrt.rz.f",
    F64 = "llvm.nvvm.sqrt.rz.d",
);
impl_unary!(
    SqrtRm,
    __intrinsic_sqrt_rm,
    F32 = "llvm.nvvm.sqrt.rm.f",
    F64 = "llvm.nvvm.sqrt.rm.d",
);
impl_unary!(
    SqrtRp,
    __intrinsic_sqrt_rp,
    F32 = "llvm.nvvm.sqrt.rp.f",
    F64 = "llvm.nvvm.sqrt.rp.d",
);
impl_unary!(
    SqrtRnFtz,
    __intrinsic_sqrt_rn_ftz,
    F32 = "llvm.nvvm.sqrt.rn.ftz.f",
);
impl_unary!(
    SqrtRzFtz,
    __intrinsic_sqrt_rz_ftz,
    F32 = "llvm.nvvm.sqrt.rz.ftz.f",
);
impl_unary!(
    SqrtRmFtz,
    __intrinsic_sqrt_rm_ftz,
    F32 = "llvm.nvvm.sqrt.rm.ftz.f",
);
impl_unary!(
    SqrtRpFtz,
    __intrinsic_sqrt_rp_ftz,
    F32 = "llvm.nvvm.sqrt.rp.ftz.f",
);
impl_unary!(
    SqrtApprox,
    __intrinsic_sqrt_approx,
    F32 = "llvm.nvvm.sqrt.approx.f",
);
impl_unary!(
    SqrtApproxFtz,
    __intrinsic_sqrt_approx_ftz,
    F32 = "llvm.nvvm.sqrt.approx.ftz.f",
);

impl_unary!(
    RcpRn,
    __intrinsic_rcp_rn,
    F32 = "llvm.nvvm.rcp.rn.f",
    F64 = "llvm.nvvm.rcp.rn.d",
);
impl_unary!(
    RcpRz,
    __intrinsic_rcp_rz,
    F32 = "llvm.nvvm.rcp.rz.f",
    F64 = "llvm.nvvm.rcp.rz.d",
);
impl_unary!(
    RcpRm,
    __intrinsic_rcp_rm,
    F32 = "llvm.nvvm.rcp.rm.f",
    F64 = "llvm.nvvm.rcp.rm.d",
);
impl_unary!(
    RcpRp,
    __intrinsic_rcp_rp,
    F32 = "llvm.nvvm.rcp.rp.f",
    F64 = "llvm.nvvm.rcp.rp.d",
);

impl_unary!(
    RcpRnFtz,
    __intrinsic_rcp_rn_ftz,
    F32 = "llvm.nvvm.rcp.rn.ftz.f",
);
impl_unary!(
    RcpRzFtz,
    __intrinsic_rcp_rz_ftz,
    F32 = "llvm.nvvm.rcp.rz.ftz.f",
);
impl_unary!(
    RcpRmFtz,
    __intrinsic_rcp_rm_ftz,
    F32 = "llvm.nvvm.rcp.rm.ftz.f",
);
impl_unary!(
    RcpRpFtz,
    __intrinsic_rcp_rp_ftz,
    F32 = "llvm.nvvm.rcp.rp.ftz.f",
);
impl_unary!(
    RcpApproxFtz,
    __intrinsic_rcp_approx_ftz,
    F32 = "llvm.nvvm.rcp.approx.ftz.f",
    F64 = "llvm.nvvm.rcp.approx.ftz.d",
);

impl_unary!(
    SinApprox,
    __intrinsic_sin_approx,
    F32 = "llvm.nvvm.sin.approx.f",
);
impl_unary!(
    SinApproxFtz,
    __intrinsic_sin_approx_ftz,
    F32 = "llvm.nvvm.sin.approx.ftz.f",
);
impl_unary!(
    CosApprox,
    __intrinsic_cos_approx,
    F32 = "llvm.nvvm.cos.approx.f",
);
impl_unary!(
    CosApproxFtz,
    __intrinsic_cos_approx_ftz,
    F32 = "llvm.nvvm.cos.approx.ftz.f",
);

// Binary Intrinsics
impl_binary!(
    Min,
    __intrinsic_min,
    F16 = "llvm.nvvm.fmin.f16",
    // F16x2 = "llvm.nvvm.fmin.f16x2",
    BF16 = "llvm.nvvm.fmin.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmin.bf16x2",
    F32 = "llvm.nvvm.fmin.f",
    F64 = "llvm.nvvm.fmin.d",
);
impl_binary!(
    Max,
    __intrinsic_max,
    F16 = "llvm.nvvm.fmax.f16",
    // F16x2 = "llvm.nvvm.fmax.f16x2",
    BF16 = "llvm.nvvm.fmax.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmax.bf16x2",
    F32 = "llvm.nvvm.fmax.f",
    F64 = "llvm.nvvm.fmax.d",
);
impl_binary!(
    MinFtz,
    __intrinsic_min_ftz,
    F16 = "llvm.nvvm.fmin.ftz.f16",
    // F16x2 = "llvm.nvvm.fmin.ftz.f16x2",
    F32 = "llvm.nvvm.fmin.ftz.f",
);
impl_binary!(
    MaxFtz,
    __intrinsic_max_ftz,
    F16 = "llvm.nvvm.fmax.ftz.f16",
    // F16x2 = "llvm.nvvm.fmax.ftz.f16x2",
    F32 = "llvm.nvvm.fmax.ftz.f",
);
impl_binary!(
    MinNan,
    __intrinsic_min_nan,
    F16 = "llvm.nvvm.fmin.nan.f16",
    // F16x2 = "llvm.nvvm.fmin.nan.f16x2",
    BF16 = "llvm.nvvm.fmin.nan.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmin.nan.bf16x2",
    F32 = "llvm.nvvm.fmin.nan.f",
);
impl_binary!(
    MaxNan,
    __intrinsic_max_nan,
    F16 = "llvm.nvvm.fmax.nan.f16",
    // F16x2 = "llvm.nvvm.fmax.nan.f16x2",
    BF16 = "llvm.nvvm.fmax.nan.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmax.nan.bf16x2",
    F32 = "llvm.nvvm.fmax.nan.f",
);
impl_binary!(
    MaxNanFtz,
    __intrinsic_max_ftz_nan,
    F16 = "llvm.nvvm.fmax.ftz.nan.f16",
    // F16x2 = "llvm.nvvm.fmax.ftz.nan.f16x2",
    F32 = "llvm.nvvm.fmax.ftz.nan.f",
);

impl_binary!(
    MinXorsignAbs,
    __intrinsic_min_xorsign_abs,
    F16 = "llvm.nvvm.fmin.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmin.xorsign.abs.f16x2",
    BF16 = "llvm.nvvm.fmin.xorsign.abs.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmin.xorsign.abs.bf16x2",
    F32 = "llvm.nvvm.fmin.xorsign.abs.f",
);
impl_binary!(
    MaxXorsignAbs,
    __intrinsic_max_xorsign_abs,
    F16 = "llvm.nvvm.fmax.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmax.xorsign.abs.f16x2",
    BF16 = "llvm.nvvm.fmax.xorsign.abs.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmax.xorsign.abs.bf16x2",
    F32 = "llvm.nvvm.fmax.xorsign.abs.f",
);
impl_binary!(
    MinXorsignAbsFtz,
    __intrinsic_min_ftz_xorsign_abs,
    F16 = "llvm.nvvm.fmin.ftz.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmin.ftz.xorsign.abs.f16x2",
    F32 = "llvm.nvvm.fmin.ftz.xorsign.abs.f",
);
impl_binary!(
    MaxXorsignAbsFtz,
    __intrinsic_max_ftz_xorsign_abs,
    F16 = "llvm.nvvm.fmax.ftz.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmax.ftz.xorsign.abs.f16x2",
    F32 = "llvm.nvvm.fmax.ftz.xorsign.abs.f",
);
impl_binary!(
    MinXorsignAbsNan,
    __intrinsic_min_nan_xorsign_abs,
    F16 = "llvm.nvvm.fmin.nan.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmin.nan.xorsign.abs.f16x2",
    BF16 = "llvm.nvvm.fmin.nan.xorsign.abs.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmin.nan.xorsign.abs.bf16x2",
    F32 = "llvm.nvvm.fmin.nan.xorsign.abs.f",
);
impl_binary!(
    MaxXorsignAbsNan,
    __intrinsic_max_nan_xorsign_abs,
    F16 = "llvm.nvvm.fmax.nan.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmax.nan.xorsign.abs.f16x2",
    BF16 = "llvm.nvvm.fmax.nan.xorsign.abs.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmax.nan.xorsign.abs.bf16x2",
    F32 = "llvm.nvvm.fmax.nan.xorsign.abs.f",
);
impl_binary!(
    MinXorsignAbsFtzNan,
    __intrinsic_min_ftz_nan_xorsign_abs,
    F16 = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f16x2",
    F32 = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f",
);
impl_binary!(
    MaxXorsignAbsFtzNan,
    __intrinsic_max_ftz_nan_xorsign_abs,
    F16 = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f16",
    // F16x2 = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f16x2",
    F32 = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f",
);

// Fast Unary Intrinsics (fastest available for each type)

impl_unary!(
    AbsFast,
    __intrinsic_abs_fast,
    F16 = "llvm.nvvm.fabs.ftz.f16",
    // F16x2 = "llvm.nvvm.fabs.ftz.f16x2",
    BF16 = "llvm.nvvm.fabs.bf16",
    // BF16x2 = "llvm.nvvm.fabs.bf16x2",
    F32 = "llvm.nvvm.fabs.ftz.f32",
    F64 = "llvm.nvvm.fabs.f64", // No FTZ for F64
);

impl_unary!(
    Exp2Fast,
    __intrinsic_exp2_fast,
    // lg2 has an f64 variant but not ex2
    // F64 = "llvm.nvvm.ex2.approx.d",
    F32 = "llvm.nvvm.ex2.approx.ftz.f",
    F16 = "llvm.nvvm.ex2.approx.f16",
    // F16x2 = "llvm.nvvm.ex2.approx.f16x2",
    // PTX, but not LLVM, supports BF16 ex2
);

impl_unary!(
    Log2Fast,
    __intrinsic_log2_fast,
    F64 = "llvm.nvvm.lg2.approx.d",
    F32 = "llvm.nvvm.lg2.approx.ftz.f",
    // PTX doesn't seem to have FP16 or BF16 support
);

impl_unary!(
    FloorFast,
    __intrinsic_floor_fast,
    F32 = "llvm.nvvm.floor.ftz.f",
    F64 = "llvm.nvvm.floor.d",
    // PTX doesn't seem to support FP16 or BF16 ??
);

impl_unary!(
    CeilFast,
    __intrinsic_ceil_fast,
    F32 = "llvm.nvvm.ceil.ftz.f",
    F64 = "llvm.nvvm.ceil.d",
    // PTX doesn't seem to support FP16 or BF16 ??
);

impl_unary!(
    RsqrtFast,
    __intrinsic_rsqrt_fast,
    F32 = "llvm.nvvm.rsqrt.approx.ftz.f",
    F64 = "llvm.nvvm.rsqrt.approx.ftz.d",
);

impl_unary!(
    SqrtFast,
    __intrinsic_sqrt_fast,
    F32 = "llvm.nvvm.sqrt.approx.ftz.f",
    F64 = "llvm.nvvm.sqrt.rn.d",
);

impl_unary!(
    RcpFast,
    __intrinsic_rcp_fast,
    F32 = "llvm.nvvm.rcp.approx.ftz.f",
    F64 = "llvm.nvvm.rcp.approx.ftz.d",
);

impl_unary!(
    SinFast,
    __intrinsic_sin_fast,
    F32 = "llvm.nvvm.sin.approx.ftz.f",
);

impl_unary!(
    CosFast,
    __intrinsic_cos_fast,
    F32 = "llvm.nvvm.cos.approx.ftz.f",
);

// Fast Binary Intrinsics (fastest available for each type)

impl_binary!(
    MinFast,
    __intrinsic_min_fast,
    F16 = "llvm.nvvm.fmin.ftz.f16",
    // F16x2 = "llvm.nvvm.fmin.ftz.f16x2",
    BF16 = "llvm.nvvm.fmin.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmin.bf16x2",
    F32 = "llvm.nvvm.fmin.ftz.f",
    F64 = "llvm.nvvm.fmin.d", // No FTZ for F64
);

impl_binary!(
    MaxFast,
    __intrinsic_max_fast,
    F16 = "llvm.nvvm.fmax.ftz.f16",
    // F16x2 = "llvm.nvvm.fmax.ftz.f16x2",
    BF16 = "llvm.nvvm.fmax.bf16",
    // PTX, but not LLVM, supports this
    // BF16x2 = "llvm.nvvm.fmax.bf16x2",
    F32 = "llvm.nvvm.fmax.ftz.f",
    F64 = "llvm.nvvm.fmax.d", // No FTZ for F64
);
