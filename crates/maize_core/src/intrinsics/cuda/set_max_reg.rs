use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
};

impl_intrinsics!(
    IncMaxNReg: "llvm.nvvm.setmaxnreg.inc.sync.aligned.u32"(u32) -> VoidType,
    DecMaxNReg: "llvm.nvvm.setmaxnreg.dec.sync.aligned.u32"(u32) -> VoidType,
);

impl CUDA {
    pub fn increase_max_regs_to(&self, num_regs: u32) {
        let num_regs = self.0.constant(num_regs);
        IncMaxNReg.call((num_regs,))
    }
    pub fn decrease_max_regs_to(&self, num_regs: u32) {
        let num_regs = self.0.constant(num_regs);
        IncMaxNReg.call((num_regs,))
    }
}
