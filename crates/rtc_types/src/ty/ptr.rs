use inkwell::{
    AddressSpace,
    context::ContextRef,
    types::PointerType,
    values::{AnyValue, AnyValueEnum, PointerValue},
};

use crate::{ty::AnyTy, val::Val};

use super::{M, P, R, Ty, ValTy, sized::SizedTy};

macro_rules! body {
    (ty) => {
        type AnyType<'ctx> = PointerType<'ctx>;
        fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
            ctx.ptr_type(AddressSpace::default())
        }
    };
    (val_ty) => {
        type Value<'ctx> = PointerValue<'ctx>;

        fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
            ctx.ptr_type(AddressSpace::default()).get_undef()
        }

        fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
            ctx.ptr_type(AddressSpace::default()).const_null()
        }

        fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
            if let AnyValueEnum::PointerValue(val) = val {
                Some(val)
            } else {
                None
            }
        }
    };
}

impl<T> AnyTy for P<T>
where
    T: AnyTy,
{
    body!(ty);
}

impl<'a, T> AnyTy for R<&'a T>
where
    T: Ty,
{
    body!(ty);
}

impl<'a, T> AnyTy for M<&'a mut T>
where
    T: Ty,
{
    body!(ty);
}

impl<T> ValTy for P<T>
where
    T: AnyTy,
{
    body!(val_ty);
}

impl<'a, T> ValTy for R<&'a T>
where
    T: Ty,
{
    body!(val_ty);
}

impl<'a, T> ValTy for M<&'a mut T>
where
    T: Ty,
{
    body!(val_ty);
}

pub trait PtrTy: ValTy {
    type PointeeTy: ValTy;
    fn instance_in_addrspace<'ctx>(
        ctx: ContextRef<'ctx>,
        address_space: AddressSpace,
    ) -> Self::Type<'ctx>;

    unsafe fn load_unchecked<'ctx>(ptr: Val<'_, Self>) -> Val<'_, Self::PointeeTy>
    where
        Self::PointeeTy: ValTy;

    unsafe fn store_unchecked<'ctx>(ptr: Val<'_, Self>, val: Val<'_, Self::PointeeTy>)
    where
        Self::PointeeTy: ValTy;
}

pub trait RefTy: PtrTy {
    fn load(ptr: Val<'_, Self>) -> Val<'_, Self::PointeeTy>;
    fn as_ptr(ptr: Val<'_, Self>) -> Val<'_, P<Self::PointeeTy>>;
}

pub trait MutTy: RefTy {
    fn store(ptr: Val<'_, Self>, val: Val<'_, Self::PointeeTy>);
    fn as_ref<'value, 'reference>(
        ptr: &'reference Val<'value, Self>,
    ) -> Val<'reference, R<&'reference Self::PointeeTy>>;
}

impl<T> PtrTy for P<T>
where
    T: ValTy,
{
    type PointeeTy = T;
    fn instance_in_addrspace<'ctx>(
        ctx: ContextRef<'ctx>,
        address_space: AddressSpace,
    ) -> Self::Type<'ctx> {
        ctx.ptr_type(address_space)
    }

    unsafe fn load_unchecked<'ctx>(ptr: Val<'_, Self>) -> Val<'_, Self::PointeeTy>
    where
        Self::PointeeTy: ValTy,
    {
        unsafe {
            let new_val = ptr
                .cx()
                .with_builder(|b| b.build_load(T::ty(ptr.ctx()), ptr.ll_typed(), "load"))
                .expect("Pointer load shuould succeed");

            Val::new(ptr.cx(), new_val.as_any_value_enum())
        }
    }

    unsafe fn store_unchecked<'ctx>(ptr: Val<'_, Self>, val: Val<'_, Self::PointeeTy>)
    where
        Self::PointeeTy: ValTy,
    {
        unsafe {
            let new_val = ptr
                .cx()
                .with_builder(|b| b.build_store(ptr.ll_typed(), val.ll_typed()))
                .expect("Pointer store shuould succeed");
        }
    }
}

impl<'a, T> PtrTy for R<&'a T> where T: ValTy {

}

impl<'a, T> RefTy for R<'a T> where T: ValTy {
    fn as_ptr(ptr: Val<'_, Self>) -> Val<'_, P<Self::PointeeTy>> {
        unsafe { Val::new(ptr.cx(), ptr.raw()) }
    }
}
