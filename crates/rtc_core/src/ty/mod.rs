mod bool;
pub mod primitive;
pub mod primitive_vec;
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
    values::{BasicValue, BasicValueEnum, FloatMathValue, InstructionValue},
};

use crate::{
    codegen::{CodegenModule, FnCodegen},
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
    fn get_args_at_idx<'lt>(cm: &'lt CodegenModule<'static>, at_idx: u32) -> Val<'lt, Self> {
        let align_kind_id = Attribute::get_named_enum_kind_id("align");
        let cx = cm.cx();
        let align_attr = cx
            .ctx()
            .create_enum_attribute(align_kind_id, Self::ALIGN as _);
        cx.func()
            .add_attribute(AttributeLoc::Param(at_idx), align_attr);
        let val = cx
            .func()
            .get_nth_param(at_idx)
            .expect("Param number mismatch!");
        Val::new(cm, val)
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

pub(crate) fn make_ins_fast_math(ins: InstructionValue<'_>) {
    const ALL_FAST_MATH: u32 = 0b1111111;
    // ins.set_fast_math_flags(ALL_FAST_MATH)
}

pub trait ArithmeticTy: Ty {
    fn try_emit_add(
        cm: &CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_sub(
        cm: &CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_mul(
        cm: &CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_div(
        cm: &CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, BuilderError>;
    fn try_emit_neg(
        cm: &CodegenModule<'static>,
        val: Self::Value,
    ) -> Result<Self::Value, BuilderError>;

    fn build_add<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_add(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit add");
        Val::new(lhs.cm(), val.as_basic_value_enum())
    }
    fn build_sub<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_sub(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit sub");
        Val::new(lhs.cm(), val.as_basic_value_enum())
    }
    fn build_mul<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_mul(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit mul");
        Val::new(lhs.cm(), val.as_basic_value_enum())
    }
    fn build_div<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_div(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit div");
        Val::new(lhs.cm(), val.as_basic_value_enum())
    }
    fn build_neg<'lt>(val: Val<'lt, Self>) -> Val<'lt, Self> {
        let cm = val.cm();
        let val = Self::try_emit_neg(val.cm(), val.to_underlying()).expect("Could not emit neg");
        Val::new(cm, val.as_basic_value_enum())
    }
}
