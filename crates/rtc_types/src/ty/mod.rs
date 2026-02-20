mod args;
mod arithmetic;
mod bool;
mod float;
mod func;
mod int;
mod ptr;
pub mod raw;
mod sized;
mod void;

use inkwell::{
    context::ContextRef,
    types::{AnyType, BasicType},
    values::{AnyValue, AnyValueEnum, BasicValue},
};

pub trait AnyTy {
    type AnyType<'ctx>: AnyType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx>;
}

pub trait Ty: AnyTy {
    type Type<'ctx>: BasicType<'ctx>;
    fn ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::Type<'ctx>;
}

pub trait ValTy: Ty {
    type Value<'ctx>: BasicValue<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx>;
    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx>;
    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>>;
    fn type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Self::Value<'ctx> {
        Self::try_type_val(val).expect("Unexpected type")
    }
}

impl<T> Ty for T
where
    T: AnyTy,
    for<'ctx> T::AnyType<'ctx>: BasicType<'ctx>,
{
    type Type<'ctx> = T::AnyType<'ctx>;
    fn ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::Type<'ctx> {
        T::any_ty(ctx)
    }
}

pub use args::IntoFuncArgs;
pub use arithmetic::MathTy;
pub use func::FnRetTy;
pub use ptr::PtrTy;
pub use raw::*;
pub use sized::SizedTy;
pub use void::VoidTy;
