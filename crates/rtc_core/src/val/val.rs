use std::{marker::PhantomData, ops::Deref};

use inkwell::values::BasicValueEnum;

use crate::{codegen::FnCodegen, ty::Ty};

use super::{Holds, S, Val};

impl<'lt, T> Val<'lt, T> {
    pub fn cx(&self) -> &'lt FnCodegen<'static> {
        &self.cx
    }
    /// # Safety:
    /// The underlying FnCodegen must indeed be valid
    /// for 'any.
    pub unsafe fn cx_with_lifetime<'any>(&self) -> &'any FnCodegen<'static> {
        // SAFETY: This is a valid lifetime extention by
        // the above precondition
        unsafe { &*(self.cx as *const _) }
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
    pub(crate) fn new(cx: &'lt FnCodegen<'static>, val: BasicValueEnum<'static>) -> Self {
        Self {
            cx,
            val,
            phantom: PhantomData,
        }
    }

    pub fn with_storage(self) -> Val<'lt, S<T>> {
        Val::new_with_storage(self.cx(), self.val)
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
        T::new(self.cx().ctx()).basic_ty()
    }
    fn get_ty(&self) -> Self::T {
        T::new(self.cx().ctx())
    }
    fn held_cx(&self) -> &FnCodegen<'static> {
        self.cx()
    }
}
