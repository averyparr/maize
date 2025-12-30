use std::{marker::PhantomData, ops::Deref};

use inkwell::values::BasicValueEnum;

use crate::{
    codegen::{CodegenModule, FnCodegen},
    ty::Ty,
};

use super::{Holds, S, Val};

impl<'lt, T> Val<'lt, T> {
    pub fn cm(&self) -> &'lt CodegenModule<'static> {
        &self.cm
    }
    /// # Safety:
    /// The underlying FnCodegen must indeed be valid
    /// for 'any.
    pub unsafe fn cm_with_lifetime<'any>(&self) -> &'any CodegenModule<'static> {
        // SAFETY: This is a valid lifetime extention by
        // the above precondition
        unsafe { &*(self.cm as *const _) }
    }
    pub fn get_val<'borrow>(&self) -> BasicValueEnum<'static>
    where
        'lt: 'borrow,
    {
        self.val
    }
}

impl<'lt, T> Val<'lt, T>
where
    T: Ty,
{
    pub(crate) fn new(cm: &'lt CodegenModule<'static>, val: BasicValueEnum<'static>) -> Self {
        Self {
            cm,
            val,
            phantom: PhantomData,
        }
    }

    pub fn with_storage(self) -> Val<'lt, S<T>> {
        Val::new_with_storage(self.cm(), self.val)
    }
}

impl<'lt, T> Holds for Val<'lt, T>
where
    T: Ty,
{
    type T = T;
    fn to_underlying(&self) -> T::Value {
        T::get_value(self.val)
    }
    fn to_underlying_ty(&self) -> T::Type {
        T::new(self.cm().cx().ctx()).basic_ty()
    }
    fn get_ty(&self) -> Self::T {
        T::new(self.cm().cx().ctx())
    }
    fn held_cm(&self) -> &CodegenModule<'static> {
        self.cm()
    }
}
