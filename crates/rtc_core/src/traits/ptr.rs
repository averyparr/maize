use inkwell::{
    AddressSpace,
    context::ContextRef,
    types::PointerType,
    values::{BasicValue, InstructionValue, PointerValue},
};

use crate::{
    traits::{HasCXVal, indexes::IndexableTy},
    ty::{
        FromCtx, Ty,
        ptr::{M, P, R},
    },
    val::Val,
};

pub trait PtrTy: Ty<Type = PointerType<'static>, Value = PointerValue<'static>> + Copy {
    type Pointee: Ty;
    fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self;

    /// # Safety
    /// You must ensure that `val`'s value is a pointer loadable as-if a *mut T
    unsafe fn load_ptr_unchecked<'a>(ptr: &Val<'a, Self>) -> Val<'a, Self::Pointee> {
        let pointee_ty = Self::Pointee::new(ptr.cx().ctx()).basic_ty();
        let cx = ptr.cx();
        let ptr_val = ptr.bval().into_pointer_value();
        let basic_val =
            unsafe { cx.with_builder(|b| b.build_load(pointee_ty, ptr_val, "load_val")) }
                .expect("Unable to generate ptr load");
        if let Some(ins) = basic_val.as_instruction_value() {
            ins.set_alignment(Self::Pointee::ALIGN as u32)
                .expect("Should be able to set alignment");
        }
        let pointed_val = Self::Pointee::get_value(basic_val);
        unsafe { Val::new(ptr.cm(), pointed_val) }
    }
    /// # Safety
    /// You must ensure that `val`'s value is a pointer storable as-if a *mut T
    unsafe fn store_ptr_unchecked<'a>(
        ptr: &Val<'a, Self>,
        to_store: Val<'a, Self::Pointee>,
    ) -> InstructionValue<'static> {
        let cx = ptr.cx();
        let ptr = ptr.bval().into_pointer_value();
        let ins = unsafe { cx.with_builder(|b| b.build_store(ptr, to_store.to_underlying())) }
            .expect("Unable to generate store");
        ins.set_alignment(Self::Pointee::ALIGN as u32)
            .expect("Should be able to set alignment");
        ins
    }
}

pub trait RefTy: PtrTy {
    fn as_ptr<'a>(ptr: &Val<'a, Self>) -> Val<'a, P<Self::Pointee>> {
        unsafe { Val::new(ptr.cm(), ptr.val().into_pointer_value()) }
    }
    fn load_value<'a>(ptr: &Val<'a, Self>) -> Val<'a, Self::Pointee> {
        // Safety: We hold a shared reference
        unsafe { P::load_ptr_unchecked(&Self::as_ptr(ptr)) }
    }
}

pub trait MutTy: RefTy {
    fn store_value<'a>(
        ptr: &mut Val<'a, Self>,
        to_store: Val<'a, Self::Pointee>,
    ) -> InstructionValue<'static> {
        // SAFETY: We hold an exclusive reference
        unsafe { P::store_ptr_unchecked(&Self::as_ptr(ptr), to_store) }
    }
}
