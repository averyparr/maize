mod ptr;
mod std_ops;

use std::marker::PhantomData;

use inkwell::{context::ContextRef, values::AnyValueEnum};

use crate::{codegen::FnCodegen, ty::ValTy};

#[derive(Clone, Copy)]
pub struct Val<'ctx, T: ?Sized>(&'ctx FnCodegen, AnyValueEnum<'ctx>, PhantomData<T>);

impl<'ctx, T> Val<'ctx, T>
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
        Self(cx, val, PhantomData)
    }
    pub(crate) fn raw(&self) -> AnyValueEnum<'ctx> {
        self.1
    }
    pub(crate) fn ll_typed(&self) -> T::Value<'ctx> {
        T::type_val(self.raw())
    }
}
