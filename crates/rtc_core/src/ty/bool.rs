use inkwell::{context::ContextRef, types::IntType, values::IntValue};

use crate::ty::{FromCtx, Ty};

pub struct Bool(ContextRef<'static>);

impl FromCtx for Bool {
    fn new(ctx: ContextRef<'static>) -> Self {
        Self(ctx)
    }
}

impl Ty for Bool {
    fn ctx(&self) -> ContextRef<'static> {
        self.0
    }
    type Type = IntType<'static>;
    fn basic_ty(&self) -> Self::Type {
        self.ctx().bool_type()
    }
    type Value = IntValue<'static>;
    fn get_value(basic_val: inkwell::values::BasicValueEnum<'static>) -> Self::Value {
        basic_val.into_int_value()
    }
}
