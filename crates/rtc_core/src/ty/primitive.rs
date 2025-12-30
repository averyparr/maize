use inkwell::{
    builder::{Builder, BuilderError},
    context::ContextRef,
    llvm_sys::core::LLVMBFloatTypeInContext,
    types::{FloatType, IntType},
    values::{BasicValueEnum, FloatValue, IntValue},
};

use crate::ty::{ArithmeticTy, CodegenModule, FromCtx, Ty};

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
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                // Safety: We have two floats so an add should be safe
                unsafe {
                    cm.cx()
                        .with_builder(|b| b.build_float_add(lhs, rhs, "add_float"))
                }
            }
            fn try_emit_sub(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                // Safety: We have two floats so a sub should be safe
                unsafe {
                    cm.cx()
                        .with_builder(|b| b.build_float_sub(lhs, rhs, "add_float"))
                }
            }
            fn try_emit_mul(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                // Safety: We have two floats so a mul should be safe
                unsafe {
                    cm.cx()
                        .with_builder(|b| b.build_float_mul(lhs, rhs, "add_float"))
                }
            }
            fn try_emit_div(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                // Safety: We have two floats so a div should be safe
                unsafe {
                    cm.cx()
                        .with_builder(|b| b.build_float_div(lhs, rhs, "add_float"))
                }
            }
            fn try_emit_neg(
                cm: &CodegenModule<'static>,
                val: Self::Value,
            ) -> Result<Self::Value, BuilderError> {
                // Safety: We have a float so a neg should be safe
                unsafe {
                    cm.cx()
                        .with_builder(|b| b.build_float_neg(val, "neg_float"))
                }
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
