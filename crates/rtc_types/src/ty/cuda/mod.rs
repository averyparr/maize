use std::marker::PhantomData;

use crate::codegen::FnCodegen;
use crate::ty::{
    AddrspacePtr, AlignedTy, AnyTy, ConstPtrTy, MutPtrTy, MutTy, RawPtrTy, RefTy, SizedTy, Ty,
    ValTy,
};
use inkwell::{
    AddressSpace,
    context::ContextRef,
    values::{AnyValueEnum, BasicMetadataValueEnum},
};

macro_rules! addrspace_ptrs {
    ($($ptr: ident => $addrspace: literal;)*) => {
    $(
        #[allow(unused)]
        pub struct $ptr<Ptr>(PhantomData<Ptr>);
        impl<Ptr> AddrspacePtr for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            type Ref<'r, PT: ValTy + ?Sized> = $ptr<Ptr::Ref<'r,PT>>
            where
                Ptr: RefTy + 'r,
                PT: 'r;
            type Mut<'r, PT: ValTy + ?Sized> = $ptr<Ptr::Mut<'r, PT>>
            where
                Ptr: MutTy + 'r,
                PT: 'r;
            type Inner = Ptr;
            const ADDRSPACE: u16 = $addrspace;
        }
        impl<Ptr> AnyTy for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            type AnyType<'ctx> = Ptr::Type<'ctx>;
            fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
                ctx.ptr_type(AddressSpace::from(Self::ADDRSPACE))
            }
        }

        impl<Ptr> ValTy for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            type Value<'ctx> = Ptr::Value<'ctx>;

            fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
                Self::ty(ctx).get_undef()
            }

            fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
                Self::ty(ctx).const_null()
            }

            fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
                Ptr::try_type_val(val)
            }
        }

        impl<Ptr> AlignedTy for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            const ALIGN: u32 = Ptr::ALIGN;
        }

        impl<Ptr> SizedTy for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            const SIZE: u32 = Ptr::SIZE;
            fn fn_arg_attrs(
                ctx: ContextRef<'_>,
            ) -> impl IntoIterator<Item = inkwell::attributes::Attribute> {
                Ptr::fn_arg_attrs(ctx)
            }
        }

        unsafe impl<Ptr> ConstPtrTy for $ptr<Ptr>
        where
            Ptr: ConstPtrTy,
        {
            type PtrConst<PT: ValTy + ?Sized> = $ptr<Ptr::PtrConst<PT>>;
            type PointeeTy = Ptr::PointeeTy;
        }

        unsafe impl<Ptr> MutPtrTy for $ptr<Ptr>
        where
            Ptr: MutPtrTy,
        {
            type PtrMut<PT: ValTy + ?Sized> = $ptr<Ptr::PtrMut<PT>>;
        }

        unsafe impl<Ptr> RefTy for $ptr<Ptr>
        where
            Ptr: RefTy,
        {
            type Ref<'r, PT: ValTy + ?Sized>
                = $ptr<Ptr::Ref<'r, PT>>
            where
                Self: 'r,
                PT: 'r;
            fn ptr_attrs(
                cx: &FnCodegen,
            ) -> impl IntoIterator<Item = (&str, Option<BasicMetadataValueEnum<'_>>)>
            where
                Self::PointeeTy: SizedTy,
            {
                Ptr::ptr_attrs(cx)
            }
        }

        unsafe impl<Ptr> MutTy for $ptr<Ptr>
        where
            Ptr: MutTy,
        {
            type Mut<'r, PT: ValTy + ?Sized>
                = $ptr<Ptr::Mut<'r, PT>>
            where
                Self: 'r,
                PT: 'r;
        }

        unsafe impl<Ptr> RawPtrTy for $ptr<Ptr>
        where
            Ptr: RawPtrTy,
        {
        }
    )*
    };
}

addrspace_ptrs!(
    Global => 1;
    Shared => 3;
    Constant => 4;
    Local => 5;
    Tensor => 6;
    Cluster => 7;
);
