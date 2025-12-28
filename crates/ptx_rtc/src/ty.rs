use inkwell::{
    AddressSpace,
    builder::Builder,
    context::ContextRef,
    types::BasicType,
    values::{BasicValue, BasicValueEnum},
};

use crate::primitives::PtrT;

pub trait Ty<'ctx>: Sized {
    fn new(ctx: ContextRef<'ctx>) -> Self;
}

pub trait BasicTy<'ctx>: Ty<'ctx> {
    fn ctx(&self) -> ContextRef<'ctx>;
    type Value: BasicValue<'ctx>;
    fn basic_ty(&self) -> impl BasicType<'ctx>
    where
        Self: 'ctx;
    fn get_value(basic_val: BasicValueEnum<'ctx>) -> Self::Value;

    fn ptr_ty(&self) -> PtrT<'ctx, &Self> {
        PtrT::new(self.ctx())
    }
    fn ptr_ty_in(&self, addrspace: u16) -> PtrT<'ctx, &Self> {
        PtrT::in_addrspace(self.ctx(), addrspace)
    }
    fn ptr_mut_ty(&self) -> PtrT<'ctx, &mut Self> {
        PtrT::new(self.ctx())
    }
    fn ptr_mut_ty_in(&self, addrspace: u16) -> PtrT<'ctx, &mut Self> {
        PtrT::in_addrspace(self.ctx(), addrspace)
    }
}

pub trait AddableTy<'ctx>: BasicTy<'ctx> {
    fn emit_add(builder: Builder<'ctx>, lhs: Self::Value, rhs: Self::Value) -> Self::Value;
}
