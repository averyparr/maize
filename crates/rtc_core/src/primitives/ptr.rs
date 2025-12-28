use std::marker::PhantomData;

use inkwell::{
    AddressSpace,
    context::ContextRef,
    values::{BasicValue, BasicValueEnum, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{FromCtx, M, P, R, Ty},
    val::{Holds, Val},
};

impl<'lt, T> Val<'lt, P<T>> {
    pub fn pointee_ty(&self) -> T
    where
        T: FromCtx,
    {
        T::new(self.cx().ctx())
    }

    /// # Safety:
    /// Treat this as identical to loading from a *mut T
    /// You must ensure that the underlying ctx lasts at least
    /// as long as `'ctx
    pub unsafe fn load_unchecked<'ctx>(&self) -> Val<'ctx, T>
    where
        T: Ty,
    {
        let pointee_ty = self.pointee_ty().basic_ty();
        let ptr = self.to_underlying();
        let cx = self.cx();
        // Safety: User promised the load is valid!
        let inner_val = unsafe { cx.load(pointee_ty, ptr) };
        // Safety: User promised 'ctx lasts as long as the underlying FnCodegen!
        let cx_extended = unsafe { self.cx_with_lifetime() };
        Val::new(cx_extended, inner_val)
    }

    /// # Safety:
    /// Treat this as identical to storing to a *mut T
    pub unsafe fn store_unchecked<Value>(&mut self, val: Value)
    where
        Value: Holds<T = T>,
        Value::T: Ty,
    {
        let ptr_val = self.to_underlying();
        let value = val.to_underlying();
        let cx = self.cx();
        // Safety: User promised that storing to *mut T is valid
        unsafe { cx.store(ptr_val, value) };
    }
}

impl<'r, 'lt, T> Val<'lt, R<'r, T>> {
    pub fn as_ptr(&self) -> Val<'lt, P<T>> {
        Val::new(self.cx(), self.to_underlying().as_basic_value_enum())
    }

    pub fn load(&self) -> Val<'lt, T>
    where
        T: Ty,
    {
        // Safety: We hold a shared reference
        unsafe { self.as_ptr().load_unchecked() }
    }
}

impl<'m, 'lt, T> Val<'lt, M<'m, T>> {
    pub fn as_ref(&self) -> Val<'lt, R<'m, T>> {
        Val::new(self.cx(), self.to_underlying().as_basic_value_enum())
    }
    pub fn as_ptr(&self) -> Val<'lt, P<T>> {
        self.as_ref().as_ptr()
    }
    pub fn load(&self) -> Val<'lt, T>
    where
        T: Ty,
    {
        self.as_ref().load()
    }
    pub fn store<Value>(&mut self, val: Value)
    where
        Value: Holds<T = T>,
        Value::T: Ty,
    {
        // Safety: We hold an exclusive reference
        unsafe { self.as_ptr().store_unchecked(val) }
    }
}
