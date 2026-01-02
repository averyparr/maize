use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{Ty, primitive::*},
    val::Val,
};

/// # Safety:
/// All of these intrinsic names must be valid unary
/// intrinsics which take values of a type Ty::Value.
pub unsafe trait RcpableType: Ty {
    const RCP_RN: &str;
    const RCP_RZ: &str;
    const RCP_RM: &str;
    const RCP_RP: &str;

    const RCP_RN_FTZ: Option<&str>;
    const RCP_RZ_FTZ: Option<&str>;
    const RCP_RM_FTZ: Option<&str>;
    const RCP_RP_FTZ: Option<&str>;

    const RCP_APPROX_FTZ: &str;
}

// SAFETY: These are the rcp variants for f32.
unsafe impl RcpableType for F32 {
    const RCP_RN: &str = "llvm.nvvm.rcp.rn.f";
    const RCP_RZ: &str = "llvm.nvvm.rcp.rz.f";
    const RCP_RM: &str = "llvm.nvvm.rcp.rm.f";
    const RCP_RP: &str = "llvm.nvvm.rcp.rp.f";

    const RCP_RN_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rn.ftz.f");
    const RCP_RZ_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rz.ftz.f");
    const RCP_RM_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rm.ftz.f");
    const RCP_RP_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rp.ftz.f");

    const RCP_APPROX_FTZ: &str = "llvm.nvvm.rcp.approx.ftz.f";
}

// SAFETY: These are the rcp variants which exist for f64.
unsafe impl RcpableType for F64 {
    const RCP_RN: &str = "llvm.nvvm.rcp.rn.d";
    const RCP_RZ: &str = "llvm.nvvm.rcp.rz.d";
    const RCP_RM: &str = "llvm.nvvm.rcp.rm.d";
    const RCP_RP: &str = "llvm.nvvm.rcp.rp.d";

    const RCP_RN_FTZ: Option<&str> = None;
    const RCP_RZ_FTZ: Option<&str> = None;
    const RCP_RM_FTZ: Option<&str> = None;
    const RCP_RP_FTZ: Option<&str> = None;

    const RCP_APPROX_FTZ: &str = "llvm.nvvm.rcp.approx.ftz.d";
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    // Default rcp (round to nearest)
    pub fn rcp<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RN;
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rcp_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RN_FTZ.unwrap_or(Float::RCP_RN);
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    // Approximate rcp
    pub fn rcp_approx_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_APPROX_FTZ;
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    // Rounding mode variants
    pub fn rcp_rz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RZ;
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rcp_rm<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RM;
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rcp_rp<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RP;
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rcp_rz_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RZ_FTZ.unwrap_or(Float::RCP_RZ);
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rcp_rm_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RM_FTZ.unwrap_or(Float::RCP_RM);
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }

    pub fn rcp_rp_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let intrinsic_name = Float::RCP_RP_FTZ.unwrap_or(Float::RCP_RP);
        // Safety: `Float` implements `RcpableType`.
        unsafe { self.cm_ref().call_unary_function(val, intrinsic_name) }
    }
}
