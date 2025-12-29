use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F32, F64, Ty},
    val::{Holds, Val},
};

pub trait SqrtableType: Ty {
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

impl SqrtableType for F32 {
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

impl SqrtableType for F64 {
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
    fn call_intrinsic<Float: SqrtableType>(
        &self,
        val: Val<'_, Float>,
        intrinsic_name: &str,
    ) -> Val<'_, Float> {
        let ty = Float::new(self.cx_ref().ctx()).basic_ty();
        let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
        let fn_val = self.mod_ref().add_function(intrinsic_name, fn_ty, None);

        // Safety: We have a sqrt function and so calling sqrt() is safe.
        let call_site = unsafe {
            self.cx_ref().with_builder(|b| {
                b.build_call(
                    fn_val,
                    &[val.to_underlying().as_basic_value_enum().into()],
                    "sqrt",
                )
            })
        }
        .expect("Could not generate a call instruction");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cx_ref(), ret_val)
    }
    pub fn sqrt<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let default_val = Float::RN;
        self.call_intrinsic(val, default_val)
    }
    pub fn sqrt_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RN_FTZ.unwrap_or(Float::RN))
    }
    pub fn sqrt_approx<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let name = Float::APPROX.unwrap_or(Float::RN);
        self.call_intrinsic(val, name)
    }
    pub fn sqrt_approx_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        let selected_ins = Float::APPROX_FTZ.or(Float::RN_FTZ).unwrap_or(Float::RN);
        self.call_intrinsic(val, selected_ins)
    }

    /// Messy intrinsics
    pub fn sqrt_rz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RZ)
    }
    pub fn sqrt_rm<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RM)
    }
    pub fn sqrt_rp<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RP)
    }
    pub fn sqrt_rz_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RZ_FTZ.unwrap_or(Float::RZ))
    }
    pub fn sqrt_rm_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RM_FTZ.unwrap_or(Float::RM))
    }
    pub fn sqrt_rp_ftz<Float: SqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_intrinsic(val, Float::RP_FTZ.unwrap_or(Float::RP))
    }
}
