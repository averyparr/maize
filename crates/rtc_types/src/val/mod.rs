mod ptr;
mod std_ops;

use std::marker::PhantomData;

use inkwell::{
    context::ContextRef,
    types::AnyType,
    values::{AnyValue, AnyValueEnum},
};

use crate::{
    codegen::FnCodegen,
    ty::{SizedTy, ValTy},
};

#[derive(Clone, Copy)]
pub struct Val<'ctx, T: ?Sized>(&'ctx FnCodegen, AnyValueEnum<'ctx>, PhantomData<T>);

impl<'ctx, T: ?Sized> Val<'ctx, T>
where
    T: ValTy,
{
    pub(crate) fn ctx(&self) -> ContextRef<'ctx> {
        self.0.ctx()
    }
    pub(crate) fn cx(&self) -> &'ctx FnCodegen {
        self.0
    }
    pub(crate) unsafe fn new(cx: &'ctx FnCodegen, val: AnyValueEnum<'ctx>) -> Self {
        assert_eq!(T::ty(cx.ctx()).as_any_type_enum(), val.get_type());
        Self(cx, val, PhantomData)
    }
    /// # Safety: This is identical to ::std::mem::transmute.
    pub unsafe fn transmute<U: ?Sized>(val: Self) -> Val<'ctx, U>
    where
        T: SizedTy,
        U: for<'a> SizedTy<Type<'a> = T::Type<'a>, Value<'a> = T::Value<'a>>,
    {
        unsafe { Val::new(val.cx(), val.raw()) }
    }
    pub(crate) fn raw(&self) -> AnyValueEnum<'ctx> {
        self.1
    }
    pub(crate) fn ll_typed(&self) -> T::Value<'ctx> {
        T::type_val(self.raw())
    }
}
