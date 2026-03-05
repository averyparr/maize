mod args;
mod arithmetic;
mod array;
mod bitcast;
mod bool;
mod contiguous;
pub mod convertible;
pub mod cuda;
mod float;
mod func;
mod int;
pub mod printable;
mod ptr;
pub mod raw;
mod sized;
mod struct_ty;
pub mod vec;
mod void;

use crate::val::Val;
use inkwell::{
    context::ContextRef,
    types::{AnyType, ArrayType, BasicType},
    values::{AnyValueEnum, BasicValue},
};

pub trait AnyTy {
    type AnyType<'ctx>: AnyType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx>;
}

pub trait Ty: AnyTy {
    type Type<'ctx>: BasicType<'ctx>;
    fn ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::Type<'ctx>;
    const NEEDS_DROP: bool = false;
    fn inner_drop(_val: &mut Val<'_, Self>) {}
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

impl<T: ?Sized> Ty for T
where
    T: AnyTy,
    for<'ctx> T::AnyType<'ctx>: BasicType<'ctx>,
{
    type Type<'ctx> = T::AnyType<'ctx>;
    fn ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::Type<'ctx> {
        T::any_ty(ctx)
    }
}

impl<T> AnyTy for [T]
where
    T: Ty,
{
    type AnyType<'ctx> = ArrayType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        T::ty(ctx).array_type(0)
    }
}

pub use args::IntoFuncArgs;
pub use arithmetic::{BitMathTy, IntMathTy, MathTy, MathVariant};
pub use bitcast::BitcastableTy;
pub use contiguous::{ContiguousUniformTy, HowToExtractElements, UniformTy};
pub use func::FnRetTy;
pub use ptr::{AddrspacePtr, ConstPtrTy, MutPtrTy, MutTy, RawPtrTy, RefTy};
pub use raw::{Bool, M, P, R, V, Void, float::*, int::*};
pub use sized::{AlignedTy, SizedTy};
pub use struct_ty::{AccessibleStructTy, StructReflectTy};
pub use void::VoidTy;
