use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F32, F64, Ty},
    val::{Holds, Val},
};

pub trait RsqrtableType: Ty {
    const APPROX: &str;
    const APPROX_FTZ: Option<&str>;
}

impl RsqrtableType for F32 {
    const APPROX: &str = "llvm.nvvm.rsqrt.approx.f";
    const APPROX_FTZ: Option<&str> = Some("llvm.nvvm.rsqrt.approx.ftz.f");
}

impl RsqrtableType for F64 {
    const APPROX: &str = "llvm.nvvm.rsqrt.approx.d";
    const APPROX_FTZ: Option<&str> = Some("llvm.nvvm.rsqrt.approx.ftz.d");
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    fn call_rsqrt_intrinsic<Float: RsqrtableType>(
        &self,
        val: Val<'_, Float>,
        intrinsic_name: &str,
    ) -> Val<'_, Float> {
        let ty = Float::new(self.cx_ref().ctx()).basic_ty();
        let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
        let fn_val = self.mod_ref().add_function(intrinsic_name, fn_ty, None);

        // Safety: We have an rsqrt function and so calling rsqrt() is safe.
        let call_site = unsafe {
            self.cx_ref().with_builder(|b| {
                b.build_call(
                    fn_val,
                    &[val.to_underlying().as_basic_value_enum().into()],
                    "rsqrt",
                )
            })
        }
        .expect("Could not generate a call instruction");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    pub fn rsqrt<Float: RsqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rsqrt_intrinsic(val, Float::APPROX)
    }

    pub fn rsqrt_ftz<Float: RsqrtableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_rsqrt_intrinsic(val, Float::APPROX_FTZ.unwrap_or(Float::APPROX))
    }
}
