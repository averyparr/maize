use std::f64::consts::LOG2_E;

use crate::{
    tipe::{BF16, F16, FromF32, MathTy, V, VecTy},
    val::Val,
};

pub unsafe trait HasLLVMExp: MathTy + Copy {
    fn from_f64(val: f64) -> Self;
}

unsafe impl HasLLVMExp for f64 {
    fn from_f64(val: f64) -> Self {
        val
    }
}

unsafe impl HasLLVMExp for f32 {
    fn from_f64(val: f64) -> Self {
        val as Self
    }
}

unsafe impl HasLLVMExp for BF16 {
    fn from_f64(val: f64) -> Self {
        Self::from_f32(f32::from_f64(val))
    }
}

unsafe impl HasLLVMExp for F16 {
    fn from_f64(val: f64) -> Self {
        Self::from_f32(f32::from_f64(val))
    }
}

unsafe impl<T: HasLLVMExp + VecTy, const N: usize> HasLLVMExp for V<T, N> {
    fn from_f64(val: f64) -> Self {
        V::new([T::from_f64(val); N])
    }
}

impl<T: HasLLVMExp> Val<T> {
    pub fn exp2(self) -> Self {
        let fn_ref = self.fn_ref().clone();
        let exp2 = fn_ref
            .get_intrinsic::<T, (T,)>("llvm.exp2", false, &[])
            .expect("ex2 should exist");
        fn_ref.call_extern(exp2, (self,), None)
    }

    pub fn exp(self) -> Val<T> {
        // Do this because exp2 exists on CUDA, but exp doesn't
        let log2_e = self.fn_ref().constant(T::from_f64(LOG2_E));
        self.exp2() * log2_e
    }
}
