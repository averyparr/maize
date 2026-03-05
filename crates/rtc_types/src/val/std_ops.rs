use std::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

use inkwell::values::BasicValue;

use crate::{
    ty::{BitMathTy, IntMathTy, MathTy, raw::*, vec::VectorizableTy},
    val::Val,
};

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

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Add<Val<'a, T>> for Val<'a, V<T, N>> {
    type Output = Self;
    fn add(self, rhs: Val<'a, T>) -> Self::Output {
        Add::add(self, Val::splat(rhs))
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Sub<Val<'a, T>> for Val<'a, V<T, N>> {
    type Output = Self;
    fn sub(self, rhs: Val<'a, T>) -> Self::Output {
        Sub::sub(self, Val::splat(rhs))
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Mul<Val<'a, T>> for Val<'a, V<T, N>> {
    type Output = Self;
    fn mul(self, rhs: Val<'a, T>) -> Self::Output {
        Mul::mul(self, Val::splat(rhs))
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Div<Val<'a, T>> for Val<'a, V<T, N>> {
    type Output = Self;
    fn div(self, rhs: Val<'a, T>) -> Self::Output {
        Div::div(self, Val::splat(rhs))
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Add<Val<'a, V<T, N>>> for Val<'a, T> {
    type Output = Val<'a, V<T, N>>;
    fn add(self, rhs: Val<'a, V<T, N>>) -> Self::Output {
        Add::add(Val::splat(self), rhs)
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Sub<Val<'a, V<T, N>>> for Val<'a, T> {
    type Output = Val<'a, V<T, N>>;
    fn sub(self, rhs: Val<'a, V<T, N>>) -> Self::Output {
        Sub::sub(Val::splat(self), rhs)
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Mul<Val<'a, V<T, N>>> for Val<'a, T> {
    type Output = Val<'a, V<T, N>>;
    fn mul(self, rhs: Val<'a, V<T, N>>) -> Self::Output {
        Mul::mul(Val::splat(self), rhs)
    }
}

impl<'a, T: VectorizableTy + MathTy + Copy, const N: usize> Div<Val<'a, V<T, N>>> for Val<'a, T> {
    type Output = Val<'a, V<T, N>>;
    fn div(self, rhs: Val<'a, V<T, N>>) -> Self::Output {
        Div::div(Val::splat(self), rhs)
    }
}

macro_rules! impl_math_for_constants {
    (inner: $op_ty: ident, $op_fn: ident, $trace_ty: ty, $real_ty: ty) => {

        impl<'a> $op_ty<$real_ty> for Val<'a, $trace_ty> {
            type Output = Self;
            fn $op_fn(self, rhs: $real_ty) -> Self::Output {
                let const_typed = self.cx().constant::<$trace_ty>(rhs);
                $op_ty::$op_fn(self, const_typed)
            }
        }
        impl<'a> $op_ty<Val<'a, $trace_ty>> for $real_ty {
            type Output = Val<'a, $trace_ty>;
            fn $op_fn(self, rhs: Val<'a, $trace_ty>) -> Self::Output {
                let const_typed = rhs.cx().constant::<$trace_ty>(self);
                $op_ty::$op_fn(const_typed, rhs)
            }
        }
    };
    ($trace_ty: ty => $real_ty: ty) => {
        impl_math_for_constants!(inner: Add, add, $trace_ty, $real_ty);
        impl_math_for_constants!(inner: Sub, sub, $trace_ty, $real_ty);
        impl_math_for_constants!(inner: Mul, mul, $trace_ty, $real_ty);
        impl_math_for_constants!(inner: Div, div, $trace_ty, $real_ty);
    };
}

// These are here and use f32 because Rust doesn't have great support
// so we don't want to require (or imply) excessive precision.
impl_math_for_constants!(BF16 => f32);
impl_math_for_constants!(F16 => f32);

impl_math_for_constants!(F32 => f32);
impl_math_for_constants!(F64 => f64);

impl_math_for_constants!(U8 => u8);
impl_math_for_constants!(U16 => u16);
impl_math_for_constants!(U32 => u32);
impl_math_for_constants!(U64 => u64);

impl_math_for_constants!(I8 => i8);
impl_math_for_constants!(I16 => i16);
impl_math_for_constants!(I32 => i32);
impl_math_for_constants!(I64 => i64);

impl Not for Val<'_, Bool> {
    type Output = Self;
    fn not(self) -> Self::Output {
        let value = self.ll_typed();
        let raw_val = unsafe { self.cx().with_builder(|b| b.build_not(value, "not")) }
            .expect("Build not should succed");
        unsafe { Val::new(self.cx(), raw_val.as_basic_value_enum()) }
    }
}

impl<'a, IntT: IntMathTy> Rem<Self> for Val<'a, IntT> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        IntT::rem(self, rhs)
    }
}

impl<'a, IntT: IntMathTy> Shl<Self> for Val<'a, IntT> {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        IntT::left_shift(self, rhs)
    }
}

impl<'a, IntT: IntMathTy> Shr<Self> for Val<'a, IntT> {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        IntT::right_shift(self, rhs)
    }
}

impl<'a, IntT: BitMathTy> BitXor<Self> for Val<'a, IntT> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        IntT::xor(self, rhs)
    }
}
impl<'a, IntT: BitMathTy> BitAnd<Self> for Val<'a, IntT> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        IntT::and(self, rhs)
    }
}
impl<'a, IntT: BitMathTy> BitOr<Self> for Val<'a, IntT> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        IntT::or(self, rhs)
    }
}

