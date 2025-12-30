use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F16, F32, F64, Ty},
    val::{Holds, Val},
};

pub trait Exp2ableType: Ty {
    const EX2_APPROX: &str;
    const EX2_APPROX_FTZ: Option<&str>;
}

impl Exp2ableType for F32 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.f";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.f");
}

impl Exp2ableType for F64 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.d";
    const EX2_APPROX_FTZ: Option<&str> = None;
}

impl Exp2ableType for F16 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.f16";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.f16");
}

pub trait Log2ableType: Ty {
    const LG2_APPROX: &str;
    const LG2_APPROX_FTZ: Option<&str>;
}

impl Log2ableType for F32 {
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.f";
    const LG2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.lg2.approx.ftz.f");
}

impl Log2ableType for F64 {
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.d";
    const LG2_APPROX_FTZ: Option<&str> = None;
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    fn call_unary_intrinsic<Float: Ty>(
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
                    "intrinsic",
                )
            })
        }
        .expect("Could not generate intrinsic call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    // Exp2
    pub fn exp2<Float: Exp2ableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_unary_intrinsic(val, Float::EX2_APPROX)
    }

    pub fn exp2_ftz<Float: Exp2ableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_unary_intrinsic(val, Float::EX2_APPROX_FTZ.unwrap_or(Float::EX2_APPROX))
    }

    // Log2
    pub fn log2<Float: Log2ableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_unary_intrinsic(val, Float::LG2_APPROX)
    }

    pub fn log2_ftz<Float: Log2ableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_unary_intrinsic(val, Float::LG2_APPROX_FTZ.unwrap_or(Float::LG2_APPROX))
    }
}
