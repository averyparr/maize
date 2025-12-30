use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F16, F32, F64, Ty},
    val::{Holds, Val},
};

pub trait FabsableType: Ty {
    const FABS: &str;
    const FABS_FTZ: Option<&str>;
}

impl FabsableType for F32 {
    const FABS: &str = "llvm.nvvm.fabs.f";
    const FABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.f");
}

impl FabsableType for F64 {
    const FABS: &str = "llvm.nvvm.fabs.d";
    const FABS_FTZ: Option<&str> = None;
}

impl FabsableType for F16 {
    const FABS: &str = "llvm.nvvm.fabs.f16";
    const FABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.f16");
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    fn call_fabs_intrinsic<Float: FabsableType>(
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
                    "fabs",
                )
            })
        }
        .expect("Could not generate fabs call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    pub fn fabs<Float: FabsableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_fabs_intrinsic(val, Float::FABS)
    }

    pub fn fabs_ftz<Float: FabsableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_fabs_intrinsic(val, Float::FABS_FTZ.unwrap_or(Float::FABS))
    }
}
