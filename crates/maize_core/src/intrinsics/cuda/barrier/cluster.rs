use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_argless_intrinsics},
};

impl_argless_intrinsics!(
    ClusterArrive: "llvm.nvvm.barrier.cluster.arrive"() -> VoidType,
    ClusterArriveRelaxed: "llvm.nvvm.barrier.cluster.arrive.relaxed"() ->VoidType,
    ClusterArriveAligned: "llvm.nvvm.barrier.cluster.arrive.aligned"() -> VoidType,
    ClusterArriveRelaxedAligned: "llvm.nvvm.barrier.cluster.arrive.relaxed.aligned"() -> VoidType,
    ClusterWait: "llvm.nvvm.barrier.cluster.wait"() -> VoidType,
    ClusterWaitAligned: "llvm.nvvm.barrier.cluster.wait.aligned"() -> VoidType,
);

impl CUDA {
    pub fn cluster_arrive(&self) {
        ClusterArrive(self.0.clone()).call(())
    }
    pub fn cluster_arrive_aligned(&self) {
        ClusterArriveAligned(self.0.clone()).call(())
    }
    pub fn cluster_arrive_relaxed(&self) {
        panic!("LLVM doesn't seem to support this instruction");
        #[expect(unused)]
        ClusterArriveRelaxed(self.0.clone()).call(())
    }
    pub fn cluster_arrive_relaxed_aligned(&self) {
        panic!("LLVM doesn't seem to support this instruction");
        #[expect(unused)]
        ClusterArriveRelaxedAligned(self.0.clone()).call(())
    }
    pub fn cluster_wait(&self) {
        ClusterWait(self.0.clone()).call(())
    }
    pub fn cluster_wait_aligned(&self) {
        ClusterWaitAligned(self.0.clone()).call(())
    }
}
