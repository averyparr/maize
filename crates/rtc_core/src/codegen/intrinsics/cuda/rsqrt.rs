use crate::{
    codegen::func_with_args::Func,
    ty::{Ty, primitive::*},
    val::Val,
};

/// # Safety:
/// All of these intrinsic names must be valid unary
/// intrinsics which take values of a type Ty::Value.
pub unsafe trait RsqrtableType: Ty {
    const APPROX: &str;
    const APPROX_FTZ: Option<&str>;
}

// SAFETY: These are the valid rsqrt intrinsics for f32
unsafe impl RsqrtableType for F32 {
    const APPROX: &str = "llvm.nvvm.rsqrt.approx.f";
    const APPROX_FTZ: Option<&str> = Some("llvm.nvvm.rsqrt.approx.ftz.f");
}

unsafe impl RsqrtableType for F64 {
    const APPROX: &str = "llvm.nvvm.rsqrt.approx.d";
    const APPROX_FTZ: Option<&str> = Some("llvm.nvvm.rsqrt.approx.ftz.d");
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn rsqrt<Float: RsqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::APPROX;
        // SAFETY: `Float` ipmlements `RsqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rsqrt_ftz<Float: RsqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::APPROX_FTZ.unwrap_or(Float::APPROX);
        // SAFETY: `Float` ipmlements `RsqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
}
