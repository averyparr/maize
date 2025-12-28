use std::marker::PhantomData;

use inkwell::{context::ContextRef, values::PointerValue};

use crate::{codegen::Codegen, primitives::PtrT, ty::BasicTy, val::Val};

impl<'ctx, T> Val<'ctx, PtrT<'ctx, &T>>
where
    T: BasicTy<'ctx> + 'ctx,
{
    pub fn load<'t>(&self) -> Val<'ctx, T> {
        let binding = T::new(self.cx.ctx());
        let basic_ty = binding.basic_ty();
        Val {
            cx: self.cx,
            val: self.cx.load(basic_ty, self.val.into_pointer_value()),
            phantom: PhantomData,
        }
    }
}

impl<'ctx, T> Val<'ctx, PtrT<'ctx, &mut T>>
where
    T: BasicTy<'ctx> + 'ctx,
{
    pub fn to_ref(&self) -> Val<'ctx, PtrT<'ctx, &T>> {
        Val {
            cx: self.cx,
            val: self.val,
            phantom: PhantomData,
        }
    }
    pub fn load<'t>(&'t self) -> Val<'ctx, T> {
        let v = self.to_ref();
        v.load()
    }
    pub fn store(&self, val: Val<'ctx, T>) {
        let _ins = self.cx.store(self.to_value(), val.val);
    }
}
