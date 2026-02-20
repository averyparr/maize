use std::marker::PhantomData;

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
    I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F16, BF16, F32, F64, F128, Void, Bool
);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct V<T, const N: usize>(PhantomData<T>);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct P<T>(PhantomData<T>);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct R<T>(PhantomData<T>);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct M<T>(PhantomData<T>);

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
    pub use super::{F32, F64};
}
