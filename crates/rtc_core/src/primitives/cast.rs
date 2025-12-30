use inkwell::{
    types::{FloatMathType, IntMathType},
    values::{BasicValue, FloatMathValue, InstructionOpcode, IntMathValue},
};

use crate::{
    ty::{SignedInt, Ty, UnsignedInt},
    val::{Holds, Val},
};

impl<'lt, T> Val<'lt, T>
where
    T: Ty + 'lt,
{
    pub fn float_cast<U>(self) -> Val<'lt, U>
    where
        T::Value: FloatMathValue<'static>,
        U: Ty<Type = <T::Value as FloatMathValue<'static>>::BaseType>,
    {
        let cx = self.cm().cx();
        let new_float_type = U::new(cx.ctx());
        // SAFETY: We have a float value so float cast should be valid
        let res = unsafe {
            cx.with_builder(|b| {
                b.build_float_cast(
                    self.to_underlying(),
                    new_float_type.basic_ty(),
                    "cast_float",
                )
            })
        }
        .expect("Should be able to cast floats!");
        Val::new(self.cm(), res.as_basic_value_enum())
    }


    pub fn int_cast<U>(self) -> Val<'lt, U>
    where
        T::Value: IntMathValue<'static>,
        U: Ty<Type = <T::Value as IntMathValue<'static>>::BaseType>,
    {
        let cx = self.cm().cx();
        let new_integer_type = U::new(cx.ctx());
        // SAFETY: We have an int value so int cast should be valid (module wrapping)
        let res = unsafe {
            cx.with_builder(|b| {
                b.build_int_cast(
                    self.to_underlying(),
                    new_integer_type.basic_ty(),
                    "cast_int",
                )
            })
        }
        .expect("Should be able to cast ints!");
        Val::new(self.cm(), res.as_basic_value_enum())
    }

    pub fn cast_int_to_float<U>(self) -> Val<'lt, U>
    where
        T: SignedInt,
        T::Value: IntMathValue<'static>,
        U:
            Ty<
                Type = <<T::Value as IntMathValue<'static>>::BaseType as IntMathType<
                    'static,
                >>::MathConvType,
            >,
        U::Type: FloatMathType<'static>,
    {
        let cx = self.cm().cx();
        let u_ty = U::basic_ty(&U::new(cx.ctx()));
        // SAFETY: We have an int and can safely cast to float type
        let val_inner = unsafe {
            cx.with_builder(|b| {
                b.build_signed_int_to_float(self.to_underlying(), u_ty, "signed_int_to_float")
            })
        }
        .expect("Cannot emit signed int to float cast");
        Val::new(self.cm(), val_inner.as_basic_value_enum())
    }
     
    pub fn cast_uint_to_float<U>(self) -> Val<'lt, U>
    where
        T: UnsignedInt,
        T::Value: IntMathValue<'static>,
        U:
            Ty<
                Type = <<T::Value as IntMathValue<'static>>::BaseType as IntMathType<
                    'static,
                >>::MathConvType,
            >,
        U::Type: FloatMathType<'static>,
    {
        let cx = self.cm().cx();
        let u_ty = U::basic_ty(&U::new(cx.ctx()));
        // SAFETY: We have an int and can safely cast to float type
        let val_inner = unsafe {
            cx.with_builder(|b| {
                b.build_unsigned_int_to_float(self.to_underlying(), u_ty, "signed_int_to_float")
            })
        }
        .expect("Cannot emit signed int to float cast");
        Val::new(self.cm(), val_inner.as_basic_value_enum())
    }

    pub fn cast_to_int<U>(self) -> Val<'lt, U>
    where
        T::Value: FloatMathValue<'static>,
        U:
            Ty<
                Type = <<T::Value as FloatMathValue<'static>>::BaseType as FloatMathType<
                    'static,
                >>::MathConvType,
            >,
        U::Type: IntMathType<'static>,
        U: SignedInt,
    {
        let cx = self.cm().cx();
        let u_ty = U::basic_ty(&U::new(cx.ctx()));
        // SAFETY: We have a float and can safely cast to U type
        let val_inner = unsafe {
            cx.with_builder(|b| {
                b.build_float_to_signed_int(self.to_underlying(), u_ty, "float_to_signed_int")
            })
        }
        .expect("Cannot emit float to int cast");
        Val::new(self.cm(), val_inner.as_basic_value_enum())
    }

    pub fn cast_to_uint<U>(self) -> Val<'lt, U>
    where
        T::Value: FloatMathValue<'static>,
        U:
            Ty<
                Type = <<T::Value as FloatMathValue<'static>>::BaseType as FloatMathType<
                    'static,
                >>::MathConvType,
            >,
        U::Type: IntMathType<'static>,
        U: UnsignedInt,
    {
        let cx = self.cm().cx();
        let u_ty = U::basic_ty(&U::new(cx.ctx()));
        // SAFETY: We have a float and can safely cast to U type
        let val_inner = unsafe {
            cx.with_builder(|b| {
                b.build_float_to_unsigned_int(self.to_underlying(), u_ty, "float_to_signed_int")
            })
        }
        .expect("Cannot emit float to int cast");
        Val::new(self.cm(), val_inner.as_basic_value_enum())
    }
}