macro_rules! impl_int_math_for_constants {
    (inner: $op_ty: ident, $op_fn: ident, $trace_ty: ty, $real_ty: ty) => {

        impl<'a> $op_ty<$real_ty> for Val<'a, $trace_ty> {
            type Output = Self;
            fn $op_fn(self, rhs: $real_ty) -> Self::Output {
                let const_typed = self.cx().constant::<$trace_ty>(rhs);
                $op_ty::$op_fn(self, const_typed)
            }
        }
        impl<'a> $op_ty<Val<'a, $trace_ty>> for $real_ty {
            type Output = Val<'a, $trace_ty>;
            fn $op_fn(self, rhs: Val<'a, $trace_ty>) -> Self::Output {
                let const_typed = rhs.cx().constant::<$trace_ty>(self);
                $op_ty::$op_fn(const_typed, rhs)
            }
        }
    };
    ($trace_ty: ty => $real_ty: ty) => {
        impl_int_math_for_constants!(inner: Rem, rem, $trace_ty, $real_ty);
        impl_int_math_for_constants!(inner: Shr, shr, $trace_ty, $real_ty);
        impl_int_math_for_constants!(inner: Shl, shl, $trace_ty, $real_ty);
        impl_int_math_for_constants!(inner: BitXor, bitxor, $trace_ty, $real_ty);
        impl_int_math_for_constants!(inner: BitAnd, bitand, $trace_ty, $real_ty);
        impl_int_math_for_constants!(inner: BitOr, bitor, $trace_ty, $real_ty);
    };
}

impl_int_math_for_constants!(U8 => u8);
impl_int_math_for_constants!(U16 => u16);
impl_int_math_for_constants!(U32 => u32);
impl_int_math_for_constants!(U64 => u64);

impl_int_math_for_constants!(I8 => i8);
impl_int_math_for_constants!(I16 => i16);
impl_int_math_for_constants!(I32 => i32);
impl_int_math_for_constants!(I64 => i64);

impl_int_math_for_constants!(inner: BitXor, bitxor, Bool, bool);
impl_int_math_for_constants!(inner: BitAnd, bitand, Bool, bool);
impl_int_math_for_constants!(inner: BitOr, bitor, Bool, bool);
