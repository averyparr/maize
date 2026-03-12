use inkwell::builder::Builder;
use inkwell::types::IntMathType;
use inkwell::values::BasicValue;
use inkwell::values::BasicValueEnum;
use inkwell::values::FastMathFlags;
use inkwell::values::FloatMathValue;
use inkwell::values::InstructionValue;
use inkwell::values::IntMathValue;

use crate::ty::SizedTy;
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

macro_rules! add_dispatched_binary_function {
    ($slf: ident, $fn_name:
        ident => $float_variant: ident $(($ffunc: ident))?,
        $signed_variant: ident  $(($sfunc: ident))?,
        $unsigned_variant: ident  $(($ufunc: ident))?$(,)?
    ) => {
        fn $fn_name<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
            let build = |b: Builder<'static>| match $slf::MATH_VARIANT {
                MathVariant::Float => b
                    .$float_variant(
                        Self::type_as_float(lhs.raw()),
                        Self::type_as_float(rhs.raw()),
                        concat!("f", stringify!($fn_name)),
                    )
                    .map(|v| {
                        $(if let Some(ins) = v.as_instruction_value() {
                            $ffunc(ins);
                        })?
                        v.as_basic_value_enum()
                    }),
                MathVariant::SignedInt => b
                    .$signed_variant(
                        Self::type_as_int(lhs.raw()),
                        Self::type_as_int(rhs.raw()),
                        concat!("i", stringify!($fn_name)),
                    )
                    .map(|v| {
                        $(if let Some(ins) = v.as_instruction_value() {
                            $sfunc(ins);
                        })?
                        v.as_basic_value_enum()
                    }),
                MathVariant::UnsignedInt => b
                    .$unsigned_variant(
                        Self::type_as_int(lhs.raw()),
                        Self::type_as_int(rhs.raw()),
                        concat!("u", stringify!($fn_name)),
                    )
                    .map(|v| {
                        $(if let Some(ins) = v.as_instruction_value() {
                            $ufunc(ins);
                        })?
                        v.as_basic_value_enum()
                    }),
            };
            let raw = unsafe { lhs.cx().with_builder(build) }
                .expect(stringify!("Should be able to build a[n] ", $fn_name));
            let raw = post_process(lhs.cx(), raw);

            // Safety: $fn_name is (T, T) -> T
            unsafe { Val::new(lhs.cx(), raw) }
        }
    };
}

fn no_special_float_values(ins: InstructionValue<'_>) {
    let flags = FastMathFlags::NoNaNs
        | FastMathFlags::NoInfs
        | FastMathFlags::NoSignedZeros
        | FastMathFlags::AllowContract;
    ins.set_fast_math_flags(flags)
        .expect("Setting fast math flags on float add should work");
}

fn exact_div_or_rem(ins: InstructionValue<'_>) {
    ins.set_exact_flag(true)
        .expect("Called on an invalid math op")
}

/// Safety: Implementing this trait requires that nothing is done
/// with the `builder` other than emit e.g. a (T, T) -> T add
/// operation. This is true for all implementations in this file,
/// but downstream implementors must be careful.
pub unsafe trait MathTy: SizedTy {
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
    add_dispatched_binary_function!(Self, add => build_float_add, build_int_add, build_int_add);
    add_dispatched_binary_function!(Self, add_no_wrap => build_float_add (no_special_float_values), build_int_nsw_add, build_int_nuw_add);
    add_dispatched_binary_function!(Self, sub => build_float_sub, build_int_sub, build_int_sub);
    add_dispatched_binary_function!(Self, sub_no_wrap => build_float_sub (no_special_float_values), build_int_nsw_sub, build_int_nuw_sub);
    add_dispatched_binary_function!(Self, mul => build_float_mul, build_int_mul, build_int_mul);
    add_dispatched_binary_function!(Self, mul_no_wrap => build_float_mul (no_special_float_values), build_int_nsw_mul, build_int_nuw_mul);
    add_dispatched_binary_function!(Self, div => build_float_div, build_int_signed_div, build_int_unsigned_div);
    add_dispatched_binary_function!(Self, div_nonzero => build_float_div (no_special_float_values), build_int_exact_signed_div, build_int_unsigned_div (exact_div_or_rem));
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

pub unsafe trait BitMathTy:
    for<'a> ValTy<Type<'a>: IntMathType<'a>, Value<'a>: IntMathValue<'a>>
{
    fn xor<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let lhs = lhs.ll_typed();
        let rhs = rhs.ll_typed();

        let raw_ret = unsafe { cx.with_builder(|b| b.build_xor(lhs, rhs, "xor_values")) }
            .expect("XOR should have built");
        let raw = post_process(cx, raw_ret.as_basic_value_enum());
        unsafe { Val::new(cx, raw) }
    }
    fn and<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let lhs = lhs.ll_typed();
        let rhs = rhs.ll_typed();

        let raw_ret = unsafe { cx.with_builder(|b| b.build_and(lhs, rhs, "xor_values")) }
            .expect("AND should have built");
        let raw = post_process(cx, raw_ret.as_basic_value_enum());
        unsafe { Val::new(cx, raw) }
    }
    fn or<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        let cx = lhs.cx();

        let lhs = lhs.ll_typed();
        let rhs = rhs.ll_typed();

        let raw_ret = unsafe { cx.with_builder(|b| b.build_or(lhs, rhs, "xor_values")) }
            .expect("AND should have built");
        let raw = post_process(cx, raw_ret.as_basic_value_enum());
        unsafe { Val::new(cx, raw) }
    }
}

pub unsafe trait IntMathTy: MathTy + BitMathTy {
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
            unsafe impl BitMathTy for $tipes {}
            unsafe impl IntMathTy for $tipes {}
        )*
    };
}

unsafe impl BitMathTy for Bool {}

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

unsafe impl<T: BitMathTy + VectorizableTy, const N: usize> BitMathTy for V<T, N> {}
unsafe impl<T: IntMathTy + VectorizableTy, const N: usize> IntMathTy for V<T, N> {}
