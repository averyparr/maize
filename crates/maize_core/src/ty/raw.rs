use std::marker::PhantomData;

use crate::ty::ptr::DefaultAddressSpace;

macro_rules! declare_zst_types {
    ($($tipes: ident),*) => {
        $(
            #[allow(unused)]
            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $tipes;

            impl $tipes {
                #[allow(unused)]
                pub fn new() -> Self {
                    Self
                }
            }
        )*
    };
}

declare_zst_types!(
    I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F16, BF16, F32, F64, F128, Void, Bool, E4M3,
    E5M2
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct V<T, const N: usize>(PhantomData<T>);

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct P<T: ?Sized, Space = DefaultAddressSpace>(PhantomData<T>, PhantomData<Space>);
impl<T, Space> Clone for P<T, Space> {
    fn clone(&self) -> Self {
        P(PhantomData, PhantomData)
    }
}
impl<T, Space> Copy for P<T, Space> {}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct R<T: ?Sized, Space = DefaultAddressSpace>(PhantomData<T>, PhantomData<Space>);
impl<T, Space> Clone for R<T, Space> {
    fn clone(&self) -> Self {
        R(PhantomData, PhantomData)
    }
}
impl<T, Space> Copy for R<T, Space> {}
// _Must_ not be Copy or we immediately break aliasing
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct M<T: ?Sized, Space = DefaultAddressSpace>(PhantomData<T>, PhantomData<Space>);

impl<T, const N: usize> V<T, N> {
    #[allow(unused)]
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub mod int {
    pub use super::{I8, I16, I32, I64, I128, U8, U16, U32, U64, U128};
}

pub mod float {
    pub use super::{BF16, F16, F128};
    pub use super::{E4M3, E5M2};
    pub use super::{F32, F64};
}
