use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{Ty, primitive::*},
    val::Val,
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
    pub fn fmin<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, Float::MIN) }
    }

    pub fn fmin_ftz<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MIN_FTZ.unwrap_or(Float::MIN);
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }

    pub fn fmin_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MIN_NAN.unwrap_or(Float::MIN);
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }

    pub fn fmin_ftz_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MIN_FTZ_NAN.unwrap_or(Float::MIN);
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }

    pub fn fmax<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MAX;
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }

    pub fn fmax_ftz<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MAX_FTZ.unwrap_or(Float::MAX);
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }

    pub fn fmax_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MAX_NAN.unwrap_or(Float::MAX);
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }

    pub fn fmax_ftz_nan<Float: MinMaxableType>(
        &self,
        a: Val<'_, Float>,
        b: Val<'_, Float>,
    ) -> Val<'_, Float> {
        let intrinsic = Float::MAX_FTZ_NAN.unwrap_or(Float::MAX);
        // Safety: `Float` implements `MinMaxableType`
        unsafe { self.cm_ref().call_binary_function(a, b, intrinsic) }
    }
}
