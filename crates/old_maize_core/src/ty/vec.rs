use inkwell::{
    context::ContextRef,
    types::{FloatType, IntType, VectorType},
    values::{AnyValueEnum, VectorValue},
};

use crate::ty::{AlignedTy, AnyTy, SizedTy, Ty, V, ValTy};

trait Vectorizable<'ctx> {
    fn inkwell_vec_ty(self, size: u32) -> VectorType<'ctx>;
}

impl<'ctx> Vectorizable<'ctx> for IntType<'ctx> {
    fn inkwell_vec_ty(self, size: u32) -> VectorType<'ctx> {
        self.vec_type(size)
    }
}

impl<'ctx> Vectorizable<'ctx> for FloatType<'ctx> {
    fn inkwell_vec_ty(self, size: u32) -> VectorType<'ctx> {
        self.vec_type(size)
    }
}

impl<T> VectorizableTy for T
where
    T: SizedTy,
    for<'ctx> T::Type<'ctx>: Vectorizable<'ctx>,
{
}

#[expect(private_bounds)]
pub trait VectorizableTy: for<'ctx> SizedTy<Type<'ctx>: Vectorizable<'ctx>> {
    fn vec_ty(ctx: ContextRef<'_>, size: usize) -> VectorType<'_> {
        Self::Type::inkwell_vec_ty(
            Self::ty(ctx),
            u32::try_from(size).expect("usize -> u32 overflow"),
        )
    }
}

impl<T, const N: usize> AnyTy for V<T, N>
where
    T: VectorizableTy,
{
    type AnyType<'ctx> = VectorType<'ctx>;
    fn any_ty<'ctx>(ctx: inkwell::context::ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        T::vec_ty(ctx, N as _)
    }
}

impl<T, const N: usize> ValTy for V<T, N>
where
    T: VectorizableTy,
{
    type Value<'ctx> = VectorValue<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).get_undef()
    }

    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).const_zero()
    }

    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        if let AnyValueEnum::VectorValue(val) = val {
            Some(val)
        } else {
            None
        }
    }
}

const fn align_of_vector<T: AlignedTy, const N: usize>() -> u32 {
    let mut n = N;
    let mut full_align = T::ALIGN;
    while n % 2 == 0 {
        full_align *= 2;
        n /= 2;
    }
    full_align
}

impl<T, const N: usize> AlignedTy for V<T, N>
where
    T: AlignedTy + VectorizableTy,
{
    const ALIGN: u32 = align_of_vector::<T, N>();
}

impl<T, const N: usize> SizedTy for V<T, N>
where
    T: VectorizableTy,
{
    const SIZE: u32 = (N as u32) * T::ALIGN;
}
