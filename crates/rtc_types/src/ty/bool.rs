use inkwell::{
    context::ContextRef,
    types::IntType,
    values::{AnyValueEnum, IntValue},
};

use crate::ty::{AlignedTy, ValTy};

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
impl AlignedTy for Bool {
    const ALIGN: u32 = ::std::mem::align_of::<bool>() as _;
}
impl SizedTy for Bool {
    const SIZE: u32 = ::std::mem::size_of::<bool>() as _;
}
