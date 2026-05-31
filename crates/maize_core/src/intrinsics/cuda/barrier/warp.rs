use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
    val::Val,
};

impl_intrinsics!(WarpSync: "llvm.nvvm.bar.warp.sync"(u32) -> VoidType);

impl CUDA {
    pub fn warp_sync(&self, mask: Val<u32>) {
        WarpSync.call((mask,))
    }
    pub fn warp_sync_uniform(&self) {
        self.warp_sync(self.0.constant(0xFFFFFFFF));
    }
}
