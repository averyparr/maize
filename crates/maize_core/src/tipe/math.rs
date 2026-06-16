use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Sub};

use inkwell::{
    builder::{Builder, BuilderError},
    values::BasicValue,
};

use crate::{
    backend::{FnRef, UntypedValue},
    tipe::{BF16, F16, Ty},
    val::Val,
};

pub enum DivType {
    Signed,
    Unsigned,
    Float,
}

pub trait MathTy: Ty + Sized {
    fn try_emit_add(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
    fn try_emit_sub(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
    fn try_emit_mul(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
    fn try_emit_div(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
    const DIV_TYPE: DivType;
}

pub trait BitTy: Ty + Sized {
    fn try_emit_xor(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
    fn try_emit_and(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
    fn try_emit_or(
        b: Builder<'static>,
        fn_ref: FnRef,
        lhs: UntypedValue,
        rhs: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
}

pub trait SignedTy: MathTy {
    fn try_emit_neg(
        b: Builder<'static>,
        fn_ref: FnRef,
        val: UntypedValue,
    ) -> Result<Val<Self>, BuilderError>;
}

macro_rules! try_emit_impl {
    ($fn_name: ident, $build_call: ident, $out_name: expr) => {
        fn $fn_name(
            b: Builder<'static>,
            fn_ref: FnRef,
            lhs: UntypedValue,
            rhs: UntypedValue,
        ) -> Result<Val<Self>, BuilderError>
        where
            Self: Sized,
        {
            b.$build_call(Self::type_val(lhs), Self::type_val(rhs), $out_name)
                .map(|v| UntypedValue(v.as_basic_value_enum()))
                // Safety: primitive impls do produce the right output type
                .map(|uv| unsafe { Val::new(fn_ref, uv) })
        }
    };
}

macro_rules! float_add_impl {
    ($($tipes: ident),*) => {
        $(
            impl MathTy for $tipes {
                try_emit_impl!(try_emit_add, build_float_add, concat!(stringify!($tipes), "add"));
                try_emit_impl!(try_emit_sub, build_float_sub, concat!(stringify!($tipes), "add"));
                try_emit_impl!(try_emit_mul, build_float_mul, concat!(stringify!($tipes), "mul"));
                try_emit_impl!(try_emit_div, build_float_div, concat!(stringify!($tipes), "div"));
                const DIV_TYPE: DivType = DivType::Float;
            }
            impl SignedTy for $tipes {
                fn try_emit_neg(b: Builder<'static>, fn_ref: FnRef, val: UntypedValue) -> Result<Val<Self>, BuilderError> {
                    b
                        .build_float_neg(Self::type_val(val), concat!(stringify!($tipes), "neg"))
                        .map(|v| UntypedValue(v.as_basic_value_enum()))
                        // Safety: Primitive negation
                        .map(|uv| unsafe { Val::new(fn_ref, uv) })
                }
            }
        )*
    };
}

macro_rules! unsigned_add_impl {
    ($($tipes: ident),*) => {
        $(
            impl MathTy for $tipes {
                try_emit_impl!(try_emit_add, build_int_add, concat!(stringify!($tipes), "add"));
                try_emit_impl!(try_emit_sub, build_int_sub, concat!(stringify!($tipes), "add"));
                try_emit_impl!(try_emit_mul, build_int_mul, concat!(stringify!($tipes), "mul"));
                try_emit_impl!(try_emit_div, build_int_unsigned_div, concat!(stringify!($tipes), "div"));
                const DIV_TYPE: DivType = DivType::Unsigned;
            }
        )*
    };
}

macro_rules! signed_add_impl {
    ($($tipes: ident),*) => {
        $(
            impl MathTy for $tipes {
                try_emit_impl!(try_emit_add, build_int_add, concat!(stringify!($tipes), "add"));
                try_emit_impl!(try_emit_sub, build_int_sub, concat!(stringify!($tipes), "add"));
                try_emit_impl!(try_emit_mul, build_int_mul, concat!(stringify!($tipes), "mul"));
                try_emit_impl!(try_emit_div, build_int_signed_div, concat!(stringify!($tipes), "div"));
                const DIV_TYPE: DivType = DivType::Signed;
            }
            impl SignedTy for $tipes {
                fn try_emit_neg(b: Builder<'static>, fn_ref: FnRef, val: UntypedValue) -> Result<Val<Self>, BuilderError> {
                    b
                        .build_int_neg(Self::type_val(val), concat!(stringify!($tipes), "neg"))
                        .map(|v| UntypedValue(v.as_basic_value_enum()))
                        // Safety: Primitive negation
                        .map(|uv| unsafe { Val::new(fn_ref, uv) })
                }
            }
        )*
    };
}

float_add_impl!(F16, BF16, f32, f64);
unsigned_add_impl!(u8, u16, u32, u64);
signed_add_impl!(i8, i16, i32, i64);

macro_rules! bit_math_impl {
    ($($tipes: ty),*) => {
        $(
            impl BitTy for $tipes {
                fn try_emit_xor(
                    b: Builder<'static>,
                    fn_ref: FnRef,
                    lhs: UntypedValue,
                    rhs: UntypedValue,
                ) -> Result<Val<Self>, BuilderError> {
                    b.build_xor(Self::type_val(lhs), Self::type_val(rhs), "bxor")
                    .map(|v| UntypedValue(v.as_basic_value_enum()))
                    // Safety: primitive impls do produce the right output type
                    .map(|uv| unsafe { Val::new(fn_ref, uv) })
                }
                fn try_emit_and(
                    b: Builder<'static>,
                    fn_ref: FnRef,
                    lhs: UntypedValue,
                    rhs: UntypedValue,
                ) -> Result<Val<Self>, BuilderError> {
                    b.build_and(Self::type_val(lhs), Self::type_val(rhs), "bxor")
                    .map(|v| UntypedValue(v.as_basic_value_enum()))
                    // Safety: primitive impls do produce the right output type
                    .map(|uv| unsafe { Val::new(fn_ref, uv) })
                }
                fn try_emit_or(
                    b: Builder<'static>,
                    fn_ref: FnRef,
                    lhs: UntypedValue,
                    rhs: UntypedValue,
                ) -> Result<Val<Self>, BuilderError> {
                    b.build_or(Self::type_val(lhs), Self::type_val(rhs), "bxor")
                    .map(|v| UntypedValue(v.as_basic_value_enum()))
                    // Safety: primitive impls do produce the right output type
                    .map(|uv| unsafe { Val::new(fn_ref, uv) })
                }
            }
        )*
    };
}

bit_math_impl!(bool, u8, u16, u32, u64, i8, i16, i32, i64);

impl<T> Add for Val<T>
where
    T: MathTy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_add(b, fn_ref, self.raw(), rhs.raw()).expect("Add should have succeeded")
    }
}
impl<T> Sub for Val<T>
where
    T: MathTy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_sub(b, fn_ref, self.raw(), rhs.raw()).expect("Sub should have succeeded")
    }
}
impl<T> Mul for Val<T>
where
    T: MathTy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_mul(b, fn_ref, self.raw(), rhs.raw()).expect("Mul should have succeeded")
    }
}
impl<T> Div for Val<T>
where
    T: MathTy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_div(b, fn_ref, self.raw(), rhs.raw()).expect("Div should have succeeded")
    }
}

impl<T> Neg for Val<T>
where
    T: SignedTy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_neg(b, fn_ref, self.raw()).expect("Neg should have succeeded")
    }
}

impl<T> BitAnd for Val<T>
where
    T: BitTy,
{
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_and(b, fn_ref, self.raw(), rhs.raw()).expect("BitAnd should have succeeded")
    }
}

impl<T> BitOr for Val<T>
where
    T: BitTy,
{
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_or(b, fn_ref, self.raw(), rhs.raw()).expect("BitAnd should have succeeded")
    }
}

impl<T> BitXor for Val<T>
where
    T: BitTy,
{
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let fn_ref = self.fn_ref().clone();
        T::try_emit_xor(b, fn_ref, self.raw(), rhs.raw()).expect("BitAnd should have succeeded")
    }
}

impl std::ops::Not for Val<bool> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let raw = b
            .build_not(self.typed(), "lnot")
            .expect("logical not should succeed");
        unsafe { Val::new(self.fn_ref().clone(), UntypedValue(raw.into())) }
    }
}
