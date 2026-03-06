use crate::{ty::MathTy, val::Val};

impl<'a, T> Val<'a, T>
where
    T: MathTy,
{
    pub unsafe fn add_unchecked(self, rhs: Val<'a, T>) -> Self {
        T::add_no_wrap(self, rhs)
    }
    pub unsafe fn sub_unchecked(self, rhs: Val<'a, T>) -> Self {
        T::sub_no_wrap(self, rhs)
    }
    pub unsafe fn mul_unchecked(self, rhs: Val<'a, T>) -> Self {
        T::mul_no_wrap(self, rhs)
    }
    pub unsafe fn div_exact_unchecked(self, rhs: Val<'a, T>) -> Self {
        T::div_nonzero(self, rhs)
    }
}
