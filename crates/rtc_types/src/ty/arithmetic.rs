use inkwell::builder::Builder;
use inkwell::values::AnyValue;
use inkwell::values::AnyValueEnum;
use inkwell::values::FloatMathValue;
use inkwell::values::IntMathValue;

use crate::ty::ValTy;
use crate::ty::raw::*;
use crate::ty::vec::VectorizableTy;
use crate::val::Val;

pub enum MathVariant {
    Float,
    SignedInt,
    UnsignedInt,
}

/// Safety: Implementing this trait requires that nothing is done
/// with the `builder` other than emit e.g. a (T, T) -> T add
/// operation. This is true for all implementations in this file,
/// but downstream implementors must be careful.
pub unsafe trait MathTy: ValTy {
    /// Primary customization point: Do we use int or float math?
    const MATH_VARIANT: MathVariant;
    /// Secondary customization point: are we using default float types,
    /// or [scalable] vector values?
    fn type_as_float(raw: AnyValueEnum<'_>) -> impl FloatMathValue<'_> {
        raw.into_float_value()
    }
    /// Secondary customization point: are we using default float types,
    /// or [scalable] vector values?
    fn type_as_int(raw: AnyValueEnum<'_>) -> impl IntMathValue<'_> {
        raw.into_int_value()
    }

    // None of these should be overridden
    fn add<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => b
                .build_float_add(
                    Self::type_as_float(lhs.raw()),
                    Self::type_as_float(rhs.raw()),
                    "fadd",
                )
                .map(|v| v.as_any_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_add(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "add",
                )
                .map(|v| v.as_any_value_enum()),
        };
        let raw: inkwell::values::AnyValueEnum<'_> =
            unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build an add");

        // Safety: add is (T, T) -> T
        unsafe { Val::new(lhs.cx(), raw) }
    }
    fn sub<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => b
                .build_float_sub(
                    Self::type_as_float(lhs.raw()),
                    Self::type_as_float(rhs.raw()),
                    "fsub",
                )
                .map(|v| v.as_any_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_sub(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "sub",
                )
                .map(|v| v.as_any_value_enum()),
        };
        let raw: inkwell::values::AnyValueEnum<'_> =
            unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build an sub");

        // Safety: sub is (T, T) -> T
        unsafe { Val::new(lhs.cx(), raw) }
    }
    fn mul<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => b
                .build_float_mul(
                    Self::type_as_float(lhs.raw()),
                    Self::type_as_float(rhs.raw()),
                    "fmul",
                )
                .map(|v| v.as_any_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_mul(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "mul",
                )
                .map(|v| v.as_any_value_enum()),
        };
        let raw: inkwell::values::AnyValueEnum<'_> =
            unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build an mul");

        // Safety: mul is (T, T) -> T
        unsafe { Val::new(lhs.cx(), raw) }
    }
    fn div<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => b
                .build_float_div(
                    Self::type_as_float(lhs.raw()),
                    Self::type_as_float(rhs.raw()),
                    "fadd",
                )
                .map(|v| v.as_any_value_enum()),
            MathVariant::SignedInt => b
                .build_int_signed_div(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "sdiv",
                )
                .map(|v| v.as_any_value_enum()),
            MathVariant::UnsignedInt => b
                .build_int_unsigned_div(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "sdiv",
                )
                .map(|v| v.as_any_value_enum()),
        };
        let raw: inkwell::values::AnyValueEnum<'_> =
            unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build a div");

        // Safety: div is (T, T) -> T
        unsafe { Val::new(lhs.cx(), raw) }
    }
    fn neg<'a>(val: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => b
                .build_float_neg(Self::type_as_float(val.raw()), "fneg")
                .map(|v| v.as_any_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_neg(Self::type_as_int(val.raw()), "neg")
                .map(|v| v.as_any_value_enum()),
        };
        let raw: inkwell::values::AnyValueEnum<'_> =
            unsafe { val.cx().with_builder(build) }.expect("Should be able to build an neg");

        // Safety: neg is (T) -> T
        unsafe { Val::new(val.cx(), raw) }
    }
}

macro_rules! impl_math_ty {
    (
        $($tipes: ty),*: $math_variant: ident;
        ) => {
        $(
unsafe impl MathTy for $tipes {
    const MATH_VARIANT: MathVariant = MathVariant::$math_variant;
}
        )*
    };
}

impl_math_ty!(
    BF16, F16, F32, F64, F128: Float;
);

impl_math_ty!(
    I8, I16, I32, I64, I128: SignedInt;
);

impl_math_ty!(
    U8, U16, U32, U64, U128: UnsignedInt;
);

unsafe impl<T: MathTy + VectorizableTy, const N: usize> MathTy for V<T, N> {
    const MATH_VARIANT: MathVariant = T::MATH_VARIANT;
    fn type_as_float(raw: AnyValueEnum<'_>) -> impl FloatMathValue<'_> {
        raw.into_vector_value()
    }
    fn type_as_int(raw: AnyValueEnum<'_>) -> impl IntMathValue<'_> {
        raw.into_vector_value()
    }
}
