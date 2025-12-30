use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F32, F64, Ty},
    val::{Holds, Val},
};

pub trait RcpableType: Ty {
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

impl RcpableType for F32 {
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

impl RcpableType for F64 {
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
    fn call_rcp_intrinsic<Float: RcpableType>(
        &self,
        val: Val<'_, Float>,
        intrinsic_name: &str,
    ) -> Val<'_, Float> {
        let ty = Float::new(self.cx_ref().ctx()).basic_ty();
        let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
        let fn_val = self.mod_ref().add_function(intrinsic_name, fn_ty, None);

        let call_site = unsafe {
            self.cx_ref().with_builder(|b| {
                b.build_call(
                    fn_val,
                    &[val.to_underlying().as_basic_value_enum().into()],
                    "rcp",
                )
            })
        }
        .expect("Could not generate rcp call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    // Default rcp (round to nearest)
    pub fn rcp<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RN)
    }

    pub fn rcp_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RN_FTZ.unwrap_or(Float::RCP_RN))
    }

    // Approximate rcp
    pub fn rcp_approx_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_APPROX_FTZ)
    }

    // Rounding mode variants
    pub fn rcp_rz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RZ)
    }

    pub fn rcp_rm<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RM)
    }

    pub fn rcp_rp<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RP)
    }

    pub fn rcp_rz_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RZ_FTZ.unwrap_or(Float::RCP_RZ))
    }

    pub fn rcp_rm_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RM_FTZ.unwrap_or(Float::RCP_RM))
    }

    pub fn rcp_rp_ftz<Float: RcpableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rcp_intrinsic(val, Float::RCP_RP_FTZ.unwrap_or(Float::RCP_RP))
    }
}
