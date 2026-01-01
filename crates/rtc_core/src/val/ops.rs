use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{
    traits::constants::{AcceptsConstants, C},
    ty::ArithmeticTy,
    val::Val,
};

impl<'lt, T> Add for Val<'lt, T>
where
    T: ArithmeticTy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        T::build_add(self, rhs)
    }
}

impl<'lt, T> Sub for Val<'lt, T>
where
    T: ArithmeticTy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        T::build_sub(self, rhs)
    }
}

impl<'lt, T> Mul for Val<'lt, T>
where
    T: ArithmeticTy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        T::build_mul(self, rhs)
    }
}

impl<'lt, T> Div for Val<'lt, T>
where
    T: ArithmeticTy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        T::build_div(self, rhs)
    }
}

impl<'lt, T> Neg for Val<'lt, T>
where
    T: ArithmeticTy + 'lt,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        T::build_neg(self)
    }
}

impl<'lt, T> Add<C<T::Assoc>> for Val<'lt, T>
where
    T: ArithmeticTy + 'lt + AcceptsConstants,
{
    type Output = Self;
    fn add(self, rhs: C<T::Assoc>) -> Self::Output {
        let vald = T::new_const(rhs.0, self.cm());
        self + vald
    }
}

impl<'lt, T> Sub<C<T::Assoc>> for Val<'lt, T>
where
    T: ArithmeticTy + 'lt + AcceptsConstants,
{
    type Output = Self;
    fn sub(self, rhs: C<T::Assoc>) -> Self::Output {
        let vald = T::new_const(rhs.0, self.cm());
        self - vald
    }
}

impl<'lt, T> Mul<C<T::Assoc>> for Val<'lt, T>
where
    T: ArithmeticTy + 'lt + AcceptsConstants,
{
    type Output = Self;
    fn mul(self, rhs: C<T::Assoc>) -> Self::Output {
        let vald = T::new_const(rhs.0, self.cm());
        self * vald
    }
}

impl<'lt, T> Div<C<T::Assoc>> for Val<'lt, T>
where
    T: ArithmeticTy + 'lt + AcceptsConstants,
{
    type Output = Self;
    fn div(self, rhs: C<T::Assoc>) -> Self::Output {
        let vald = T::new_const(rhs.0, self.cm());
        self / vald
    }
}
