use inkwell::values::FastMathFlags;

use crate::{
    tipe::{BF16, F16, FromF32, MathTy, V, VecTy},
    val::Val,
};

pub unsafe trait HasLLVMTrig: MathTy + Copy {
    fn from_f64(val: f64) -> Self;
}

unsafe impl HasLLVMTrig for f64 {
    fn from_f64(val: f64) -> Self {
        val
    }
}

unsafe impl HasLLVMTrig for f32 {
    fn from_f64(val: f64) -> Self {
        val as Self
    }
}

unsafe impl HasLLVMTrig for BF16 {
    fn from_f64(val: f64) -> Self {
        Self::from_f32(f32::from_f64(val))
    }
}

unsafe impl HasLLVMTrig for F16 {
    fn from_f64(val: f64) -> Self {
        Self::from_f32(f32::from_f64(val))
    }
}

unsafe impl<T: HasLLVMTrig + VecTy, const N: usize> HasLLVMTrig for V<T, N> {
    fn from_f64(val: f64) -> Self {
        V::new([T::from_f64(val); N])
    }
}

impl<T: HasLLVMTrig> Val<T> {
    fn trig(self, name: &str) -> Self {
        let fn_ref = self.fn_ref().clone();
        let mut old_flags = None;
        fn_ref.set_ins_flags(|f| {
            old_flags = Some(f.fast_math);
            f.fast_math.insert(FastMathFlags::ApproxFunc);
        });
        let func = fn_ref
            .get_intrinsic::<T, (T,)>(name, false)
            .unwrap_or_else(|e| panic!("{name} has no LLVM intrinsic, but found error {e:?}"));
        let ret = fn_ref.call_extern(func, (self,), None);
        fn_ref.set_ins_flags(|f| f.fast_math = old_flags.expect("Should be set"));
        ret
    }
    pub fn cos(self) -> Self {
        self.trig("llvm.cos")
    }
    pub fn sin(self) -> Self {
        self.trig("llvm.sin")
    }
    pub fn tanh(self) -> Self {
        self.trig("llvm.tanh")
    }
}
