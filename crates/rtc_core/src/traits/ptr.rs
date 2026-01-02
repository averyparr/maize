use inkwell::{
    AddressSpace,
    context::ContextRef,
    values::{BasicValue, InstructionValue},
};

use crate::{
    traits::HasCXVal,
    ty::{FromCtx, Ty},
};

pub trait PtrTy: Ty {
    type Pointee: Ty;
    fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self;

    /// # Safety
    /// You must ensure that `val`'s value is a pointer loadable as-if a *mut T
    unsafe fn load_ptr_unchecked(ptr: impl HasCXVal) -> <Self::Pointee as Ty>::Value {
        let pointee_ty = Self::Pointee::new(ptr.cx().ctx()).basic_ty();
        let cx = ptr.cx();
        let ptr = ptr.bval().into_pointer_value();
        let basic_val = unsafe { cx.with_builder(|b| b.build_load(pointee_ty, ptr, "load_val")) }
            .expect("Unable to generate ptr load");
        if let Some(ins) = basic_val.as_instruction_value() {
            ins.set_alignment(Self::Pointee::ALIGN as u32)
                .expect("Should be able to set alignment");
        }
        Self::Pointee::get_value(basic_val)
    }
    /// # Safety
    /// You must ensure that `val`'s value is a pointer storable as-if a *mut T
    unsafe fn store_ptr_unchecked(
        ptr: impl HasCXVal,
        to_store: <Self::Pointee as Ty>::Value,
    ) -> InstructionValue<'static> {
        let cx = ptr.cx();
        let ptr = ptr.bval().into_pointer_value();
        let ins = unsafe { cx.with_builder(|b| b.build_store(ptr, to_store)) }
            .expect("Unable to generate store");
        ins.set_alignment(Self::Pointee::ALIGN as u32)
            .expect("Should be able to set alignment");
        ins
    }
}

pub trait RefTy: PtrTy {
    /// # Safety:
    /// You must guarantee that this CXVal contains a valid shared reference
    unsafe fn load_ptr(ptr: impl HasCXVal) -> <Self::Pointee as Ty>::Value {
        // Safety: User promised!
        unsafe { Self::load_ptr_unchecked(ptr) }
    }
}

pub trait MutTy: PtrTy {
    /// # Safety:
    /// You must guarante that this CXVal contains a valid exclusive reference
    unsafe fn store(
        ptr: impl HasCXVal,
        to_store: <Self::Pointee as Ty>::Value,
    ) -> InstructionValue<'static> {
        // Safety: User promised!
        unsafe { Self::store_ptr_unchecked(ptr, to_store) }
    }
}
