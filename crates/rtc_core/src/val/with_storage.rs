use std::marker::PhantomData;

use inkwell::values::{BasicValue, BasicValueEnum};

use crate::{
    codegen::FnCodegen,
    ty::{M, R, Ty},
    val::Holds,
};

use super::{S, Val};

impl<'lt, T> Val<'lt, S<T>>
where
    T: Ty,
{
    pub fn new_with_storage(cx: &'lt FnCodegen<'static>, val: BasicValueEnum<'static>) -> Self {
        let ptr = cx.build_alloca(val);
        let ret = unsafe {
            cx.with_builder(|b| b.build_store(ptr, val))
                .expect("Unable to build store")
        };
        Val {
            cx,
            val: ptr.as_basic_value_enum(),
            phantom: PhantomData,
        }
    }

    pub fn get(&self) -> Val<'lt, T> {
        let ptr = self.val.into_pointer_value();
        let pointee_ty = self.to_underlying_ty();
        // SAFETY: This alloca only ever stored our own value
        let val = unsafe { self.cx().load(pointee_ty, ptr, Some(T::ALIGN), None) };
        if let Some(ins) = val.as_instruction_value() {
            ins.set_alignment(T::ALIGN)
                .expect("Unable to set alignment");
        }
        Val::new(self.cx(), val)
    }

    pub fn get_ref<'r>(&'r self) -> Val<'lt, R<'r, T>>
    where
        T: Ty,
    {
        Val::new(self.cx(), self.val)
    }

    pub fn get_mut<'m>(&'m mut self) -> Val<'lt, M<'m, T>>
    where
        T: Ty,
    {
        Val::new(self.cx(), self.val)
    }
}

impl<'lt, T> Holds for Val<'lt, S<T>>
where
    T: Ty,
{
    type T = T;
    fn to_underlying(&self) -> T::Value {
        self.get().to_underlying()
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

impl<'lt, T> Clone for Val<'lt, S<T>>
where
    T: Clone + Ty,
{
    fn clone(&self) -> Self {
        self.get().with_storage()
    }
}
