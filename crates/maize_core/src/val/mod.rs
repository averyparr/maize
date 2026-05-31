use std::marker::PhantomData;

use crate::{
    backend::{FnRef, UntypedValue},
    tipe::Ty,
};

#[derive(Clone)]
pub struct Val<T>(FnRef, UntypedValue, PhantomData<T>);

#[derive(Clone)]
pub struct S<T>(PhantomData<T>);

impl<T> Val<T> {
    pub fn decompose(self) -> (FnRef, UntypedValue) {
        (self.0, self.1)
    }
    pub fn copy(&self) -> Self
    where
        T: Copy,
    {
        Self(self.0.clone(), self.1, PhantomData)
    }
    pub unsafe fn new(fn_ref: FnRef, mut value: UntypedValue) -> Self
    where
        T: Ty,
    {
        assert_eq!(T::ty(&fn_ref), value.erased_type());
        T::type_metadata(&fn_ref, &mut value);
        unsafe { Self::new_untyped(fn_ref, value) }
    }
    pub unsafe fn new_untyped(fn_ref: FnRef, mut value: UntypedValue) -> Self {
        fn_ref.apply_ins_flags(&mut value);
        Self(fn_ref, value, PhantomData)
    }
    pub(crate) fn fn_ref(&self) -> &FnRef {
        &self.0
    }
    pub(crate) fn raw(&self) -> UntypedValue {
        self.1
    }
    pub(crate) fn typed(&self) -> T::LLVal
    where
        T: Ty,
    {
        T::type_val(self.raw())
    }
    pub fn with_storage(self) -> Val<S<T>>
    where
        T: Ty,
    {
        let mut ret = self.fn_ref().alloca();
        // Safety: We're moving Self, so no need to worry about drops
        unsafe { ret.get_mut().as_mut_ptr().write(self) };
        ret
    }
}

impl<T> Val<S<T>> {
    pub fn get(self) -> Val<T>
    where
        T: Ty,
    {
        unsafe { self.get_ref().as_ptr().read() }
    }
    pub fn get_ref(&self) -> Val<&T>
    where
        T: Ty,
    {
        unsafe { Val::new(self.fn_ref().clone(), self.raw()) }
    }
    pub fn get_mut(&mut self) -> Val<&mut T>
    where
        T: Ty,
    {
        unsafe { Val::new(self.fn_ref().clone(), self.raw()) }
    }
    pub fn store(&mut self, val: Val<T>)
    where
        T: Ty + Copy,
    {
        self.get_mut().store(val)
    }
    pub fn load(&self) -> Val<T>
    where
        T: Ty + Copy,
    {
        self.get_ref().load()
    }
}
