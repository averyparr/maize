use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use inkwell::{
    AddressSpace,
    attributes::{Attribute, AttributeLoc},
    context::ContextRef,
    types::PointerType,
    values::{BasicValue, PointerValue},
};

use crate::{
    ty::{FnCodegen, FromCtx, Ty},
    val::{Holds, Val},
};

enum PTXAddressSpaces {
    Generic = 0,
    Global = 1,
    Shared = 3,
    Constant = 4,
    Local = 5,
    Tensor = 6,
    Cluster = 7,
}

pub trait PtrTy: Ty {
    fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self;
}

#[derive(Clone, Copy)]
pub struct P<T>(ContextRef<'static>, AddressSpace, PhantomData<*mut T>);
#[derive(Clone, Copy)]
pub struct R<'r, T>(ContextRef<'static>, AddressSpace, PhantomData<&'r T>);
pub struct M<'m, T>(ContextRef<'static>, AddressSpace, PhantomData<&'m mut T>);

#[derive(Clone, Copy)]
pub struct Global<Ptr>(Ptr);
#[derive(Clone, Copy)]
pub struct Shared<Ptr>(Ptr);

macro_rules! addrspace_ptr {
    ($name: ident, $addrspace: ident) => {
        impl<Ptr> FromCtx for $name<Ptr>
        where
            Ptr: PtrTy,
        {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self(Ptr::new_in(
                    ctx,
                    AddressSpace::from(PTXAddressSpaces::$addrspace as u16),
                ))
            }
        }

        impl<Ptr> Ty for $name<Ptr>
        where
            Ptr: PtrTy,
        {
            const SIZE: usize = std::mem::size_of::<*mut ()>();
            const ALIGN: u32 = std::mem::align_of::<*mut ()>() as _;

            fn ctx(&self) -> ContextRef<'static> {
                self.0.ctx()
            }
            type Type = Ptr::Type;
            fn basic_ty(&self) -> Self::Type {
                self.0.basic_ty()
            }
            type Value = Ptr::Value;
            fn get_value(basic_val: inkwell::values::BasicValueEnum<'static>) -> Self::Value {
                Ptr::get_value(basic_val)
            }
        }

        impl<'lt, Ptr> Val<'lt, $name<Ptr>>
        where
            Ptr: Ty,
        {
            pub fn to_inner(&'_ self) -> Val<'lt, Ptr> {
                Val::new(self.cx(), self.get_val())
            }
        }
    };
}

addrspace_ptr!(Global, Global);
addrspace_ptr!(Shared, Shared);

macro_rules! derive_ptr_type {
    ($name: ident$(, $lt: tt)?) => {
        impl<$($lt,)?T> PtrTy for $name<$($lt,)?T> where T: Ty {
            fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self {
                Self(ctx, addrspace, PhantomData)
            }
        }

        impl<$($lt,)?T> FromCtx for $name<$($lt,)?T> where T: Ty {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self::new_in(ctx, AddressSpace::default())
            }
        }

        impl<$($lt,)?T> Ty for $name<$($lt,)?T> where T: Ty {
            const ALIGN: u32 = ::core::mem::align_of::<&()>() as _;
            const SIZE: usize = ::core::mem::size_of::<&()>();

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

            fn get_args_at_idx<'lt>(cx: &'lt FnCodegen<'static>, at_idx: u32) -> Val<'lt, Self> {
                {
                    let align_kind_id = Attribute::get_named_enum_kind_id("align");
                    let align_attr = cx
                        .ctx()
                        .create_enum_attribute(align_kind_id, T::ALIGN as _);
                    cx.func()
                        .add_attribute(AttributeLoc::Param(at_idx), align_attr);
                }
                {
                    let deref_kind_id = Attribute::get_named_enum_kind_id("dereferenceable");
                    let deref_attr = cx.ctx().create_enum_attribute(deref_kind_id, T::SIZE as _);
                    cx.func().add_attribute(AttributeLoc::Param(at_idx), deref_attr);
                }

                let val = cx
                    .func()
                    .get_nth_param(at_idx)
                    .expect("Param number mismatch!");
                Val::new(cx, val)
            }
        }
    };
}

derive_ptr_type!(P);
derive_ptr_type!(R, 'r);
derive_ptr_type!(M, 'm);

impl<T> P<T>
where
    T: Ty,
{
    pub fn ref_ty<'r>(&self) -> R<'r, T> {
        R::new(self.ctx())
    }
    pub fn mut_ty<'m>(&self) -> M<'m, T> {
        M::new(self.ctx())
    }
}

impl<'r, T> R<'r, T>
where
    T: Ty,
{
    pub fn ptr_ty(&self) -> P<T> {
        P::new(self.ctx())
    }
    pub fn mut_ty<'m>(&self) -> M<'m, T> {
        M::new(self.ctx())
    }
}

impl<'m, T> M<'m, T>
where
    T: Ty,
{
    pub fn ref_ty<'r>(&self) -> R<'r, T> {
        R::new(self.ctx())
    }
    pub fn ptr_ty(&self) -> P<T> {
        P::new(self.ctx())
    }
}
