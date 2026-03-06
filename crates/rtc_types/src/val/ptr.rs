use inkwell::{types::IntType, values::BasicValue};

use crate::{
    ty::{ConstPtrTy, MutPtrTy, MutTy, RawPtrTy, RefTy, SizedTy, Ty, U8, U16, U32, U64, ValTy},
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

impl<'a, Ptr> Val<'a, Ptr>
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
    pub fn to_const_ptr(self) -> Val<'a, Ptr::PtrConst<Ptr::PointeeTy>> {
        Ptr::to_const_ptr(self)
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
    pub fn reborrow<'b>(&'b self) -> Val<'a, Ref::Ref<'b, Ref::PointeeTy>> {
        Ref::reborrow(self)
    }
    pub fn as_ptr(self) -> Val<'a, Ref::PtrConst<Ref::PointeeTy>> {
        Ref::as_ptr(self)
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
    pub fn reborrow_mut<'b>(&self) -> Val<'a, Mut::Mut<'b, Mut::PointeeTy>>
    where
        Mut: 'b,
        'a: 'b,
    {
        Mut::reborrow_mut(self)
    }

    pub fn as_mut_ptr(self) -> Val<'a, Mut::PtrMut<Mut::PointeeTy>> {
        Mut::as_mut_ptr(self)
    }
}

pub trait PtrAddableTy: for<'ctx> ValTy<Type<'ctx> = IntType<'ctx>> {
    unsafe fn ptr_add<'a, Ptr: ConstPtrTy>(ptr: Val<'a, Ptr>, val: Val<'a, Self>) -> Val<'a, Ptr>;
}

macro_rules! impl_ptr_addable_for {
    ($($tys: ty),*) => {
        $(
impl PtrAddableTy for $tys {
    unsafe fn ptr_add<'a, Ptr: ConstPtrTy>(ptr: Val<'a, Ptr>, val: Val<'a, Self>) -> Val<'a, Ptr> {
        let cx = ptr.cx();
        let raw_ptr = ptr.ll_typed();
        let pointee_ty = Ptr::PointeeTy::ty(ptr.ctx());
        let new_raw_ptr = unsafe {
            cx.with_builder(|b| {
                b.build_in_bounds_gep(
                    pointee_ty,
                    raw_ptr,
                    &[val.ll_typed()],
                    "gep_for_ptr_add",
                )
            })
        }
        .expect("Ptr add should work");
        unsafe { Val::new(cx, new_raw_ptr.as_basic_value_enum()) }
    }
}
        )*
    };
}

impl_ptr_addable_for!(U8, U16, U32, U64);

impl<'a, Ptr> Val<'a, Ptr>
where
    Ptr: RawPtrTy,
{
    pub unsafe fn add<Rhs>(self, rhs: Val<'a, Rhs>) -> Self
    where
        Rhs: PtrAddableTy,
    {
        unsafe { Rhs::ptr_add(self, rhs) }
    }
    pub fn ptr_cast<U>(self) -> Val<'a, Ptr::PtrConst<U>>
    where
        U: ValTy,
    {
        Ptr::ptr_cast(self)
    }
    pub fn ptr_cast_mut<U>(self) -> Val<'a, Ptr::PtrMut<U>>
    where
        U: ValTy,
        Ptr: MutPtrTy,
    {
        Ptr::ptr_cast_mut(self)
    }
    pub fn to_mut_ptr(self) -> Val<'a, Ptr::AsMutPtr> {
        Ptr::to_mut_ptr(self)
    }
}
