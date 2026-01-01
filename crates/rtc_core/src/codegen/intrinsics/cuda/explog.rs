use crate::{
    codegen::func_with_args::Func,
    ty::{F16, F32, F64, Ty},
    val::Val,
};

pub trait HasExpLog2: Ty {
    const EX2_APPROX: &str;
    const EX2_APPROX_FTZ: Option<&str>;
    const LG2_APPROX: &str;
    const LG2_APPROX_FTZ: Option<&str>;
}

impl HasExpLog2 for F32 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.f";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.f");
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.f";
    const LG2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.lg2.approx.ftz.f");
}

impl HasExpLog2 for F64 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.d";
    const EX2_APPROX_FTZ: Option<&str> = None;
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.d";
    const LG2_APPROX_FTZ: Option<&str> = None;
}

impl HasExpLog2 for F16 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.f16";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.f16");
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.ftz.f16";
    const LG2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.lg2.approx.ftz.f16");
}

pub trait Log2ableType: Ty {}

impl Log2ableType for F32 {}

impl Log2ableType for F64 {}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn exp2<Float: HasExpLog2>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        // Safety: `Float` implements `HasExpLog2`
        unsafe { self.cm_ref().call_unary_function(val, Float::EX2_APPROX) }
    }

    pub fn exp2_ftz<Float: HasExpLog2>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        // Safety: `Float` implements `HasExpLog2`
        unsafe {
            self.cm_ref()
                .call_unary_function(val, Float::EX2_APPROX_FTZ.unwrap_or(Float::EX2_APPROX))
        }
    }

    pub fn log2<Float: HasExpLog2>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        // Safety: `Float` implements `HasExpLog2`
        unsafe { self.cm_ref().call_unary_function(val, Float::LG2_APPROX) }
    }

    pub fn log2_ftz<Float: HasExpLog2>(&self, val: Val<'_, Float>) -> Val<'_, Float> {
        // Safety: `Float` implements `HasExpLog2`
        unsafe {
            self.cm_ref()
                .call_unary_function(val, Float::LG2_APPROX_FTZ.unwrap_or(Float::LG2_APPROX))
        }
    }
}
