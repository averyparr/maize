use inkwell::values::BasicValue;

use crate::{codegen::FnCodegen, ty::Ty, val::Val};

use super::S;

pub trait Holds {
    type T: Ty;
    fn to_underlying(&self) -> <Self::T as Ty>::Value;
    fn to_underlying_ty(&self) -> <Self::T as Ty>::Type;
    fn get_ty(&self) -> Self::T;
    fn held_cx(&self) -> &FnCodegen<'static>;
}

impl<'lt, T, Holder> Holds for &'lt Holder
where
    Holder: Holds<T = T>,
    T: Ty,
{
    type T = T;
    fn to_underlying(&self) -> <Self::T as Ty>::Value {
        <Holder as Holds>::to_underlying(self)
    }
    fn to_underlying_ty(&self) -> <Self::T as Ty>::Type {
        <Holder as Holds>::to_underlying_ty(self)
    }
    fn get_ty(&self) -> Self::T {
        <Holder as Holds>::get_ty(self)
    }
    fn held_cx(&self) -> &FnCodegen<'static> {
        <Holder as Holds>::held_cx(self)
    }
}

impl<'lt, T, Holder> Holds for &'lt mut Holder
where
    Holder: Holds<T = T>,
    T: Ty,
{
    type T = T;
    fn to_underlying(&self) -> <Self::T as Ty>::Value {
        <Holder as Holds>::to_underlying(self)
    }
    fn to_underlying_ty(&self) -> <Self::T as Ty>::Type {
        <Holder as Holds>::to_underlying_ty(self)
    }
    fn get_ty(&self) -> Self::T {
        <Holder as Holds>::get_ty(self)
    }
    fn held_cx(&self) -> &FnCodegen<'static> {
        <Holder as Holds>::held_cx(self)
    }
}
