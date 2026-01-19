use inkwell::{
    context::ContextRef,
    types::VectorType,
    values::{BasicValueEnum, VectorValue},
};

use crate::{
    traits::indexes::IndexableTy,
    ty::{
        ArithmeticTy, FromCtx, Ty, V,
        primitive::{F16, HasFundamentalVectorTy},
    },
};

unsafe impl HasFundamentalVectorTy<2> for F16 {
    type VecTy = F16x2;
}

#[derive(Clone, Copy)]
pub struct F16x2(ContextRef<'static>);

impl FromCtx for F16x2 {
    fn new(ctx: ContextRef<'static>) -> Self {
        Self(ctx)
    }
}

impl Ty for F16x2 {
    const SIZE: usize = 4;
    const ALIGN: usize = 4;

    fn ctx(&self) -> ContextRef<'static> {
        self.0
    }

    type Type = VectorType<'static>;
    fn basic_ty(&self) -> Self::Type {
        self.ctx().f16_type().vec_type(2)
    }

    type Value = VectorValue<'static>;
    fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value {
        basic_val.into_vector_value()
    }
}

impl IndexableTy for F16x2 {
    const LEN: usize = 2;

    type ElemT = F16;

    type ParametrizedLen<const M: usize> = V<Self::ElemT, M>;
}

impl ArithmeticTy for F16x2 {
    fn try_emit_add(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        // Safety: we have two valid Self::Values so adding should be safe
        unsafe {
            cm.cx()
                .with_builder(|b| b.build_float_add(lhs, rhs, "fadd"))
        }
    }

    fn try_emit_sub(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        // Safety: we have two valid Self::Values so subtracting should be safe
        unsafe { cm.cx().with_builder(|b| b.build_float_sub(lhs, rhs, "add")) }
    }

    fn try_emit_mul(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        // Safety: we have two valid Self::Values so multiplying should be safe
        unsafe { cm.cx().with_builder(|b| b.build_float_mul(lhs, rhs, "mul")) }
    }

    fn try_emit_div(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        // Safety: we have two valid Self::Values so dividing should be safe
        unsafe { cm.cx().with_builder(|b| b.build_float_div(lhs, rhs, "div")) }
    }

    fn try_emit_neg(
        cm: &crate::codegen::CodegenModule<'static>,
        val: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        // Safety: we have a valid Self::Value so negating should eb safe
        unsafe { cm.cx().with_builder(|b| b.build_float_neg(val, "neg")) }
    }
}
