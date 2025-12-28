use std::marker::PhantomData;

use inkwell::{context::ContextRef, values::BasicValueEnum};

use crate::{codegen::Codegen, ty::BasicTy};

#[derive(Clone, Copy)]
pub struct Val<'ctx, T> {
    pub(crate) cx: Codegen<'ctx>,
    pub(crate) val: BasicValueEnum<'ctx>,
    pub(crate) phantom: PhantomData<T>,
}

impl<'ctx, T> Val<'ctx, T> {
    pub fn ctx(&self) -> ContextRef<'ctx> {
        self.cx.ctx()
    }
    fn get_ty(&self) -> T
    where
        T: BasicTy<'ctx>,
    {
        T::new(self.ctx())
    }
}

impl<'ctx, T> Val<'ctx, T>
where
    T: BasicTy<'ctx>,
{
    pub fn to_value(&self) -> T::Value {
        T::get_value(self.val)
    }
}
