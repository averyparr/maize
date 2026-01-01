use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F16, F32, F64, Ty},
    val::{Holds, Val},
};

pub trait HasAbs: Ty {
    const ABS: &str;
    const ABS_FTZ: Option<&str>;
}

impl HasAbs for F32 {
    const ABS: &str = "llvm.nvvm.fabs.f";
    const ABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.f");
}

impl HasAbs for F64 {
    const ABS: &str = "llvm.nvvm.fabs.d";
    const ABS_FTZ: Option<&str> = None;
}

impl HasAbs for F16 {
    const ABS: &str = "llvm.nvvm.fabs.f16";
    const ABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.f16");
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn fabs<T: HasAbs>(&self, val: Val<'_, T>) -> Val<'_, T> {
        // Safety: `T` `HasAbs` intrinsics
        unsafe { self.cm_ref().call_unary_function(val, T::ABS) }
    }

    pub fn fabs_ftz<T: HasAbs>(&self, val: Val<'_, T>) -> Val<'_, T> {
        // Safety: `T` `HasAbs` intrinsics
        unsafe {
            self.cm_ref()
                .call_unary_function(val, T::ABS_FTZ.unwrap_or(T::ABS))
        }
    }
}
