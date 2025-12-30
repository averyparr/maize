use std::marker::PhantomData;

use inkwell::{
    context::ContextRef,
    types::{FloatType, IntType, PointerType, VectorType},
    values::{BasicValueEnum, VectorValue},
};

use crate::ty::{ArithmeticTy, CodegenModule, FromCtx, Ty, VecTy};

#[derive(Clone, Copy)]
pub struct VF<T, const N: u32>(ContextRef<'static>, PhantomData<T>);
#[derive(Clone, Copy)]
pub struct VI<T, const N: u32>(ContextRef<'static>, PhantomData<T>);
#[derive(Clone, Copy)]
pub struct VU<T, const N: u32>(ContextRef<'static>, PhantomData<T>);

macro_rules! impl_traits_for_vec_ty {
    ($name: ident, $inkwell_ty: ident, $add: ident, $sub: ident, $mul: ident, $div: ident, $neg: ident) => {
        impl<T, const N: u32> FromCtx for $name<T, N> {
            fn new(ctx: ContextRef<'static>) -> Self {
                Self(ctx, PhantomData)
            }
        }

        impl<T, const N: u32> VecTy for $name<T, N>
        where
            T: Ty<Type = $inkwell_ty<'static>>,
        {
            const N: usize = N as _;
            type ElemT = T;
        }

        impl<T, const N: u32> Ty for $name<T, N>
        where
            T: Ty<Type = $inkwell_ty<'static>>,
        {
            const ALIGN: u32 = N * T::ALIGN;
            const SIZE: usize = (N as usize) * T::SIZE;
            fn ctx(&self) -> ContextRef<'static> {
                self.0
            }
            type Type = VectorType<'static>;
            fn basic_ty(&self) -> Self::Type {
                T::new(self.ctx()).basic_ty().vec_type(N)
            }
            fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value {
                basic_val.into_vector_value()
            }
            type Value = VectorValue<'static>;
        }

        impl<T, const N: u32> ArithmeticTy for $name<T, N>
        where
            T: ArithmeticTy<Type = $inkwell_ty<'static>>,
        {
            fn try_emit_add(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: We have two vecs so add is safe
                unsafe { cm.cx().with_builder(|b| b.$add(lhs, rhs, "vec_float_add")) }
            }
            fn try_emit_sub(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: We have two vecs so sub is safe
                unsafe { cm.cx().with_builder(|b| b.$sub(lhs, rhs, "vec_float_sub")) }
            }
            fn try_emit_mul(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: We have two vecs so mul is safe
                unsafe { cm.cx().with_builder(|b| b.$mul(lhs, rhs, "vec_float_mul")) }
            }
            fn try_emit_div(
                cm: &CodegenModule<'static>,
                lhs: Self::Value,
                rhs: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: We have two vecs so div is safe
                unsafe { cm.cx().with_builder(|b| b.$div(lhs, rhs, "vec_float_div")) }
            }
            fn try_emit_neg(
                cm: &CodegenModule<'static>,
                val: Self::Value,
            ) -> Result<Self::Value, inkwell::builder::BuilderError> {
                // Safety: We have a vec-val so neg is safe
                unsafe { cm.cx().with_builder(|b| b.$neg(val, "vec_float_neg")) }
            }
        }
    };
}

impl_traits_for_vec_ty!(
    VF,
    FloatType,
    build_float_add,
    build_float_sub,
    build_float_mul,
    build_float_div,
    build_int_neg
);
impl_traits_for_vec_ty!(
    VI,
    IntType,
    build_int_add,
    build_int_sub,
    build_int_mul,
    build_int_signed_div,
    build_int_neg
);
impl_traits_for_vec_ty!(
    VU,
    IntType,
    build_int_add,
    build_int_sub,
    build_int_mul,
    build_int_unsigned_div,
    build_int_neg
);
