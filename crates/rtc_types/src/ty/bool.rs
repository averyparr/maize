use inkwell::{
    context::ContextRef,
    types::IntType,
    values::{AnyValueEnum, IntValue},
};

use crate::ty::{AlignedTy, ValTy, sized::HasMaterializedType};

use super::{AnyTy, SizedTy, raw::Bool};

impl AnyTy for Bool {
    type AnyType<'ctx> = IntType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        ctx.bool_type()
    }
}
impl ValTy for Bool {
    type Value<'ctx> = IntValue<'ctx>;
    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        ctx.bool_type().get_undef()
    }
    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        ctx.bool_type().const_zero()
    }
    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        if let AnyValueEnum::IntValue(val) = val {
            Some(val)
        } else {
            None
        }
    }
}
impl HasMaterializedType for Bool {
    type Materialized = bool;
}
