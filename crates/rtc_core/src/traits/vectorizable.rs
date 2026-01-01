use inkwell::{
    context::ContextRef,
    types::{FloatType, IntType, PointerType, VectorType},
};

use crate::ty::Ty;

pub trait InkwellVectorizableType {
    fn vectorize_scalar_type<'ctx>(self, len: u32) -> VectorType<'ctx>
    where
        Self: 'ctx;
}

impl<'slf> InkwellVectorizableType for FloatType<'slf> {
    fn vectorize_scalar_type<'ctx>(self, len: u32) -> VectorType<'ctx>
    where
        Self: 'ctx,
    {
        self.vec_type(len)
    }
}

impl<'slf> InkwellVectorizableType for IntType<'slf> {
    fn vectorize_scalar_type<'ctx>(self, len: u32) -> VectorType<'ctx>
    where
        Self: 'ctx,
    {
        self.vec_type(len)
    }
}

impl<'slf> InkwellVectorizableType for PointerType<'slf> {
    fn vectorize_scalar_type<'ctx>(self, len: u32) -> VectorType<'ctx>
    where
        Self: 'ctx,
    {
        self.vec_type(len)
    }
}

pub trait VectorizableTy: Ty {
    fn vec_ty(ctx: ContextRef<'static>, len: usize) -> VectorType<'static>;
}

impl<T> VectorizableTy for T
where
    T: Ty,
    T::Type: InkwellVectorizableType + 'static,
{
    fn vec_ty(ctx: ContextRef<'static>, len: usize) -> VectorType<'static> {
        let ty = T::new(ctx);
        T::Type::vectorize_scalar_type(ty.basic_ty(), len as _)
    }
}
