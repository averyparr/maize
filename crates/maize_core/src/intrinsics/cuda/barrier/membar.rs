use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_argless_intrinsics},
};

impl_argless_intrinsics!(
    MembarCta: "llvm.nvvm.membar.cta"() -> VoidType,
    MembarGlobal: "llvm.nvvm.membar.gl"() -> VoidType,
    MembarSys: "llvm.nvvm.membar.sys"() -> VoidType,
);

impl CUDA {
    pub fn membar_cta(&self) {
        MembarCta(self.0.clone()).call(())
    }
    pub fn membar(&self) {
        MembarGlobal(self.0.clone()).call(())
    }
    pub fn membar_sys(&self) {
        MembarSys(self.0.clone()).call(())
    }
}
