use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{ty::MathTy, val::Val};

impl<T: MathTy> Add for Val<'_, T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        T::add(self, rhs)
    }
}

impl<T: MathTy> Sub for Val<'_, T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        T::sub(self, rhs)
    }
}

impl<T: MathTy> Mul for Val<'_, T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        T::mul(self, rhs)
    }
}

impl<T: MathTy> Div for Val<'_, T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        T::div(self, rhs)
    }
}

impl<T: MathTy> Neg for Val<'_, T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        T::neg(self)
    }
}
