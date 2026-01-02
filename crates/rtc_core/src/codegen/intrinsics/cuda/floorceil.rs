use crate::{
    codegen::func_with_args::Func,
    ty::{Ty, primitive::*},
    val::Val,
};

pub trait HasFloorCeil: Ty {
    const FLOOR: &str;
    const FLOOR_FTZ: Option<&str>;
    const CEIL: &str;
    const CEIL_FTZ: Option<&str>;
}

impl HasFloorCeil for F32 {
    const FLOOR: &str = "llvm.nvvm.floor.f";
    const FLOOR_FTZ: Option<&str> = Some("llvm.nvvm.floor.ftz.f");
    const CEIL: &str = "llvm.nvvm.ceil.f";
    const CEIL_FTZ: Option<&str> = Some("llvm.nvvm.ceil.ftz.f");
}

impl HasFloorCeil for F64 {
    const FLOOR: &str = "llvm.nvvm.floor.d";
    const FLOOR_FTZ: Option<&str> = None;
    const CEIL: &str = "llvm.nvvm.ceil.d";
    const CEIL_FTZ: Option<&str> = None;
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn floor<T: HasFloorCeil>(&self, val: Val<'_, T>) -> Val<'_, T> {
        // Safety: `T` `HasFloorCeil` intrinsic
        unsafe { self.cm_ref().call_unary_function(val, T::FLOOR) }
    }

    pub fn floor_ftz<T: HasFloorCeil>(&self, val: Val<'_, T>) -> Val<'_, T> {
        // Safety: `T` `HasFloorCeil` intrinsic
        unsafe {
            self.cm_ref()
                .call_unary_function(val, T::FLOOR_FTZ.unwrap_or(T::FLOOR))
        }
    }

    pub fn ceil<T: HasFloorCeil>(&self, val: Val<'_, T>) -> Val<'_, T> {
        // Safety: `T` `HasFloorCeil` intrinsic
        unsafe { self.cm_ref().call_unary_function(val, T::CEIL) }
    }

    pub fn ceil_ftz<T: HasFloorCeil>(&self, val: Val<'_, T>) -> Val<'_, T> {
        // Safety: `T` `HasFloorCeil` intrinsic
        unsafe {
            self.cm_ref()
                .call_unary_function(val, T::CEIL_FTZ.unwrap_or(T::CEIL))
        }
    }
}
