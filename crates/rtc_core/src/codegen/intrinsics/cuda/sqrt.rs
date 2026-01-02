use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{Ty, primitive::*},
    val::Val,
};

/// # Safety:
/// All of these intrinsic names must be valid unary
/// intrinsics which take values of a type Ty::Value.
pub unsafe trait SqrtableType: Ty {
    const RN: &str;
    const RZ: &str;
    const RM: &str;
    const RP: &str;

    const RN_FTZ: Option<&str>;
    const RZ_FTZ: Option<&str>;
    const RM_FTZ: Option<&str>;
    const RP_FTZ: Option<&str>;

    const APPROX: Option<&str>;
    const APPROX_FTZ: Option<&str>;
}

// Safety: These are the NVVM intrinsics for sqrt for f32
unsafe impl SqrtableType for F32 {
    const RN: &str = "llvm.nvvm.sqrt.rn.f";
    const RZ: &str = "llvm.nvvm.sqrt.rz.f";
    const RM: &str = "llvm.nvvm.sqrt.rm.f";
    const RP: &str = "llvm.nvvm.sqrt.rp.f";

    const RN_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rn.ftz.f");
    const RZ_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rz.ftz.f");
    const RM_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rm.ftz.f");
    const RP_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rp.ftz.f");

    const APPROX: Option<&str> = Some("llvm.nvvm.sqrt.approx.f");
    const APPROX_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.approx.ftz.f");
}

// Safety: These are the NVVM intrinsics for sqrt for f64
unsafe impl SqrtableType for F64 {
    const RN: &str = "llvm.nvvm.sqrt.rn.d";
    const RZ: &str = "llvm.nvvm.sqrt.rz.d";
    const RM: &str = "llvm.nvvm.sqrt.rm.d";
    const RP: &str = "llvm.nvvm.sqrt.rp.d";

    const RN_FTZ: Option<&str> = None;
    const RZ_FTZ: Option<&str> = None;
    const RM_FTZ: Option<&str> = None;
    const RP_FTZ: Option<&str> = None;

    const APPROX: Option<&str> = None;
    const APPROX_FTZ: Option<&str> = None;
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn sqrt<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RN;
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RN_FTZ.unwrap_or(Float::RN);
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_approx<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::APPROX.unwrap_or(Float::RN);
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_approx_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::APPROX_FTZ.or(Float::RN_FTZ).unwrap_or(Float::RN);
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    /// Messy intrinsics
    pub fn sqrt_rz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RZ;
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_rm<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RM;
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_rp<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RP;
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_rz_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RZ_FTZ.unwrap_or(Float::RZ);
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_rm_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RM_FTZ.unwrap_or(Float::RM);
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
    pub fn sqrt_rp_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RP_FTZ.unwrap_or(Float::RP);
        // Safety: `Float` implements `SqrtableType`
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
}
