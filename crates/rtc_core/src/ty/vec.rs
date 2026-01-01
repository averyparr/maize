use std::marker::PhantomData;

use inkwell::{
    builder::Builder,
    context::ContextRef,
    types::VectorType,
    values::{BasicValueEnum, VectorValue},
};

use crate::{
    traits::vectorizable::VectorizableTy,
    ty::{ArithmeticTy, FromCtx, Ty},
};

#[derive(Clone, Copy)]
pub struct V<T, const N: usize>(ContextRef<'static>, PhantomData<T>);

impl<T, const N: usize> FromCtx for V<T, N> {
    fn new(ctx: ContextRef<'static>) -> Self {
        Self(ctx, PhantomData)
    }
}

impl<T, const N: usize> Ty for V<T, N>
where
    T: VectorizableTy,
{
    const SIZE: usize = N * T::SIZE;
    const ALIGN: usize = N * T::ALIGN;

    fn ctx(&self) -> ContextRef<'static> {
        self.0
    }

    type Type = VectorType<'static>;
    fn basic_ty(&self) -> Self::Type {
        T::vec_ty(self.ctx(), N)
    }

    type Value = VectorValue<'static>;
    fn get_value(basic_val: BasicValueEnum<'static>) -> Self::Value {
        basic_val.into_vector_value()
    }
}

impl<T, const N: usize> ArithmeticTy for V<T, N>
where
    T: VectorizableTy + ArithmeticTy,
{
    fn try_emit_add(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        let add_dispatch = |b: Builder<'static>| match lhs.get_element_as_constant(0) {
            BasicValueEnum::IntValue(_) => b.build_int_add(lhs, rhs, "vint_add"),
            BasicValueEnum::FloatValue(_) => b.build_float_add(lhs, rhs, "vfloat_add"),
            BasicValueEnum::PointerValue(_) => {
                panic!("Have not determined how I want to deal with vecs of pointers")
            }
            _ => unreachable!("Nothing other than ints, floats, or pointers should be in a vec"),
        };
        // SAFETY: We have a valid pair of vec types
        unsafe { cm.cx().with_builder(add_dispatch) }
    }

    fn try_emit_sub(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        let sub_dispatch = |b: Builder<'static>| match lhs.get_element_as_constant(0) {
            BasicValueEnum::IntValue(_) => b.build_int_sub(lhs, rhs, "vint_sub"),
            BasicValueEnum::FloatValue(_) => b.build_float_sub(lhs, rhs, "vfloat_sub"),
            BasicValueEnum::PointerValue(_) => {
                panic!("Have not determined how I want to deal with vecs of pointers")
            }
            _ => unreachable!("Nothing other than ints, floats, or pointers should be in a vec"),
        };
        // SAFETY: We have a valid pair of vec types
        unsafe { cm.cx().with_builder(sub_dispatch) }
    }

    fn try_emit_mul(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        let mul_dispatch = |b: Builder<'static>| match lhs.get_element_as_constant(0) {
            BasicValueEnum::IntValue(_) => b.build_int_mul(lhs, rhs, "vint_mul"),
            BasicValueEnum::FloatValue(_) => b.build_float_mul(lhs, rhs, "vfloat_mul"),
            BasicValueEnum::PointerValue(_) => {
                panic!("Have not determined how I want to deal with vecs of pointers")
            }
            _ => unreachable!("Nothing other than ints, floats, or pointers should be in a vec"),
        };
        // SAFETY: We have a valid pair of vec types
        unsafe { cm.cx().with_builder(mul_dispatch) }
    }

    fn try_emit_div(
        cm: &crate::codegen::CodegenModule<'static>,
        lhs: Self::Value,
        rhs: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        let div_dispatch = |b: Builder<'static>| match lhs.get_element_as_constant(0) {
            // I don't know if I should do signed or unsigned div!
            BasicValueEnum::IntValue(_) => b.build_int_signed_div(lhs, rhs, "vint_div"),
            BasicValueEnum::FloatValue(_) => b.build_float_div(lhs, rhs, "vfloat_div"),
            BasicValueEnum::PointerValue(_) => {
                panic!("Have not determined how I want to deal with vecs of pointers")
            }
            _ => unreachable!("Nothing other than ints, floats, or pointers should be in a vec"),
        };
        // SAFETY: We have a valid pair of vec types
        unsafe { cm.cx().with_builder(div_dispatch) }
    }

    fn try_emit_neg(
        cm: &crate::codegen::CodegenModule<'static>,
        val: Self::Value,
    ) -> Result<Self::Value, inkwell::builder::BuilderError> {
        let neg_dispatch = |b: Builder<'static>| match val.get_element_as_constant(0) {
            // I don't know if I should do signed or unsigned div!
            BasicValueEnum::IntValue(_) => b.build_int_neg(val, "vint_neg"),
            BasicValueEnum::FloatValue(_) => b.build_float_neg(val, "vfloat_neg"),
            BasicValueEnum::PointerValue(_) => {
                panic!("Have not determined how I want to deal with vecs of pointers")
            }
            _ => unreachable!("Nothing other than ints, floats, or pointers should be in a vec"),
        };
        // SAFETY: We have a valid vec type
        unsafe { cm.cx().with_builder(neg_dispatch) }
    }
}
