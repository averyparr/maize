use inkwell::{
    context::ContextRef,
    types::{ArrayType, BasicType},
    values::{AnyValueEnum, ArrayValue},
};

use crate::ty::{AlignedTy, AnyTy, SizedTy, Ty, ValTy};

impl<T, const N: usize> AnyTy for [T; N]
where
    T: Ty,
{
    type AnyType<'ctx> = ArrayType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        T::ty(ctx).array_type(N as _)
    }
}

impl<T, const N: usize> ValTy for [T; N]
where
    T: Ty,
{
    type Value<'ctx> = ArrayValue<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).get_undef()
    }

    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).const_zero()
    }

    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        if let AnyValueEnum::ArrayValue(val) = val {
            Some(val)
        } else {
            None
        }
    }
}

impl<T, const N: usize> AlignedTy for [T; N]
where
    T: SizedTy,
{
    const ALIGN: u32 = T::ALIGN;
}

impl<T, const N: usize> SizedTy for [T; N]
where
    T: SizedTy,
{
    const SIZE: u32 = T::SIZE * (N as u32);
}
