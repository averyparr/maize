use inkwell::values::AnyValue;

use crate::{
    ty::{P, PtrTy, ValTy},
    val::Val,
};

impl<'a, Ptr> Val<'a, Ptr>
where
    Ptr: PtrTy,
{
    // This is unsafe in the same way as reading through a *const T is
    pub unsafe fn load_unchecked(self) -> Val<'a, Ptr::PointeeTy> {
        unsafe { Ptr::load_unchecked(self) }
    }

    // This is unsafe in the same way as writing through a *mut T is
    pub unsafe fn store_unchecked(self, val: Val<'a, Ptr::PointeeTy>) {
        unsafe { Ptr::store_unchecked(self, val) }
    }
}
