use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{F32, F64, Ty},
    val::{Holds, Val},
};

pub trait FloorCeilableType: Ty {
    const FLOOR: &str;
    const FLOOR_FTZ: Option<&str>;
    const CEIL: &str;
    const CEIL_FTZ: Option<&str>;
}

impl FloorCeilableType for F32 {
    const FLOOR: &str = "llvm.nvvm.floor.f";
    const FLOOR_FTZ: Option<&str> = Some("llvm.nvvm.floor.ftz.f");
    const CEIL: &str = "llvm.nvvm.ceil.f";
    const CEIL_FTZ: Option<&str> = Some("llvm.nvvm.ceil.ftz.f");
}

impl FloorCeilableType for F64 {
    const FLOOR: &str = "llvm.nvvm.floor.d";
    const FLOOR_FTZ: Option<&str> = None;
    const CEIL: &str = "llvm.nvvm.ceil.d";
    const CEIL_FTZ: Option<&str> = None;
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    fn call_floorceil_intrinsic<Float: FloorCeilableType>(
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
                    "floorceil",
                )
            })
        }
        .expect("Could not generate floor/ceil call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        Val::new(self.cm_ref(), ret_val)
    }

    pub fn floor<Float: FloorCeilableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_floorceil_intrinsic(val, Float::FLOOR)
    }

    pub fn floor_ftz<Float: FloorCeilableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_floorceil_intrinsic(val, Float::FLOOR_FTZ.unwrap_or(Float::FLOOR))
    }

    pub fn ceil<Float: FloorCeilableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_floorceil_intrinsic(val, Float::CEIL)
    }

    pub fn ceil_ftz<Float: FloorCeilableType>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        self.call_floorceil_intrinsic(val, Float::CEIL_FTZ.unwrap_or(Float::CEIL))
    }
}
