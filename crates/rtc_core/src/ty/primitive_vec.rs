use std::ops::Add;

use inkwell::{
    context::ContextRef,
    types::VectorType,
    values::{InstructionValue, VectorValue},
};

use crate::{
    ty::{ArithmeticTy, FromCtx, Ty},
    val::Val,
};

#[derive(Clone, Copy)]
pub struct F16x2(ContextRef<'static>);

impl FromCtx for F16x2 {
    fn new(ctx: ContextRef<'static>) -> Self {
        Self(ctx)
    }
}

impl Ty for F16x2 {
    const SIZE: usize = 4;

    const ALIGN: u32 = 4;

    fn ctx(&self) -> ContextRef<'static> {
        self.0
    }

    type Type = VectorType<'static>;

    fn basic_ty(&self) -> Self::Type {
        self.ctx().f16_type().vec_type(2)
    }

    type Value = VectorValue<'static>;

    fn get_value(basic_val: inkwell::values::BasicValueEnum<'static>) -> Self::Value {
        basic_val.into_vector_value()
    }
}
