use std::ops::{Add, Div, Mul, Neg, Sub};

use inkwell::values::BasicValue;

use crate::{
    ty::{ArithmeticTy, Ty},
    val::{AcceptsConstants, C, Holds, Val},
};

fn to_inner<'lt, T, Inner>(val: Val<'lt, Inner>) -> Val<'lt, T>
where
    Val<'lt, Inner>: Holds<T = T>,
    T: Ty,
{
    let cx = val.cm();
    let val = val.to_underlying().as_basic_value_enum();
    Val::new(cx, val)
}

fn binop<'borrow, F: for<'lt> FnOnce(Val<'lt, T>, Val<'lt, T>) -> Val<'lt, T>, T, Inner>(
    lhs: Val<'borrow, Inner>,
    rhs: Val<'borrow, Inner>,
    f: F,
) -> Val<'borrow, T>
where
    Val<'borrow, Inner>: Holds<T = T>,
    T: Ty,
{
    f(to_inner(lhs), to_inner(rhs))
}

impl<'lt, T, Inner> Add for Val<'lt, Inner>
where
    T: ArithmeticTy,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn add(self, rhs: Self) -> Self::Output {
        binop(self, rhs, |lhs, rhs| T::build_add(lhs, rhs))
    }
}

impl<'lt, T, Inner> Sub for Val<'lt, Inner>
where
    T: ArithmeticTy,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn sub(self, rhs: Self) -> Self::Output {
        binop(self, rhs, |lhs, rhs| T::build_sub(lhs, rhs))
    }
}

impl<'lt, T, Inner> Mul for Val<'lt, Inner>
where
    T: ArithmeticTy,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn mul(self, rhs: Self) -> Self::Output {
        binop(self, rhs, |lhs, rhs| T::build_mul(lhs, rhs))
    }
}

impl<'lt, T, Inner> Div for Val<'lt, Inner>
where
    T: ArithmeticTy,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn div(self, rhs: Self) -> Self::Output {
        binop(self, rhs, |lhs, rhs| T::build_div(lhs, rhs))
    }
}

impl<'lt, T, Inner> Neg for Val<'lt, Inner>
where
    T: ArithmeticTy,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn neg(self) -> Self::Output {
        let val = Val::new(self.cm(), self.to_underlying().as_basic_value_enum());
        T::build_neg(val)
    }
}

impl<'lt, T, Inner> Add<C<T::Assoc>> for Val<'lt, Inner>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn add(self, rhs: C<T::Assoc>) -> Self::Output {
        let rhs = T::new_const(rhs.0, self.cm());
        to_inner(self) + rhs
    }
}

impl<'lt, T, Inner> Sub<C<T::Assoc>> for Val<'lt, Inner>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn sub(self, rhs: C<T::Assoc>) -> Self::Output {
        let rhs = T::new_const(rhs.0, self.cm());
        to_inner(self) - rhs
    }
}

impl<'lt, T, Inner> Mul<C<T::Assoc>> for Val<'lt, Inner>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn mul(self, rhs: C<T::Assoc>) -> Self::Output {
        let rhs = T::new_const(rhs.0, self.cm());
        to_inner(self) * rhs
    }
}

impl<'lt, T, Inner> Div<C<T::Assoc>> for Val<'lt, Inner>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn div(self, rhs: C<T::Assoc>) -> Self::Output {
        let rhs = T::new_const(rhs.0, self.cm());
        to_inner(self) / rhs
    }
}

impl<'lt, T, Inner> Add<Val<'lt, Inner>> for C<T::Assoc>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn add(self, rhs: Val<'lt, Inner>) -> Self::Output {
        let lhs = T::new_const(self.0, rhs.cm());
        lhs + to_inner(rhs)
    }
}

impl<'lt, T, Inner> Sub<Val<'lt, Inner>> for C<T::Assoc>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn sub(self, rhs: Val<'lt, Inner>) -> Self::Output {
        let lhs = T::new_const(self.0, rhs.cm());
        lhs - to_inner(rhs)
    }
}

impl<'lt, T, Inner> Mul<Val<'lt, Inner>> for C<T::Assoc>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn mul(self, rhs: Val<'lt, Inner>) -> Self::Output {
        let lhs = T::new_const(self.0, rhs.cm());
        lhs * to_inner(rhs)
    }
}

impl<'lt, T, Inner> Div<Val<'lt, Inner>> for C<T::Assoc>
where
    T: ArithmeticTy + AcceptsConstants,
    Val<'lt, Inner>: Holds<T = T>,
{
    type Output = Val<'lt, T>;
    fn div(self, rhs: Val<'lt, Inner>) -> Self::Output {
        let lhs = T::new_const(self.0, rhs.cm());
        lhs / to_inner(rhs)
    }
}
