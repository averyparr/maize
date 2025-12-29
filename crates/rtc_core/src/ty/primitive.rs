use inkwell::{
    builder::{Builder, BuilderError},
    context::ContextRef,
    types::{FloatType, IntType},
    values::{BasicValueEnum, FloatValue, IntValue},
};

use crate::ty::{ArithmeticTy, FromCtx, Ty};

macro_rules! float_impl {
    ($name: ident, $basic_ty: ident, $align: literal, $size: literal) => {
        #[derive(Clone, Copy)]
        pub struct $name(ContextRef<'static>);
        impl FromCtx for $name {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self(ctx)
            }
        }
        impl Ty for $name {
            const ALIGN: u32 = $align;
            const SIZE: usize = $size;

            fn ctx(&self) -> ContextRef<'static> {
                self.0
            }
            type Type = FloatType<'static>;
            fn basic_ty(&self) -> Self::Type {
                self.ctx().$basic_ty()
            }
            type Value = FloatValue<'static>;
            fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value {
                basic_val.into_float_value()
            }
        }

        impl ArithmeticTy for $name {
            fn try_emit_add(
                builder: Builder<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                builder.build_float_add(lhs, rhs, "add_float")
            }
            fn try_emit_sub(
                builder: Builder<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                builder.build_float_sub(lhs, rhs, "sub_float")
            }
            fn try_emit_mul(
                builder: Builder<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                builder.build_float_mul(lhs, rhs, "mul_float")
            }
            fn try_emit_div(
                builder: Builder<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                builder.build_float_div(lhs, rhs, "div_float")
            }
            fn try_emit_neg(
                builder: Builder<'static>,
                val: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                builder.build_float_neg(val, "neg_float")
            }
        }
    };
}

macro_rules! int_impl {
    ($name: ident, $basic_ty: ident, $align: literal, $size: literal) => {
        pub struct $name(ContextRef<'static>);
        impl FromCtx for $name {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self(ctx)
            }
        }
        impl Ty for $name {
            const ALIGN: u32 = $align;
            const SIZE: usize = $size;

            fn ctx(&self) -> ContextRef<'static> {
                self.0
            }
            type Type = IntType<'static>;
            fn basic_ty(&self) -> Self::Type {
                self.ctx().$basic_ty()
            }
            type Value = IntValue<'static>;
            fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value {
                basic_val.into_int_value()
            }
        }
    };
}

float_impl!(F16, f16_type, 2, 2);
float_impl!(F32, f32_type, 4, 4);
float_impl!(F64, f64_type, 8, 8);
float_impl!(F128, f128_type, 16, 16);

int_impl!(I8, i8_type, 1, 1);
int_impl!(I16, i16_type, 2, 2);
int_impl!(I32, i32_type, 4, 4);
int_impl!(I64, i64_type, 8, 8);
int_impl!(I128, i128_type, 16, 16);

int_impl!(U8, i8_type, 1, 1);
int_impl!(U16, i16_type, 2, 2);
int_impl!(U32, i32_type, 4, 4);
int_impl!(U64, i64_type, 8, 8);
int_impl!(U128, i128_type, 16, 16);

pub trait UnsignedInt {}
impl UnsignedInt for U8 {}
impl UnsignedInt for U16 {}
impl UnsignedInt for U32 {}
impl UnsignedInt for U64 {}
impl UnsignedInt for U128 {}

pub trait SignedInt {}
impl SignedInt for I8 {}
impl SignedInt for I16 {}
impl SignedInt for I32 {}
impl SignedInt for I64 {}
impl SignedInt for I128 {}
