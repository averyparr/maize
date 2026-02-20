use inkwell::values::AnyValue;

use crate::{
    ty::{AlignedTy, ConstPtrTy, MutPtrTy, MutTy, P, R, RefTy, SizedTy, ValTy},
    val::Val,
};

impl<'a, Ptr> Val<'a, Ptr>
where
    Ptr: ConstPtrTy,
{
    /// See `ConstPtrTy::read_unaligned`
    pub unsafe fn read_unaligned(self) -> Val<'a, Ptr::PointeeTy> {
        // See `ConstPtrTy::read_unaligned`
        unsafe { Ptr::read_unaligned(self) }
    }
    /// See `ConstPtrTy::read`
    pub unsafe fn read(self) -> Val<'a, Ptr::PointeeTy>
    where
        Ptr::PointeeTy: SizedTy,
    {
        // See `ConstPtrTy::read`
        unsafe { Ptr::read(self) }
    }
}

impl<Ptr> Val<'_, Ptr>
where
    Ptr: MutPtrTy,
{
    /// See `MutPtrTy::write_unaligned`   
    pub unsafe fn write_unaligned(self, val: Val<'_, Ptr::PointeeTy>) {
        // See `MutPtrTy::write_unaligned`
        unsafe { Ptr::write_unaligned(self, val) }
    }

    /// See `MutPtrTy::write`   
    pub unsafe fn write(self, val: Val<'_, Ptr::PointeeTy>)
    where
        Ptr::PointeeTy: SizedTy,
    {
        // See `MutPtrTy::write`
        unsafe { Ptr::write(self, val) }
    }
}

impl<'a, Ref> Val<'a, Ref>
where
    Ref: RefTy,
{
    pub fn load(&self) -> Val<'a, Ref::PointeeTy>
    where
        Ref::PointeeTy: SizedTy + Copy,
    {
        Ref::load(self)
    }
    pub fn as_ref<'b>(&'b self) -> Val<'a, R<&'b Ref::PointeeTy>> {
        Ref::reborrow(self)
    }
}

impl<'a, Mut> Val<'a, Mut>
where
    Mut: MutTy,
{
    pub fn swap(&mut self, val: Val<'a, Mut::PointeeTy>) -> Val<'a, Mut::PointeeTy>
    where
        Mut::PointeeTy: SizedTy,
    {
        Mut::swap(self, val)
    }
    pub fn store(&mut self, val: Val<'a, Mut::PointeeTy>)
    where
        Mut::PointeeTy: SizedTy,
    {
        Mut::store(self, val);
    }
}
