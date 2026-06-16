use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_argless_intrinsics, impl_intrinsics},
    val::Val,
};

impl_intrinsics!(
    CtaArriveCount: "llvm.nvvm.barrier.cta.arrive.count" (u32, u32) -> VoidType,
    CtaReduceAndCount: "llvm.nvvm.barrier.cta.red.and.count" (u32, u32, bool) -> bool,
    CtaReduceOrCount: "llvm.nvvm.barrier.cta.red.or.count" (u32, u32, bool) -> bool,
    CtaReducePopcCount: "llvm.nvvm.barrier.cta.red.popc.count" (u32, u32, bool) -> u32,
    CtaSyncCount: "llvm.nvvm.barrier.cta.sync.count" (u32, u32) -> VoidType,
    CtaSync: "llvm.nvvm.barrier.cta.sync.aligned.all"(u32) -> VoidType,
);

impl_argless_intrinsics!();

impl CUDA {
    pub fn cta_arrive_count(num_threads: Val<u32>, barrier_id: u32) {
        assert!(barrier_id < 16);
        CtaArriveCount.call((num_threads.fn_ref().constant(barrier_id), num_threads))
    }
    pub fn cta_reduce_and_count(
        num_threads: Val<u32>,
        val: Val<bool>,
        barrier_id: u32,
    ) -> Val<bool> {
        assert!(barrier_id < 16);
        CtaReduceAndCount.call((num_threads.fn_ref().constant(barrier_id), num_threads, val))
    }
    pub fn cta_reduce_or_count(
        num_threads: Val<u32>,
        val: Val<bool>,
        barrier_id: u32,
    ) -> Val<bool> {
        assert!(barrier_id < 16);
        CtaReduceOrCount.call((num_threads.fn_ref().constant(barrier_id), num_threads, val))
    }
    pub fn cta_reduce_popc_count(
        num_threads: Val<u32>,
        val: Val<bool>,
        barrier_id: u32,
    ) -> Val<u32> {
        assert!(barrier_id < 16);
        CtaReducePopcCount.call((num_threads.fn_ref().constant(barrier_id), num_threads, val))
    }
    pub fn cta_sync_count(num_threads: Val<u32>, barrier_id: u32) {
        assert!(barrier_id < 16);
        CtaSyncCount.call((num_threads.fn_ref().constant(barrier_id), num_threads))
    }
    pub fn cta_sync(&self, barrier_id: u32) {
        assert!(barrier_id < 16);
        CtaSync.call((self.0.constant(barrier_id),))
    }
    pub fn sync_threads(&self) {
        self.cta_sync(0);
    }
}
