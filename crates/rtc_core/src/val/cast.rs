use inkwell::{
    types::{FloatMathType, FloatType},
    values::{BasicValue, FloatMathValue, FloatValue},
};

use crate::{traits::HasCXVal, ty::Ty, val::Val};

pub trait CastableFrom<FromT> {
    fn cast_from(val: Val<'_, FromT>) -> Val<'_, Self>;
}

impl<FloatFrom, FloatTo, BaseInkwellType> CastableFrom<FloatFrom> for FloatTo
where
    FloatFrom: Ty,
    FloatTo: Ty<Type = BaseInkwellType, Value = FloatFrom::Value>,
    FloatFrom::Value: FloatMathValue<'static, BaseType = BaseInkwellType>,
    FloatTo::Value: FloatMathValue<'static, BaseType = BaseInkwellType>,
{
    fn cast_from(val: Val<'_, FloatFrom>) -> Val<'_, Self> {
        let llvm_val = val.to_underlying();
        let new_ty = Self::new(val.cx().ctx()).basic_ty();
        let cast_llvm_val = unsafe {
            val.cx()
                .with_builder(|b| b.build_float_cast(llvm_val, new_ty, "cast_float"))
        }
        .expect("should be able to float cast");
        if let Some(ins) = cast_llvm_val.as_instruction_value() {
            ins.set_fast_math_flags(0b11111111);
        }
        unsafe { Val::new(val.cm(), cast_llvm_val) }
    }
}

impl<'lt, CastFrom> Val<'lt, CastFrom> {
    pub fn cast<CastTo>(self) -> Val<'lt, CastTo>
    where
        CastTo: CastableFrom<CastFrom>,
    {
        CastTo::cast_from(self)
    }
}
