use inkwell::{
    types::IntType,
    values::{BasicValue, IntValue},
};

use crate::{
    ty::{PtrTy, SizedTy, Ty, U8, U16, U32, U64, ValTy},
    val::Val,
};

pub trait PtrAddableTy:
    for<'ctx> ValTy<Type<'ctx> = IntType<'ctx>, Value<'ctx> = IntValue<'ctx>>
{
    unsafe fn ptr_add<'a, Ptr: PtrTy>(ptr: Val<'a, Ptr>, val: Val<'a, Self>) -> Val<'a, Ptr>
    where
        Ptr::Pointee: SizedTy,
    {
        let cx = ptr.cx();
        let raw_ptr = ptr.ll_typed();
        let pointee_ty = Ptr::Pointee::ty(cx.ctx());
        let new_raw_ptr = unsafe {
            cx.with_builder(|b| {
                b.build_in_bounds_gep(pointee_ty, raw_ptr, &[val.ll_typed()], "ptr_gep")
            })
        }
        .expect("Pointer GEP should have succeeded");
        unsafe { Val::new(cx, new_raw_ptr.as_basic_value_enum()) }
    }
}

impl PtrAddableTy for U8 {}
impl PtrAddableTy for U16 {}
impl PtrAddableTy for U32 {}
impl PtrAddableTy for U64 {}

impl<'a, Ptr> Val<'a, Ptr>
where
    Ptr: PtrTy,
    Ptr::Pointee: SizedTy,
{
    pub unsafe fn add<Rhs>(self, rhs: Val<'a, Rhs>) -> Self
    where
        Rhs: PtrAddableTy,
    {
        unsafe { Rhs::ptr_add(self, rhs) }
    }
}
