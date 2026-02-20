use std::ops::{Add, Div, Mul, Neg, Sub};

use inkwell::values::BasicValue;

use crate::{ty::MathTy, val::Val};

fn post_process<T: MathTy>(val: Val<'_, T>) -> Val<'_, T> {
    if let Some(ins) = val.ll_typed().as_instruction_value() {
        val.cx().apply_ins_opt(ins);
    }
    val
}

impl<T: MathTy> Add for Val<'_, T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        post_process(T::add(self, rhs))
    }
}

impl<T: MathTy> Sub for Val<'_, T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        post_process(T::sub(self, rhs))
    }
}

impl<T: MathTy> Mul for Val<'_, T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        post_process(T::mul(self, rhs))
    }
}

impl<T: MathTy> Div for Val<'_, T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        post_process(T::div(self, rhs))
    }
}

impl<T: MathTy> Neg for Val<'_, T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        post_process(T::neg(self))
    }
}
