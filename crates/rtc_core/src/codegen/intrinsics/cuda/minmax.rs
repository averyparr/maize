use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F32, F64, Ty},
    val::{Holds, Val},
};

pub trait MinMaxableType: Ty {
    const MIN: &str;
    const MIN_FTZ: Option<&str>;
    const MIN_NAN: Option<&str>;
    const MIN_FTZ_NAN: Option<&str>;

    const MAX: &str;
    const MAX_FTZ: Option<&str>;
    const MAX_NAN: Option<&str>;
    const MAX_FTZ_NAN: Option<&str>;
}

impl MinMaxableType for F32 {
    const MIN: &str = "llvm.nvvm.fmin.f";
    const MIN_FTZ: Option<&str> = Some("llvm.nvvm.fmin.ftz.f");
    const MIN_NAN: Option<&str> = Some("llvm.nvvm.fmin.nan.f");
    const MIN_FTZ_NAN: Option<&str> = Some("llvm.nvvm.fmin.ftz_nan.f");

    const MAX: &str = "llvm.nvvm.fmax.f";
    const MAX_FTZ: Option<&str> = Some("llvm.nvvm.fmax.ftz.f");
    const MAX_NAN: Option<&str> = Some("llvm.nvvm.fmax.nan.f");
    const MAX_FTZ_NAN: Option<&str> = Some("llvm.nvvm.fmax.ftz_nan.f");
}

impl MinMaxableType for F64 {
    const MIN: &str = "llvm.nvvm.fmin.d";
    const MIN_FTZ: Option<&str> = None;
    const MIN_NAN: Option<&str> = None;
    const MIN_FTZ_NAN: Option<&str> = None;

    const MAX: &str = "llvm.nvvm.fmax.d";
    const MAX_FTZ: Option<&str> = None;
    const MAX_NAN: Option<&str> = None;
    const MAX_FTZ_NAN: Option<&str> = None;
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    fn call_minmax_intrinsic<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
        intrinsic_name: &str,
    ) -> Val<'_, Float> {
        let ty = Float::new(self.cx_ref().ctx()).basic_ty();
        let fn_ty = ty.fn_type(
            &[
                ty.as_basic_type_enum().into(),
                ty.as_basic_type_enum().into(),
            ],
            false,
        );
        let fn_val = self.mod_ref().add_function(intrinsic_name, fn_ty, None);

        let call_site = unsafe {
            self.cx_ref().with_builder(|builder| {
                builder.build_call(
                    fn_val,
                    &[
                        a.to_underlying().as_basic_value_enum().into(),
                        b.to_underlying().as_basic_value_enum().into(),
                    ],
                    "minmax",
                )
            })
        }
        .expect("Could not generate minmax call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    // Min variants
    pub fn fmin<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MIN)
    }

    pub fn fmin_ftz<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MIN_FTZ.unwrap_or(Float::MIN))
    }

    pub fn fmin_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MIN_NAN.unwrap_or(Float::MIN))
    }

    pub fn fmin_ftz_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MIN_FTZ_NAN.unwrap_or(Float::MIN))
    }

    // Max variants
    pub fn fmax<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MAX)
    }

    pub fn fmax_ftz<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MAX_FTZ.unwrap_or(Float::MAX))
    }

    pub fn fmax_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MAX_NAN.unwrap_or(Float::MAX))
    }

    pub fn fmax_ftz_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        self.call_minmax_intrinsic(a, b, Float::MAX_FTZ_NAN.unwrap_or(Float::MAX))
    }
}
