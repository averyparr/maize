mod array;
mod assert;
mod cmp;
mod const_val;
mod no_overflow;
mod ptr;
mod std_ops;
mod vec;

pub(crate) fn post_process<'a>(
    cx: &FnCodegen,
    pre_store_value: BasicValueEnum<'a>,
) -> BasicValueEnum<'a> {
    if let Some(ins) = pre_store_value.as_instruction_value() {
        cx.apply_ins_opt(ins);
    }
    pre_store_value
}

use std::marker::PhantomData;

use inkwell::{
    context::ContextRef,
    types::{BasicType, BasicTypeEnum},
    values::{AnyValue, BasicValue, BasicValueEnum, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{M, R, SizedTy, Ty, ValTy},
};

#[derive(Clone, Copy)]
pub struct Val<'ctx, T: ?Sized>(&'ctx FnCodegen, BasicValueEnum<'static>, PhantomData<T>);
/* CANNOT be Copy because eaech one owns an alloca */
pub struct S<T: ?Sized>(PhantomData<T>);

impl<T> Clone for Val<'_, S<T>>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        todo!();
    }
}

pub mod __structreflect {
    use super::*;
    pub fn _ctx<T>(val: &Val<'_, T>) -> ContextRef<'static> {
        val.ctx()
    }
    pub fn _lltyped<T: ValTy>(val: &Val<'_, T>) -> T::Value<'static> {
        val.ll_typed()
    }
    pub unsafe fn _new<'a, T: ValTy>(
        cx: &'a FnCodegen,
        val: BasicValueEnum<'static>,
    ) -> Val<'a, T> {
        unsafe { Val::new(cx, val) }
    }
    pub fn _raw<T>(val: &Val<'_, T>) -> BasicValueEnum<'static> {
        val.raw()
    }
}

impl<'ctx, T: ?Sized> Val<'ctx, T> {
    pub(crate) fn ctx(&self) -> ContextRef<'static> {
        self.0.ctx()
    }
    pub fn cx(&self) -> &'ctx FnCodegen {
        self.0
    }
    pub fn get_type(&self) -> BasicTypeEnum<'static>
    where
        T: Ty,
    {
        T::ty(self.ctx()).as_basic_type_enum()
    }
    pub(crate) unsafe fn new(cx: &'ctx FnCodegen, val: BasicValueEnum<'static>) -> Self {
        Self(cx, val, PhantomData)
    }
    pub fn zero(&self) -> Self
    where
        T: ValTy,
    {
        Self::zeros(self.cx())
    }
    pub fn zeros(cx: &'ctx FnCodegen) -> Self
    where
        T: ValTy,
    {
        Self(cx, T::zeros(cx.ctx()).as_basic_value_enum(), PhantomData)
    }
    pub unsafe fn new_undef(cx: &'ctx FnCodegen) -> Self
    where
        T: ValTy,
    {
        Self(cx, T::undef(cx.ctx()).as_basic_value_enum(), PhantomData)
    }

    /// # Safety: This is identical to ::std::mem::transmute.
    pub unsafe fn transmute<U: ?Sized>(val: Self) -> Val<'ctx, U>
    where
        T: SizedTy,
        U: SizedTy,
    {
        unsafe { Val::new(val.cx(), val.raw()) }
    }
    pub(crate) fn raw(&self) -> BasicValueEnum<'static> {
        self.1
    }
    pub(crate) fn ll_typed(&self) -> T::Value<'static>
    where
        T: ValTy,
    {
        T::type_val(self.raw().as_any_value_enum())
    }

    pub fn with_storage(self) -> Val<'ctx, S<T>>
    where
        T: Ty,
    {
        let alloca = self.cx().store_in_alloca(self.raw());
        let raw_ptr = alloca.raw();
        Val(self.cx(), raw_ptr, PhantomData)
    }
}

impl<'ctx, T: Ty> Val<'ctx, S<T>> {
    pub fn as_ref<'a>(&'a self) -> Val<'ctx, R<&'a T>> {
        Val(self.cx(), self.raw(), PhantomData)
    }
    pub fn as_mut<'a>(&'a mut self) -> Val<'ctx, M<&'a mut T>> {
        Val(self.cx(), self.raw(), PhantomData)
    }
    pub fn alloca_ptr(&self) -> PointerValue<'static> {
        self.1.into_pointer_value()
    }
    pub fn get(self) -> Val<'ctx, T> {
        let raw_ptr = self.raw().into_pointer_value();
        let val = unsafe {
            self.cx()
                .with_builder(|b| b.build_load(T::ty(self.ctx()), raw_ptr, "get_from_alloca"))
        }
        .expect("Load should succeed");
        Val(self.cx(), val, PhantomData)
    }
}

pub trait OwnsValue {
    type Val: ValTy;
    fn as_stores<'a>(self) -> Val<'a, S<Self::Val>>
    where
        Self: 'a;
}

impl<'ctx, T: ValTy> OwnsValue for Val<'ctx, T> {
    type Val = T;
    fn as_stores<'a>(self) -> Val<'a, S<Self::Val>>
    where
        Self: 'a,
    {
        self.with_storage()
    }
}
