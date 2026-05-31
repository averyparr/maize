use inkwell::values::FastMathFlags;

use crate::{
    intrinsics::{Intrinsic, impl_intrinsics},
    tipe::{BF16, F16, V},
    val::Val,
};

impl_intrinsics!(
    Lg2ApproxF64: "llvm.nvvm.lg2.approx.d"(f64) -> f64,
    Lg2ApproxF32: "llvm.nvvm.lg2.approx.f"(f32) -> f32,
    Lg2ApproxFtzF32: "llvm.nvvm.lg2.approx.ftz.f"(f32) -> f32,
);

trait NVVMLog: Sized {
    fn log2(val: Val<Self>) -> Val<Self>;
}

#[expect(private_bounds)]
impl<T: NVVMLog> Val<T> {
    pub fn nvvm_log2(self) -> Self {
        T::log2(self)
    }
}

impl NVVMLog for f32 {
    fn log2(val: Val<Self>) -> Val<Self> {
        let mut is_approx = None;
        val.fn_ref().set_ins_flags(|flags| {
            is_approx = Some(flags.fast_math.contains(FastMathFlags::ApproxFunc))
        });
        let is_approx = is_approx.expect("Should have been set");
        if is_approx {
            Lg2ApproxFtzF32.call((val,))
        } else {
            Lg2ApproxF32.call((val,))
        }
    }
}

impl NVVMLog for f64 {
    fn log2(val: Val<Self>) -> Val<Self> {
        Lg2ApproxF64.call((val,))
    }
}

impl NVVMLog for BF16 {
    fn log2(val: Val<Self>) -> Val<Self> {
        val.cvt::<f32>().nvvm_log2().cvt()
    }
}

impl NVVMLog for F16 {
    fn log2(val: Val<Self>) -> Val<Self> {
        val.cvt::<f32>().nvvm_log2().cvt()
    }
}

impl<const N: usize> NVVMLog for V<f64, N> {
    fn log2(val: Val<Self>) -> Val<Self> {
        Val::from_elements(val.elements().map(|e| e.nvvm_log2()))
    }
}

impl<const N: usize> NVVMLog for V<f32, N> {
    fn log2(val: Val<Self>) -> Val<Self> {
        Val::from_elements(val.elements().map(|e| e.nvvm_log2()))
    }
}

// Doing conv before dispatch to intrinsics leads to better LLVM
// recombination of e.g. other casts or math; the intrinsic
// seems to act like an optimization barrier.
impl<const N: usize> NVVMLog for V<BF16, N> {
    fn log2(val: Val<Self>) -> Val<Self> {
        Val::from_elements(val.cvt::<V<f32, N>>().elements().map(|e| e.nvvm_log2())).cvt()
    }
}

impl<const N: usize> NVVMLog for V<F16, N> {
    fn log2(val: Val<Self>) -> Val<Self> {
        Val::from_elements(val.cvt::<V<f32, N>>().elements().map(|e| e.nvvm_log2())).cvt()
    }
}
