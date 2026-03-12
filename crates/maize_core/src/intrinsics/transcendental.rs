use inkwell::{types::FloatType, values::BasicValue};

use crate::{
    intrinsics::{
        UnaryIntrinsic, call_binary_intrinsic, call_unary_intrinsic,
        cuda::{Log2Approx, Log2ApproxFtz},
    },
    ty::{BF16, F16, F32, FastMathFlags, MathTy, V, vec::VectorizableTy},
    val::Val,
};

trait Transcendental {}

impl Transcendental for FloatType<'_> {}

pub trait TranscendentalTy: MathTy {
    fn exp(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.exp", false)
    }
    fn exp2(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.exp2", false)
    }
    fn exp10(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.exp10", false)
    }
    fn log(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.log", false)
    }
    fn log2(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.log2", false)
    }
    fn log10(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.log10", false)
    }
    fn sin(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.sin", false)
    }
    fn cos(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.cos", false)
    }
    fn sqrt(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.sqrt", false)
    }
    fn fabs(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.fabs", false)
    }
    fn floor(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.floor", false)
    }
    fn ceil(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.ceil", false)
    }
    fn trunc(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.trunc", false)
    }
    fn rint(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.rint", false)
    }
    fn round(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, "llvm.round", false)
    }
    fn minnum<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        call_binary_intrinsic(lhs, rhs, "llvm.minnum", false)
    }
    fn maxnum<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        call_binary_intrinsic(lhs, rhs, "llvm.maxnum", false)
    }
    fn minimum<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        call_binary_intrinsic(lhs, rhs, "llvm.minimum", false)
    }
    fn maximum<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        call_binary_intrinsic(lhs, rhs, "llvm.maximum", false)
    }
}

impl<T> TranscendentalTy for T where for<'ctx> T: MathTy<Type<'ctx>: Transcendental> {}

impl<T, const N: usize> TranscendentalTy for V<T, N> where T: TranscendentalTy + VectorizableTy {}

impl<'a, T: TranscendentalTy> Val<'a, T> {
    /// This one is supported by HW for f32 but not LLVM
    /// for FP64
    pub fn exp2(self) -> Self {
        T::exp2(self)
    }
    /// This one is supported by HW for F32 but not LLVM
    /// for any type
    // pub fn log2(self) -> Self {
    //     T::log2(self)
    // }

    /// These ones have no LLVM libcall for PTX and are
    /// typically not HW supported
    // pub fn exp(self) -> Self {
    //     T::exp(self)
    // }
    // pub fn exp10(self) -> Self {
    //     T::exp10(self)
    // }
    // pub fn log(self) -> Self {
    //     T::log(self)
    // }
    // pub fn log10(self) -> Self {
    //     T::log10(self)
    // }

    pub fn sin(self) -> Self {
        T::sin(self)
    }
    pub fn cos(self) -> Self {
        let res = T::cos(self);
        // This is necessary because these intrinsics
        // are _only_ available in approx variants
        if let Some(ins) = res.ll_typed().as_basic_value_enum().as_instruction_value() {
            ins.set_fast_math_flags(FastMathFlags::ApproxFunc())
            // .expect("Setting approx function should not fail on a fn ret val");
        }
        res
    }
    pub fn sqrt(self) -> Self {
        let res = T::sqrt(self);
        // This is necessary because these intrinsics
        // are _only_ available in approx variants
        if let Some(ins) = res.ll_typed().as_basic_value_enum().as_instruction_value() {
            ins.set_fast_math_flags(FastMathFlags::ApproxFunc())
            // .expect("Setting approx function should not fail on a fn ret val");
        }
        res
    }
    pub fn floor(self) -> Self {
        T::floor(self)
    }
    pub fn ceil(self) -> Self {
        T::ceil(self)
    }
    pub fn trunc(self) -> Self {
        T::trunc(self)
    }
    pub fn round_rni(self) -> Self {
        T::rint(self)
    }
    pub fn round(self) -> Self {
        T::round(self)
    }
    pub fn minnum(self, rhs: Self) -> Self {
        T::minnum(self, rhs)
    }
    pub fn maxnum(self, rhs: Self) -> Self {
        T::maxnum(self, rhs)
    }
    pub fn minimum(self, rhs: Self) -> Self {
        T::minimum(self, rhs)
    }
    pub fn maximum(self, rhs: Self) -> Self {
        T::maximum(self, rhs)
    }
}

pub trait Log2AbleTy: TranscendentalTy {
    fn call_log2(val: Val<'_, Self>) -> Val<'_, Self>;
}

impl<T: UnaryIntrinsic<Log2Approx> + UnaryIntrinsic<Log2ApproxFtz> + TranscendentalTy> Log2AbleTy
    for T
{
    fn call_log2(val: Val<'_, Self>) -> Val<'_, Self> {
        let mut allow_ftz = false;
        val.cx().change_opt(|o| allow_ftz = o.allow_approx_funcs());
        if allow_ftz {
            val.__intrinsic_log2_approx_ftz()
        } else {
            val.__intrinsic_log2_approx()
        }
    }
}

impl Log2AbleTy for BF16 {
    fn call_log2(val: Val<'_, Self>) -> Val<'_, Self> {
        val.cast::<F32>().log2().cast::<Self>()
    }
}

impl Log2AbleTy for F16 {
    fn call_log2(val: Val<'_, Self>) -> Val<'_, Self> {
        val.cast::<F32>().log2().cast::<Self>()
    }
}

impl<T, const N: usize> Log2AbleTy for V<T, N>
where
    T: Log2AbleTy + VectorizableTy,
{
    fn call_log2(val: Val<'_, Self>) -> Val<'_, Self> {
        Val::from_elements(val.elements().map(|e| e.log2()))
    }
}

impl<T> Val<'_, T>
where
    T: Log2AbleTy,
{
    pub fn log2(self) -> Self {
        T::call_log2(self)
    }
}
