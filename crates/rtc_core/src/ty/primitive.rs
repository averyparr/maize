use inkwell::{
    context::ContextRef,
    types::{FloatType, IntType},
    values::{BasicValueEnum, FloatValue, IntValue},
};

use crate::ty::{ArithmeticTy, FromCtx, Ty};

macro_rules! derive_primitive {
    ($name: ident, $prim: ty, $enum_ty: ident, $enum_val: ident, $basic_ty: ident, $basic_val: ident, $add: ident, $sub: ident, $mul: ident, $div: ident, $neg: ident, $size: literal, $align: literal) => {
        #[derive(Clone, Copy)]
        pub struct $name(ContextRef<'static>);

        impl FromCtx for $name {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self(ctx)
            }
        }

        impl Ty for $name {
            const SIZE: usize = $size;
            const ALIGN: usize = $align;

            fn ctx(&self) -> ContextRef<'static> {
                self.0
            }

            type Type = $enum_ty<'static>;
            fn basic_ty(&self) -> Self::Type {
                self.ctx().$basic_ty()
            }

            type Value = $enum_val<'static>;
            fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value {
                basic_val.$basic_val()
            }
        }

        impl ArithmeticTy for $name {
            fn try_emit_add(
                cm: &crate::codegen::CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: we have two valid Self::Values so adding should be safe
                unsafe { cm.cx().with_builder(|b| b.$add(lhs, rhs, "fadd")) }
            }

            fn try_emit_sub(
                cm: &crate::codegen::CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: we have two valid Self::Values so subtracting should be safe
                unsafe { cm.cx().with_builder(|b| b.$sub(lhs, rhs, "add")) }
            }

            fn try_emit_mul(
                cm: &crate::codegen::CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: we have two valid Self::Values so multiplying should be safe
                unsafe { cm.cx().with_builder(|b| b.$mul(lhs, rhs, "mul")) }
            }

            fn try_emit_div(
                cm: &crate::codegen::CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: we have two valid Self::Values so dividing should be safe
                unsafe { cm.cx().with_builder(|b| b.$div(lhs, rhs, "div")) }
            }

            fn try_emit_neg(
                cm: &crate::codegen::CodegenModule<'static>,
                val: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: we have a valid Self::Value so negating should eb safe
                unsafe { cm.cx().with_builder(|b| b.$neg(val, "neg")) }
            }
        }
    };

    (float: $name: ident, $prim: ty, $basic_ty: ident, $size: literal, $align: literal) => {
        derive_primitive!(
            $name,
            $prim,
            FloatType,
            FloatValue,
            $basic_ty,
            into_float_value,
            build_float_add,
            build_float_sub,
            build_float_mul,
            build_float_div,
            build_float_neg,
            $size,
            $align
        );
    };

    (int: $name: ident, $prim: ty, $basic_ty: ident, $size: literal, $align: literal) => {
        derive_primitive!(
            $name,
            $prim,
            IntType,
            IntValue,
            $basic_ty,
            into_int_value,
            build_int_add,
            build_int_sub,
            build_int_mul,
            build_int_signed_div,
            build_int_neg,
            $size,
            $align
        );
    };

    (uint: $name: ident, $prim: ty, $basic_ty: ident, $size: literal, $align: literal) => {
        derive_primitive!(
            $name,
            $prim,
            IntType,
            IntValue,
            $basic_ty,
            into_int_value,
            build_int_add,
            build_int_sub,
            build_int_mul,
            build_int_unsigned_div,
            build_int_neg,
            $size,
            $align
        );
    };
}

derive_primitive!(float: F16, f32, f16_type, 2, 2);
derive_primitive!(float: F32, f32, f32_type, 4, 4);
derive_primitive!(float: F64, f64, f64_type, 8, 8);

derive_primitive!(int: I8, i8, i8_type, 1, 1);
derive_primitive!(int: I16, i16, i16_type, 2, 2);
derive_primitive!(int: I32, i32, i32_type, 4, 4);
derive_primitive!(int: I64, i64, i64_type, 8, 8);
derive_primitive!(int: I128, i128, i128_type, 16, 16);

derive_primitive!(uint: U8, u8, i8_type, 1, 1);
derive_primitive!(uint: U16, u16, i16_type, 2, 2);
derive_primitive!(uint: U32, u32, i32_type, 4, 4);
derive_primitive!(uint: U64, u64, i64_type, 8, 8);
derive_primitive!(uint: U128, u128, i128_type, 16, 1);
