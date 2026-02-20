use crate::ty::{AnyTy, raw::*};

use super::ValTy;

pub trait HasMaterializedType {
    type Materialized;
}

pub trait SizedTy: ValTy {
    const SIZE: usize;
    const ALIGN: usize;
}

impl<T> SizedTy for T
where
    T: ValTy + HasMaterializedType,
{
    const SIZE: usize = ::std::mem::size_of::<T::Materialized>();
    const ALIGN: usize = ::std::mem::align_of::<T::Materialized>();
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

impl<T> HasMaterializedType for P<T>
where
    T: HasMaterializedType,
{
    type Materialized = *mut T::Materialized;
}

impl<'a, T> HasMaterializedType for R<&'a T> {
    type Materialized = &'a T;
}

impl<'a, T> HasMaterializedType for M<&'a mut T> {
    type Materialized = &'a mut T;
}
