pub mod primitive;
pub mod ptr;
pub mod vec;
pub mod void;

use std::u32;

pub use vec::*;
pub use void::Void;

use inkwell::{
    attributes::{Attribute, AttributeLoc},
    builder::BuilderError,
    context::ContextRef,
    types::{BasicMetadataTypeEnum, BasicType, FloatMathType, FunctionType, VectorType},
    values::{BasicValue, BasicValueEnum, FloatMathValue, InstructionValue},
};

use crate::{
    codegen::CodegenModule,
    ty::ptr::{M, P, R},
    val::Val,
};

pub trait FromCtx {
    fn new(ctx: ContextRef<'static>) -> Self;
}

pub trait Ty: FromCtx + Sized {
    const SIZE: usize;
    const ALIGN: usize;

    fn ctx(&self) -> ContextRef<'static>;
    type Type: BasicType<'static>;
    fn basic_ty(&self) -> Self::Type;

    type Value: BasicValue<'static>;
    fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value;

    fn ptr_ty(&self) -> P<Self> {
        P::new(self.ctx())
    }
    fn ref_ty(&self) -> R<&Self> {
        R::new(self.ctx())
    }
    fn mut_ty(&self) -> M<&mut Self> {
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
        // SAFETY: We just got this from a (hopefully?) strongly
        // typed function and so this should be a valid cast.
        unsafe { Val::new(cm, Self::get_value(val)) }
    }
}

pub trait VecTy: Ty<Type = VectorType<'static>> {
    const N: usize;
    type ElemT: Ty;
    type LenParametrized<const N: u32>: VecTy<ElemT = Self::ElemT>;
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
    ins.set_fast_math_flags(ALL_FAST_MATH)
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
        // SAFETY: We have just built this from an add
        // of two values
        unsafe { Val::new(lhs.cm(), val) }
    }
    fn build_sub<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_sub(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit sub");
        // SAFETY: We have just built this from a sub
        // of two values
        unsafe { Val::new(lhs.cm(), val) }
    }
    fn build_mul<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_mul(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit mul");
        // SAFETY: We have just built this from a mul
        // of two values
        unsafe { Val::new(lhs.cm(), val) }
    }
    fn build_div<'lt>(lhs: Val<'lt, Self>, rhs: Val<'lt, Self>) -> Val<'lt, Self> {
        assert!(lhs.cm() == rhs.cm(), "Vals must agree on FnCodegen");
        let val = Self::try_emit_div(lhs.cm(), lhs.to_underlying(), rhs.to_underlying())
            .expect("Could not emit div");
        // SAFETY: We have just built this from a div
        // of two values
        unsafe { Val::new(lhs.cm(), val) }
    }
    fn build_neg<'lt>(val: Val<'lt, Self>) -> Val<'lt, Self>
    where
        Self: 'lt,
    {
        let cm = val.cm();
        let neg_val =
            Self::try_emit_neg(val.cm(), val.to_underlying()).expect("Could not emit neg");
        // SAFETY: We have just built this from a neg of a value
        unsafe { Val::new(cm, neg_val) }
    }
}
