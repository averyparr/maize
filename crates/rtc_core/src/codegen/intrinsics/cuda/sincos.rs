use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F32, Ty},
    val::{Holds, Val},
};

pub trait SinCosableType: Ty {
    const SIN_APPROX: &str;
    const SIN_APPROX_FTZ: Option<&str>;
    const COS_APPROX: &str;
    const COS_APPROX_FTZ: Option<&str>;
}

impl SinCosableType for F32 {
    const SIN_APPROX: &str = "llvm.nvvm.sin.approx.f";
    const SIN_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.sin.approx.ftz.f");
    const COS_APPROX: &str = "llvm.nvvm.cos.approx.f";
    const COS_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.cos.approx.ftz.f");
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    fn call_sincos_intrinsic<Float: Ty>(
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
                    "sincos",
                )
            })
        }
        .expect("Could not generate sin/cos call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    // Sin
    pub fn sin<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_sincos_intrinsic(val, Float::SIN_APPROX)
    }

    pub fn sin_ftz<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_sincos_intrinsic(val, Float::SIN_APPROX_FTZ.unwrap_or(Float::SIN_APPROX))
    }

    // Cos
    pub fn cos<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_sincos_intrinsic(val, Float::COS_APPROX)
    }

    pub fn cos_ftz<Float: SinCosableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_sincos_intrinsic(val, Float::COS_APPROX_FTZ.unwrap_or(Float::COS_APPROX))
    }
}
