use crate::ty::{AnyTy, raw::*};

use super::ValTy;

pub trait HasMaterializedType {
    type Materialized;
}

pub trait AlignedTy: AnyTy {
    const ALIGN: u32;
}

pub trait SizedTy: AlignedTy + ValTy {
    const SIZE: u32;
}

impl<T> AlignedTy for T
where
    T: ValTy + HasMaterializedType,
{
    const ALIGN: u32 = ::std::mem::align_of::<T::Materialized>() as u32;
}

impl<T> SizedTy for T
where
    T: ValTy + HasMaterializedType,
{
    const SIZE: u32 = ::std::mem::size_of::<T::Materialized>() as u32;
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
            impl HasMaterializedType for $mock {
                type Materialized = $materialized;
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

impl<T> HasMaterializedType for P<*const T> {
    type Materialized = *const T;
}

impl<T> HasMaterializedType for P<*mut T> {
    type Materialized = *mut T;
}

impl<'a, T> HasMaterializedType for R<&'a T> {
    type Materialized = &'a T;
}

impl<'a, T> HasMaterializedType for M<&'a mut T> {
    type Materialized = &'a mut T;
}
