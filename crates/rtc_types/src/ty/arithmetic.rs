use inkwell::builder::Builder;
use inkwell::values::BasicValue;
use inkwell::values::BasicValueEnum;
use inkwell::values::FloatMathValue;
use inkwell::values::IntMathValue;

use crate::ty::ValTy;
use crate::ty::raw::*;
use crate::ty::vec::VectorizableTy;
use crate::val::Val;
use crate::val::post_process;

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
    fn type_as_float(raw: BasicValueEnum<'_>) -> impl FloatMathValue<'_> {
        raw.into_float_value()
    }
    /// Secondary customization point: are we using default float types,
    /// or [scalable] vector values?
    fn type_as_int(raw: BasicValueEnum<'_>) -> impl IntMathValue<'_> {
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
                .map(|v| v.as_basic_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_add(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "add",
                )
                .map(|v| v.as_basic_value_enum()),
        };
        let raw = unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build an add");

        let raw = post_process(lhs.cx(), raw);

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
                .map(|v| v.as_basic_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_sub(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "sub",
                )
                .map(|v| v.as_basic_value_enum()),
        };
        let raw = unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build an sub");
        let raw = post_process(lhs.cx(), raw);

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
                .map(|v| v.as_basic_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_mul(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "mul",
                )
                .map(|v| v.as_basic_value_enum()),
        };
        let raw = unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build an mul");
        let raw = post_process(lhs.cx(), raw);

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
                .map(|v| v.as_basic_value_enum()),
            MathVariant::SignedInt => b
                .build_int_signed_div(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "sdiv",
                )
                .map(|v| v.as_basic_value_enum()),
            MathVariant::UnsignedInt => b
                .build_int_unsigned_div(
                    Self::type_as_int(lhs.raw()),
                    Self::type_as_int(rhs.raw()),
                    "sdiv",
                )
                .map(|v| v.as_basic_value_enum()),
        };
        let raw = unsafe { lhs.cx().with_builder(build) }.expect("Should be able to build a div");
        let raw = post_process(lhs.cx(), raw);

        // Safety: div is (T, T) -> T
        unsafe { Val::new(lhs.cx(), raw) }
    }
    fn neg<'a>(val: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => b
                .build_float_neg(Self::type_as_float(val.raw()), "fneg")
                .map(|v| v.as_basic_value_enum()),
            MathVariant::SignedInt | MathVariant::UnsignedInt => b
                .build_int_neg(Self::type_as_int(val.raw()), "neg")
                .map(|v| v.as_basic_value_enum()),
        };
        let raw = unsafe { val.cx().with_builder(build) }.expect("Should be able to build an neg");
        let raw = post_process(val.cx(), raw);

        // Safety: neg is (T) -> T
        unsafe { Val::new(val.cx(), raw) }
    }
}

pub unsafe trait IntMathTy: MathTy {
    fn rem<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => {
                let lhs = Self::type_as_float(lhs.raw());
                let rhs = Self::type_as_float(rhs.raw());
                b.build_float_rem(lhs, rhs, "float_rem")
                    .expect("Build float rem should work")
            }
            .as_basic_value_enum(),
            MathVariant::SignedInt => {
                let lhs = Self::type_as_int(lhs.raw());
                let rhs = Self::type_as_int(rhs.raw());
                b.build_int_signed_rem(lhs, rhs, "sint_rem")
                    .expect("Build sint rem should work")
                    .as_basic_value_enum()
            }
            MathVariant::UnsignedInt => {
                let lhs = Self::type_as_int(lhs.raw());
                let rhs = Self::type_as_int(rhs.raw());
                b.build_int_unsigned_rem(lhs, rhs, "sint_rem")
                    .expect("Build uint rem should work")
                    .as_basic_value_enum()
            }
        };

        let raw_ret = unsafe { cx.with_builder(build) };
        let raw = post_process(cx, raw_ret);
        unsafe { Val::new(cx, raw) }
    }
    fn left_shift<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let build = |b: Builder<'static>| match Self::MATH_VARIANT {
            MathVariant::Float => {
                panic!("Float left/right shifts are not supported");
            }
            MathVariant::SignedInt | MathVariant::UnsignedInt => {
                let lhs = Self::type_as_int(lhs.raw());
                let rhs = Self::type_as_int(rhs.raw());
                b.build_left_shift(lhs, rhs, "left_shift")
                    .expect("Build left shift should work")
                    .as_basic_value_enum()
            }
        };

        let raw_ret = unsafe { cx.with_builder(build) };
        let raw = post_process(cx, raw_ret);
        unsafe { Val::new(cx, raw) }
    }
    fn right_shift<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let sign_extend = match Self::MATH_VARIANT {
            MathVariant::Float => panic!("Float left/right shifts are not supported"),
            MathVariant::SignedInt => true,
            MathVariant::UnsignedInt => false,
        };

        let build = |b: Builder<'static>| {
            let lhs = Self::type_as_int(lhs.raw());
            let rhs = Self::type_as_int(rhs.raw());
            b.build_right_shift(lhs, rhs, sign_extend, "left_shift")
                .expect("Build left shift should work")
        };

        let raw_ret = unsafe { cx.with_builder(build) };
        let raw = post_process(cx, raw_ret.as_basic_value_enum());
        unsafe { Val::new(cx, raw) }
    }
    fn xor<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let lhs = Self::type_as_int(lhs.raw());
        let rhs = Self::type_as_int(rhs.raw());

        let raw_ret = unsafe { cx.with_builder(|b| b.build_xor(lhs, rhs, "xor_values")) }
            .expect("XOR should have built");
        let raw = post_process(cx, raw_ret.as_basic_value_enum());
        unsafe { Val::new(cx, raw) }
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

macro_rules! impl_int_math_ty {
    ($($tipes: ty),*) => {
        $(
            unsafe impl IntMathTy for $tipes {}
        )*
    };
}

impl_int_math_ty!(I8, I16, I32, I64, U8, U16, U32, U64);

unsafe impl<T: MathTy + VectorizableTy, const N: usize> MathTy for V<T, N> {
    const MATH_VARIANT: MathVariant = T::MATH_VARIANT;
    fn type_as_float(raw: BasicValueEnum<'_>) -> impl FloatMathValue<'_> {
        raw.into_vector_value()
    }
    fn type_as_int(raw: BasicValueEnum<'_>) -> impl IntMathValue<'_> {
        raw.into_vector_value()
    }
}
