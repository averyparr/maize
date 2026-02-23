mod ptr;
mod std_ops;

use std::marker::PhantomData;

use inkwell::{
    context::ContextRef,
    types::AnyType,
    values::{AnyValue, AnyValueEnum, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{M, R, SizedTy, Ty, ValTy},
};

#[derive(Clone, Copy)]
pub struct Val<'ctx, T: ?Sized>(&'ctx FnCodegen, AnyValueEnum<'static>, PhantomData<T>);

impl<'ctx, T: ?Sized> Val<'ctx, T> {
    pub(crate) fn ctx(&self) -> ContextRef<'static> {
        self.0.ctx()
    }
    pub(crate) fn cx(&self) -> &'ctx FnCodegen {
        self.0
    }
    pub(crate) unsafe fn new(cx: &'ctx FnCodegen, val: AnyValueEnum<'static>) -> Self {
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
    pub(crate) fn raw(&self) -> AnyValueEnum<'static> {
        self.1
    }
    pub(crate) fn ll_typed(&self) -> T::Value<'static>
    where
        T: ValTy,
    {
        T::type_val(self.raw())
    }
    pub fn with_storage(self) -> Val<'ctx, S<T>>
    where
        T: SizedTy + Sized,
    {
        let ty = T::ty(self.ctx());
        let alloca = unsafe {
            self.cx()
                .with_builder(|b| b.build_alloca(ty, "with_storage_alloca"))
        }
        .expect("Should be able to build alloca for type");
        let res = unsafe {
            self.cx()
                .with_builder(|b| b.build_store(alloca, self.ll_typed()))
        }
        .expect("Store should work...");
        unsafe { Val::new(self.cx(), alloca.as_any_value_enum()) }
    }
}

pub struct S<T: ?Sized>(PhantomData<T>);

impl<'ctx, T: ?Sized> Val<'ctx, S<T>> {
    pub(crate) fn get(self) -> Val<'ctx, T>
    where
        T: Ty,
    {
        let pointee_ty = T::ty(self.ctx());
        let val_at_ptr = unsafe {
            self.cx()
                .with_builder(|b| b.build_load(pointee_ty, self.storage(), "load_for_stored"))
        }
        .expect("Pointer load should succeed");
        unsafe { Val::new(self.cx(), val_at_ptr.as_any_value_enum()) }
    }
    pub(crate) fn storage(&self) -> PointerValue<'static> {
        self.1.into_pointer_value()
    }
    pub fn as_ref<'a>(&'a self) -> Val<'ctx, R<&'a T>> {
        let raw_ptr = self.storage();
        unsafe { Val::new(self.cx(), raw_ptr.as_any_value_enum()) }
    }
    pub fn as_mut<'a>(&'a mut self) -> Val<'ctx, M<&'a mut T>> {
        let raw_ptr = self.storage();
        unsafe { Val::new(self.cx(), raw_ptr.as_any_value_enum()) }
    }
}
