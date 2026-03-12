use crate::{
    ty::{MathTy, MathVariant, SizedTy, raw::*, vec::VectorizableTy},
    val::Val,
};

pub trait MinMaxTy: MathTy {
    fn call_min<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self>
    where
        Self: SizedTy,
    {
        let intrinsic_name = match Self::MATH_VARIANT {
            MathVariant::Float => "llvm.minnum",
            MathVariant::SignedInt => "llvm.smin",
            MathVariant::UnsignedInt => "llvm.umin",
        };
        let func = lhs
            .cx()
            .get_intrinsic::<Self, (Self, Self)>(intrinsic_name, false);
        lhs.cx().call_fn(func, (lhs, rhs))
    }
    fn call_max<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self>
    where
        Self: SizedTy,
    {
        let intrinsic_name = match Self::MATH_VARIANT {
            MathVariant::Float => "llvm.maxnum",
            MathVariant::SignedInt => "llvm.smax",
            MathVariant::UnsignedInt => "llvm.umax",
        };
        let func = lhs
            .cx()
            .get_intrinsic::<Self, (Self, Self)>(intrinsic_name, false);
        lhs.cx().call_fn(func, (lhs, rhs))
    }
}

impl MinMaxTy for I8 {}
impl MinMaxTy for I16 {}
impl MinMaxTy for I32 {}
impl MinMaxTy for I64 {}

impl MinMaxTy for U8 {}
impl MinMaxTy for U16 {}
impl MinMaxTy for U32 {}
impl MinMaxTy for U64 {}

impl MinMaxTy for F16 {}
impl MinMaxTy for BF16 {}
impl MinMaxTy for F32 {}
impl MinMaxTy for F64 {}

impl<T: MinMaxTy + VectorizableTy, const N: usize> MinMaxTy for V<T, N> {}

impl<T: MinMaxTy> Val<'_, T> {
    pub fn min(self, rhs: Self) -> Self {
        T::call_min(self, rhs)
    }
    pub fn max(self, rhs: Self) -> Self {
        T::call_max(self, rhs)
    }
}
