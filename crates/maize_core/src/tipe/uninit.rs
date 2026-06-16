use std::mem::MaybeUninit;

use crate::{
    tipe::{A, Ty},
    val::Val,
};

impl<T: Ty> Ty for MaybeUninit<T> {
    type LLType = T::LLType;

    type LLVal = T::LLVal;

    fn raw_ty(ctx: crate::ContextRef) -> Self::LLType {
        T::raw_ty(ctx)
    }

    fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
        T::type_val(val)
    }

    fn const_val(self, fn_ref: crate::backend::FnRef) -> crate::val::Val<Self>
    where
        Self: Sized,
    {
        panic!("You must initialize a MaybeUninit before creating a constant")
    }
}

impl<'a, T: Ty> Val<&'a mut MaybeUninit<T>> {
    pub fn write(mut self, val: Val<T>) -> Val<&'a mut T> {
        unsafe { self.reborrow().as_mut_ptr().ptr_cast().write(val) };
        let (fn_ref, value) = self.decompose();
        unsafe { Val::new(fn_ref, value) }
    }
}

impl<'a, T: Ty, const ADDRSPACE: u16> Val<A<&'a mut MaybeUninit<T>, ADDRSPACE>> {
    pub fn write(mut self, val: Val<T>) -> Val<A<&'a mut T, ADDRSPACE>> {
        unsafe { self.reborrow().as_mut_ptr().ptr_cast().write(val) };
        let (fn_ref, value) = self.decompose();
        unsafe { Val::new(fn_ref, value) }
    }
}
