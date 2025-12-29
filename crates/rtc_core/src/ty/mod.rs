mod bool;
pub mod primitive;
mod ptr;
mod vec;
mod void;

use std::u32;

pub use bool::Bool;
pub use primitive::*;
pub use ptr::*;
pub use vec::*;
pub use void::Void;

use inkwell::{
    attributes::{Attribute, AttributeLoc},
    builder::{Builder, BuilderError},
    context::ContextRef,
    types::{BasicMetadataTypeEnum, BasicType, FloatMathType, FunctionType},
    values::{BasicValue, BasicValueEnum, FloatMathValue},
};

use crate::{
    codegen::FnCodegen,
    val::{Holds, Val},
};

pub trait FromCtx {
    fn new(ctx: ContextRef<'static>) -> Self;
}

pub trait Ty: FromCtx + Sized {
    const SIZE: usize;
    const ALIGN: u32;

    fn ctx(&self) -> ContextRef<'static>;
    type Type: BasicType<'static>;
    fn basic_ty(&self) -> Self::Type;

    type Value: BasicValue<'static>;
    fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value;

    fn ptr_ty(&self) -> P<Self> {
        P::new(self.ctx())
    }
    fn ref_ty(&self) -> R<Self> {
        R::new(self.ctx())
    }
    fn mut_ty(&self) -> M<Self> {
        M::new(self.ctx())
    }
    fn get_args_at_idx<'lt>(cx: &'lt FnCodegen<'static>, at_idx: u32) -> Val<'lt, Self> {
        let align_kind_id = Attribute::get_named_enum_kind_id("align");
        let align_attr = cx
            .ctx()
            .create_enum_attribute(align_kind_id, Self::ALIGN as _);
        cx.func()
            .add_attribute(AttributeLoc::Param(at_idx), align_attr);
        let val = cx
            .func()
            .get_nth_param(at_idx)
            .expect("Param number mismatch!");
        Val::new(cx, val)
    }
}

pub trait VecTy: Ty {
    const N: usize;
    type ElemT: Ty;
}

pub trait FnReturnTy: FromCtx {
    fn func_type(&self, args: &[BasicMetadataTypeEnum<'static>]) -> FunctionType<'static>;
}

impl<T> FnReturnTy for T
where
    T: Ty,
{
    fn func_type(&self, args: &[BasicMetadataTypeEnum<'static>]) -> FunctionType<'static> {
        self.basic_ty().fn_type(args, false)
    }
}

pub trait FloatTy: Ty {
    type FloatVal: FloatMathValue<'static>;
    fn float_ty(&self) -> impl FloatMathType<'static>;
}

const ALL_FAST_MATH: u32 = 0b1111111;

pub trait ArithmeticTy: Ty {
    fn try_emit_add(
        builder: Builder<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_sub(
        builder: Builder<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_mul(
        builder: Builder<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_div(
        builder: Builder<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_neg(
        builder: Builder<'static>,
        val: Self::Value,
    ) -> Result<Self::Value, BuilderError>;

    fn build_add<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cx() == rhs.cx(), "Vals must agree on FnCodegen");
        // SAFETY: we have two ArithmeticTy so emitting an add is safe
        let val = unsafe {
            lhs.cx()
                .with_builder(|b| Self::try_emit_add(b, lhs.to_underlying(), rhs.to_underlying()))
        }
        .expect("Unable to emit add");
        if let Some(ins) = val.as_instruction_value() {
            ins.set_fast_math_flags(ALL_FAST_MATH);
        }
        Val::new(lhs.cx(), val.as_basic_value_enum())
    }
    fn build_sub<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cx() == rhs.cx(), "Vals must agree on FnCodegen");
        // SAFETY: we have two ArithmeticTy so emitting a sub is safe
        let val = unsafe {
            lhs.cx()
                .with_builder(|b| Self::try_emit_sub(b, lhs.to_underlying(), rhs.to_underlying()))
        }
        .expect("Unable to emit sub");
        if let Some(ins) = val.as_instruction_value() {
            ins.set_fast_math_flags(ALL_FAST_MATH);
        }
        Val::new(lhs.cx(), val.as_basic_value_enum())
    }
    fn build_mul<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cx() == rhs.cx(), "Vals must agree on FnCodegen");
        // SAFETY: we have two ArithmeticTy so emitting a mul is safe
        let val = unsafe {
            lhs.cx()
                .with_builder(|b| Self::try_emit_mul(b, lhs.to_underlying(), rhs.to_underlying()))
        }
        .expect("Unable to emit sub");
        if let Some(ins) = val.as_instruction_value() {
            ins.set_fast_math_flags(ALL_FAST_MATH);
        }
        Val::new(lhs.cx(), val.as_basic_value_enum())
    }
    fn build_div<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cx() == rhs.cx(), "Vals must agree on FnCodegen");
        // SAFETY: we have two ArithmeticTy so emitting a div is safe
        let val = unsafe {
            lhs.cx()
                .with_builder(|b| Self::try_emit_div(b, lhs.to_underlying(), rhs.to_underlying()))
        }
        .expect("Unable to emit sub");
        if let Some(ins) = val.as_instruction_value() {
            ins.set_fast_math_flags(ALL_FAST_MATH);
        }
        Val::new(lhs.cx(), val.as_basic_value_enum())
    }
    fn build_neg<'lt>(lhs: Val<'lt, Self>) -> Val<'lt, Self> {
        // SAFETY: we have a valid ArithmeticTy so emitting a neg is safe
        let val = unsafe {
            lhs.cx()
                .with_builder(|b| Self::try_emit_neg(b, lhs.to_underlying()))
        }
        .expect("Unable to emit sub");
        if let Some(ins) = val.as_instruction_value() {
            ins.set_fast_math_flags(ALL_FAST_MATH);
        }
        Val::new(lhs.cx(), val.as_basic_value_enum())
    }
}
