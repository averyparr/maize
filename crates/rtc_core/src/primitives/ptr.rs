use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use inkwell::{
    AddressSpace,
    context::ContextRef,
    values::{BasicValue, BasicValueEnum, PointerValue},
};

use crate::{
    ty::{FromCtx, Global, M, P, R, Shared, Ty},
    val::{Holds, Val},
};

pub trait RawPtrOps {
    type Pointee: Ty;
    fn pointee_ty(&self) -> Self::Pointee;
    /// # Safety:
    /// Treat this as identical to loading from a *mut T
    /// You must ensure that the underlying ctx lasts at least
    /// as long as `'ctx
    unsafe fn load_unchecked<'ctx>(&self) -> Val<'ctx, Self::Pointee>;
    /// # Safety:
    /// Treat this as identical to storing to a *mut T
    unsafe fn store_unchecked<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>;
}

pub trait RefOps: RawPtrOps {
    fn load(&self) -> Val<'_, Self::Pointee>;
}

pub trait MutOps: RefOps {
    fn store<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>;
}

impl<'r, 'lt, T> Val<'lt, R<'r, T>> {
    pub fn as_ptr(&self) -> Val<'lt, P<T>> {
        Val::new(self.cx(), self.to_underlying().as_basic_value_enum())
    }
}

impl<'m, 'lt, T> Val<'lt, M<'m, T>> {
    pub fn as_ref(&self) -> Val<'lt, R<'m, T>> {
        Val::new(self.cx(), self.to_underlying().as_basic_value_enum())
    }
    pub fn as_ptr(&self) -> Val<'lt, P<T>> {
        self.as_ref().as_ptr()
    }
}

impl<'lt, T> RawPtrOps for Val<'lt, P<T>>
where
    T: Ty,
{
    type Pointee = T;
    fn pointee_ty(&self) -> Self::Pointee {
        Self::Pointee::new(self.cx().ctx())
    }

    /// # Safety:
    /// See `RawPtrOps::load_unchecked`.
    unsafe fn load_unchecked<'ctx>(&self) -> Val<'ctx, Self::Pointee> {
        let pointee_ty = self.pointee_ty().basic_ty();
        let ptr = self.to_underlying();
        let cx = self.cx();
        // Safety: User promised the load is valid!
        let inner_val = unsafe { cx.load(pointee_ty, ptr) };
        // Safety: User promised 'ctx lasts as long as the underlying FnCodegen!
        let cx_extended = unsafe { self.cx_with_lifetime() };
        Val::new(cx_extended, inner_val)
    }
    unsafe fn store_unchecked<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>,
    {
        let ptr_val = self.to_underlying();
        let value = val.to_underlying();
        let cx = self.cx();
        // Safety: User promised that storing to *mut T is valid
        unsafe { cx.store(ptr_val, value) };
    }
}

impl<'r, 'lt, T> RawPtrOps for Val<'lt, R<'r, T>>
where
    T: Ty,
{
    type Pointee = T;
    fn pointee_ty(&self) -> Self::Pointee {
        self.as_ptr().pointee_ty()
    }
    unsafe fn load_unchecked<'ctx>(&self) -> Val<'ctx, Self::Pointee> {
        // Safety: We have a reference!
        unsafe { self.as_ptr().load_unchecked() }
    }
    unsafe fn store_unchecked<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>,
    {
        // Safety: User promised!
        unsafe { self.as_ptr().store_unchecked(val) };
    }
}

impl<'m, 'lt, T> RawPtrOps for Val<'lt, M<'m, T>>
where
    T: Ty,
{
    type Pointee = T;
    fn pointee_ty(&self) -> Self::Pointee {
        self.as_ptr().pointee_ty()
    }
    unsafe fn load_unchecked<'ctx>(&self) -> Val<'ctx, Self::Pointee> {
        // Safety: We have an exclusive reference
        unsafe { self.as_ptr().load_unchecked() }
    }
    unsafe fn store_unchecked<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>,
    {
        // Safety: We ahve an exclusive reference
        unsafe { self.as_ptr().store_unchecked(val) }
    }
}

impl<'r, 'lt, T> RefOps for Val<'lt, R<'r, T>>
where
    T: Ty,
{
    fn load(&self) -> Val<'_, Self::Pointee> {
        // Safety: We hold a shared reference
        unsafe { self.as_ptr().load_unchecked() }
    }
}

impl<'m, 'lt, T> RefOps for Val<'lt, M<'m, T>>
where
    T: Ty,
{
    fn load(&self) -> Val<'_, Self::Pointee> {
        // Safety: We hold an exclusive reference
        unsafe { self.as_ptr().load_unchecked() }
    }
}

impl<'m, 'lt, T> MutOps for Val<'lt, M<'m, T>>
where
    T: Ty,
{
    fn store<Value>(&mut self, val: Value)
    where
        Value: Holds<T = Self::Pointee>,
    {
        // Safety: We hold an exclusive reference
        unsafe { self.as_ptr().store_unchecked(val) }
    }
}

macro_rules! impl_ptr_wrapper {
    ($wrapper_name: ident) => {
        impl<'lt, Ptr> RawPtrOps for Val<'lt, $wrapper_name<Ptr>>
        where
            Val<'lt, Ptr>: RawPtrOps,
            Ptr: Ty,
        {
            type Pointee = <Val<'lt, Ptr> as RawPtrOps>::Pointee;
            fn pointee_ty(&self) -> Self::Pointee {
                self.to_inner().pointee_ty()
            }
            unsafe fn load_unchecked<'ctx>(&self) -> Val<'ctx, Self::Pointee> {
                // SAFETY: User promised!
                unsafe { self.to_inner().load_unchecked() }
            }
            unsafe fn store_unchecked<Value>(&mut self, val: Value)
            where
                Value: Holds<T = Self::Pointee>,
            {
                // SAFETY: User promised!
                unsafe { self.to_inner().store_unchecked(val) }
            }
        }

        impl<'lt, Ptr> RefOps for Val<'lt, $wrapper_name<Ptr>>
        where
            Val<'lt, Ptr>: RefOps,
            Ptr: Ty + 'lt,
        {
            fn load(&self) -> Val<'lt, Self::Pointee> {
                // Safety: We hold a shared reference
                unsafe { self.to_inner().load_unchecked() }
            }
        }

        impl<'lt, Ptr> MutOps for Val<'lt, $wrapper_name<Ptr>>
        where
            Val<'lt, Ptr>: MutOps,
            Ptr: Ty + 'lt,
        {
            fn store<Value>(&mut self, val: Value)
            where
                Value: Holds<T = Self::Pointee>,
            {
                unsafe { self.to_inner().store_unchecked(val) }
            }
        }
    };
}

impl_ptr_wrapper!(Global);
impl_ptr_wrapper!(Shared);
