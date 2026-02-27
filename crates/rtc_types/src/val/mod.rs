mod array;
mod cmp;
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
    values::{AnyValue, BasicValue, BasicValueEnum, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{M, R, SizedTy, Ty, ValTy},
};

/* CANNOT be Copy because eaech one owns an alloca */
pub struct Val<'ctx, T: ?Sized>(&'ctx FnCodegen, PointerValue<'static>, PhantomData<T>);

impl<'ctx, T: ?Sized> Val<'ctx, T>
where
    T: Ty + Copy,
{
    pub fn copy(&self) -> Self {
        // This is cheap here, so we provide a different name
        self.clone()
    }
}

impl<'ctx, T> Clone for Val<'ctx, T>
where
    T: Ty,
{
    fn clone(&self) -> Self {
        // This is safe and just ensures we don't just copy the underlying
        // pointers
        unsafe { Self::new_from_value(self.cx(), self.get_raw()) }
    }
}

impl<'ctx, T: ?Sized> Val<'ctx, T> {
    pub(crate) fn ctx(&self) -> ContextRef<'static> {
        self.0.ctx()
    }
    pub(crate) fn cx(&self) -> &'ctx FnCodegen {
        self.0
    }
    pub(crate) unsafe fn new_from_value(cx: &'ctx FnCodegen, val: BasicValueEnum<'static>) -> Self
    where
        T: Ty,
    {
        let ty = T::ty(cx.ctx());
        let alloca = unsafe {
            cx.with_builder(|b| b.build_alloca(ty, "new_value_alloca"))
                .expect("Alloca for stack values should succeed")
        };
        let _raw_store = unsafe { cx.with_builder(|b| b.build_store(alloca, val)) }
            .expect("store to alloca should work");
        Self(cx, alloca, PhantomData)
    }
    pub(crate) unsafe fn new(cx: &'ctx FnCodegen, val: PointerValue<'static>) -> Self {
        Self(cx, val, PhantomData)
    }
    /// # Safety: This is identical to ::std::mem::transmute.
    pub unsafe fn transmute<U: ?Sized>(val: Self) -> Val<'ctx, U>
    where
        T: SizedTy,
        U: for<'a> SizedTy<Type<'a> = T::Type<'a>, Value<'a> = T::Value<'a>>,
    {
        unsafe { Val::new(val.cx(), val.raw_ptr()) }
    }
    pub(crate) fn raw_ptr(&self) -> PointerValue<'static> {
        self.1
    }
    pub(crate) fn get_raw(&self) -> BasicValueEnum<'static>
    where
        T: Ty,
    {
        unsafe {
            self.cx()
                .with_builder(|b| b.build_load(T::ty(self.ctx()), self.raw_ptr(), "get_raw"))
        }
        .expect("Load should succeed")
        .as_basic_value_enum()
    }
    pub(crate) fn get_ll_typed(&self) -> T::Value<'static>
    where
        T: ValTy,
    {
        T::type_val(self.get_raw().as_any_value_enum())
    }

    pub fn as_ref<'a>(&'a self) -> Val<'ctx, R<&'a T>>
    where
        T: Ty,
    {
        let raw_ptr = self.raw_ptr();
        unsafe { Val::new_from_value(self.cx(), raw_ptr.as_basic_value_enum()) }
    }
    pub fn as_mut<'a>(&'a mut self) -> Val<'ctx, M<&'a mut T>>
    where
        T: Ty,
    {
        let raw_ptr = self.raw_ptr();
        unsafe { Val::new_from_value(self.cx(), raw_ptr.as_basic_value_enum()) }
    }
}
