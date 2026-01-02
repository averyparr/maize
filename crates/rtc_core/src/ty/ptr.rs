use std::marker::PhantomData;

use inkwell::{
    AddressSpace,
    attributes::{Attribute, AttributeLoc},
    context::ContextRef,
    types::PointerType,
    values::PointerValue,
};

use crate::{
    traits::ptr::{MutTy, PtrTy, RefTy},
    ty::{CodegenModule, FromCtx, Ty},
    val::Val,
};

#[expect(unused, reason = "Please remove once gone")]
enum PTXAddressSpaces {
    Generic = 0,
    Global = 1,
    Shared = 3,
    Constant = 4,
    Local = 5,
    Tensor = 6,
    Cluster = 7,
}

#[derive(Clone, Copy)]
pub struct P<T>(ContextRef<'static>, AddressSpace, PhantomData<*mut T>);
#[derive(Clone, Copy)]
pub struct R<T>(ContextRef<'static>, AddressSpace, PhantomData<T>);
pub struct M<T>(ContextRef<'static>, AddressSpace, PhantomData<T>);

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
            const ALIGN: usize = std::mem::align_of::<*mut ()>();

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
            Ptr: PtrTy,
        {
            pub fn to_inner(&'_ self) -> Val<'lt, Ptr> {
                // SAFETY: This is just a matter of decaying the name
                // of the type
                unsafe { Val::new(self.cm(), self.to_underlying()) }
            }
        }

        impl<'lt, Ptr> PtrTy for $name<Ptr>
        where
            Ptr: PtrTy,
        {
            type Pointee = Ptr::Pointee;
            fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self {
                let my_addrspace = AddressSpace::from(PTXAddressSpaces::$addrspace as u16);
                assert_eq!(addrspace, my_addrspace);
                Self::new(ctx)
            }
        }

        impl<'lt, Ptr> RefTy for $name<Ptr> where Ptr: RefTy {}
        impl<'lt, Ptr> MutTy for $name<Ptr> where Ptr: MutTy {}
    };
}

addrspace_ptr!(Global, Global);
addrspace_ptr!(Shared, Shared);

macro_rules! derive_ptr_type {
    ($name: ident$(: $mt: tt mut)?$(, $rf: tt)?) => {
        impl<T> PtrTy for $name<$($mt mut)?$($rf)?T> where T: Ty {
            type Pointee = T;
            fn new_in(ctx: ContextRef<'static>, addrspace: AddressSpace) -> Self {
                Self(ctx, addrspace, PhantomData)
            }
        }

        impl<T> FromCtx for $name<$($mt mut)?$($rf)?T> where T: Ty {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self::new_in(ctx, AddressSpace::default())
            }
        }

        impl<T> Ty for $name<$($mt mut)?$($rf)?T> where T: Ty {
            const ALIGN: usize = ::core::mem::align_of::<&()>();
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

            fn get_args_at_idx<'lt>(cm: &'lt CodegenModule<'static>, at_idx: u32) -> Val<'lt, Self> {
                let cx = cm.cx();
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
                // SAFETY: We just got the value from a newly created
                // function which should error if it passes the wrong value.
                unsafe {Val::new(cm, Self::get_value(val))}
            }
        }
    };
}

derive_ptr_type!(P);
derive_ptr_type!(R,&);
derive_ptr_type!(M:&mut);

impl<T> P<T>
where
    T: Ty,
{
    pub fn ref_ty(&self) -> R<&T> {
        R::new(self.ctx())
    }
    pub fn mut_ty(&self) -> M<&mut T> {
        M::new(self.ctx())
    }
}

impl<T> R<&T>
where
    T: Ty,
{
    pub fn ptr_ty(&self) -> P<T> {
        P::new(self.ctx())
    }
    pub fn mut_ty<'m>(&self) -> M<&mut T> {
        M::new(self.ctx())
    }
}

impl<T> M<&mut T>
where
    T: Ty,
{
    pub fn ref_ty<'r>(&self) -> R<&T> {
        R::new(self.ctx())
    }
    pub fn ptr_ty(&self) -> P<T> {
        P::new(self.ctx())
    }
}

impl<T> RefTy for R<&T> where T: Ty {}
impl<T> RefTy for M<&mut T> where T: Ty {}
impl<T> MutTy for M<&mut T> where T: Ty {}
