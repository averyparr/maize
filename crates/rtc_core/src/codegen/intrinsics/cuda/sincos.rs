use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{Ty, primitive::F32},
    val::Val,
};

/// # Safety:
/// All of these intrinsic names must be valid unary
/// intrinsics which take values of a type Ty::Value.
pub unsafe trait SinCosableType: Ty {
    const SIN_APPROX: &str;
    const SIN_APPROX_FTZ: Option<&str>;
    const COS_APPROX: &str;
    const COS_APPROX_FTZ: Option<&str>;
}

// Safety: These are the NVVM intrinsics for Sin/Cos for f32
unsafe impl SinCosableType for F32 {
    const SIN_APPROX: &str = "llvm.nvvm.sin.approx.f";
    const SIN_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.sin.approx.ftz.f");
    const COS_APPROX: &str = "llvm.nvvm.cos.approx.f";
    const COS_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.cos.approx.ftz.f");
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    // Sin
    pub fn sin<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::SIN_APPROX;
        // Safety: `Float` implements `SinCosableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn sin_ftz<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::SIN_APPROX_FTZ.unwrap_or(Float::SIN_APPROX);
        // Safety: `Float` implements `SinCosableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    // Cos
    pub fn cos<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::COS_APPROX;
        // Safety: `Float` implements `SinCosableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn cos_ftz<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::COS_APPROX_FTZ.unwrap_or(Float::COS_APPROX);
        // Safety: `Float` implements `SinCosableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
}
