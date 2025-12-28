use std::marker::PhantomData;

use inkwell::{AddressSpace, context::ContextRef, types::PointerType, values::PointerValue};

use crate::ty::{FromCtx, Ty};

#[derive(Clone, Copy)]
pub struct P<T>(ContextRef<'static>, AddressSpace, PhantomData<*mut T>);
#[derive(Clone, Copy)]
pub struct R<'r, T>(ContextRef<'static>, AddressSpace, PhantomData<&'r T>);
pub struct M<'m, T>(ContextRef<'static>, AddressSpace, PhantomData<&'m mut T>);

macro_rules! derive_ptr_type {
    ($name: ident$(, $lt: tt)?) => {
        impl<$($lt,)?T> $name<$($lt,)?T> {
            fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self {
                Self(ctx, addrspace, PhantomData)
            }
        }

        impl<$($lt,)?T> FromCtx for $name<$($lt,)?T> {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self(ctx, AddressSpace::default(), PhantomData)
            }
        }

        impl<$($lt,)?T> Ty for $name<$($lt,)?T> {
            fn ctx(&self) -> ContextRef<'static> {
                self.0
            }
            type Type = PointerType<'static>;
            fn basic_ty(&self) -> Self::Type {
                self.ctx().ptr_type(self.1)
            }
            type Value = PointerValue<'static>;
            fn get_value(basic_val: inkwell::values::BasicValueEnum<'static>) -> Self::Value {
                basic_val.into_pointer_value()
            }
        }
    };
}

derive_ptr_type!(P);
derive_ptr_type!(R, 'r);
derive_ptr_type!(M, 'm);

impl<T> P<T> {
    pub fn ref_ty<'r>(&self) -> R<'r, T> {
        R::new(self.ctx())
    }
    pub fn mut_ty<'m>(&self) -> M<'m, T> {
        M::new(self.ctx())
    }
}

impl<'r, T> R<'r, T> {
    pub fn ptr_ty(&self) -> P<T> {
        P::new(self.ctx())
    }
    pub fn mut_ty<'m>(&self) -> M<'m, T> {
        M::new(self.ctx())
    }
}

impl<'m, T> M<'m, T> {
    pub fn ref_ty<'r>(&self) -> R<'r, T> {
        R::new(self.ctx())
    }
    pub fn ptr_ty(&self) -> P<T> {
        P::new(self.ctx())
    }
}
