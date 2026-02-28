use inkwell::{attributes::Attribute, context::ContextRef};

use crate::ty::{AnyTy, Ty, raw::*};

use super::ValTy;

pub trait AlignedTy: AnyTy {
    const ALIGN: u32;
}

pub trait SizedTy: AlignedTy + ValTy {
    const SIZE: u32;
    fn fn_arg_attrs(ctx: ContextRef<'_>) -> impl IntoIterator<Item = Attribute> {
        let align_id = Attribute::get_named_enum_kind_id("align");
        let enum_attr = ctx.create_enum_attribute(align_id, Self::ALIGN as _);
        [enum_attr]
    }
}

impl<T> AlignedTy for [T]
where
    T: SizedTy,
{
    const ALIGN: u32 = T::ALIGN;
}

macro_rules! impl_size_align {
    ($($mock: ty => $materialized: ty),*$(,)?) => {
        $(
            impl AlignedTy for $mock {
                const ALIGN: u32 = ::std::mem::align_of::<$materialized>() as _;
            }
            impl SizedTy for $mock {
                const SIZE: u32 = ::std::mem::size_of::<$materialized>() as _;
            }
        )*
    };
}

impl_size_align!(
    F16 => u16,
    F32 => f32,
    F64 => f64,
    F128 => u128,
    BF16 => u16,

    I8 => i8,
    I16 => i16,
    I32 => i32,
    I64 => i64,
    I128 => i128,

    U8 => u8,
    U16 => u16,
    U32 => u32,
    U64 => u64,
    U128 => u128,
);

impl<T: ?Sized> AlignedTy for P<*const T>
where
    T: AnyTy,
{
    const ALIGN: u32 = ::std::mem::align_of::<*const ()>() as _;
}

impl<T: ?Sized> AlignedTy for P<*mut T>
where
    T: AnyTy,
{
    const ALIGN: u32 = ::std::mem::align_of::<*const ()>() as _;
}

impl<T: ?Sized> AlignedTy for R<&T>
where
    T: Ty,
{
    const ALIGN: u32 = ::std::mem::align_of::<&()>() as _;
}

impl<T: ?Sized> AlignedTy for M<&mut T>
where
    T: Ty,
{
    const ALIGN: u32 = ::std::mem::align_of::<&mut ()>() as _;
}

impl<T: ?Sized> SizedTy for P<*const T>
where
    T: AnyTy,
{
    const SIZE: u32 = ::std::mem::size_of::<*const ()>() as _;
}

impl<T: ?Sized> SizedTy for P<*mut T>
where
    T: AnyTy,
{
    const SIZE: u32 = ::std::mem::size_of::<*const ()>() as _;
}

impl<T: ?Sized> SizedTy for R<&T>
where
    T: Ty,
{
    const SIZE: u32 = ::std::mem::size_of::<&()>() as _;

    fn fn_arg_attrs(ctx: ContextRef<'_>) -> impl IntoIterator<Item = Attribute> {
        let align_id = Attribute::get_named_enum_kind_id("align");
        let enum_attr = ctx.create_enum_attribute(align_id, Self::ALIGN as _);
        let noalias_id = Attribute::get_named_enum_kind_id("noalias");
        let noalias_attr = ctx.create_enum_attribute(noalias_id, 0);
        let nonnull_id = Attribute::get_named_enum_kind_id("nonnull");
        let nonnull_attr = ctx.create_enum_attribute(nonnull_id, 0);
        [enum_attr, noalias_attr, nonnull_attr]
    }
}

impl<T: ?Sized> SizedTy for M<&mut T>
where
    T: Ty,
{
    const SIZE: u32 = ::std::mem::size_of::<&mut ()>() as _;
    fn fn_arg_attrs(ctx: ContextRef<'_>) -> impl IntoIterator<Item = Attribute> {
        let align_id = Attribute::get_named_enum_kind_id("align");
        let enum_attr = ctx.create_enum_attribute(align_id, Self::ALIGN as _);
        let noalias_id = Attribute::get_named_enum_kind_id("noalias");
        let noalias_attr = ctx.create_enum_attribute(noalias_id, 0);
        let nonnull_id = Attribute::get_named_enum_kind_id("nonnull");
        let nonnull_attr = ctx.create_enum_attribute(nonnull_id, 0);
        [enum_attr, noalias_attr, nonnull_attr]
    }
}
