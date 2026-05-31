use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
    tipe::A,
    val::Val,
};

pub struct MBar;

impl_intrinsics!(
    MbarArriveCta: "llvm.nvvm.mbarrier.arrive.shared"(A<*mut u64, 3>) -> u64,
    MbarArriveNoCompleteShared: "llvm.nvvm.mbarrier.arrive.noComplete.shared"(A<*mut u64, 3>, u32) -> u64,

    MbarInit: "llvm.nvvm.mbarrier.init.shared"(A<*mut u64, 3>, u32) -> VoidType,
    MbarInval: "llvm.nvvm.mbarrier.inval.shared"(A<*mut u64, 3>) -> VoidType,
    MbarPendingCount: "llvm.nvvm.mbarrier.pending.count"(u64) -> u32,

    MbarTestWait: "llvm.nvvm.mbarrier.test.wait.shared"(A<*mut u64, 3>, u64) -> bool,

    MbarTestWaitScopeCta: "llvm.nvvm.mbarrier.test.wait.scope.cta.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTestWaitScopeCluster: "llvm.nvvm.mbarrier.test.wait.scope.cluster.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTestWaitRelaxedScopeCta: "llvm.nvvm.mbarrier.test.wait.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTestWaitRelaxedScopeCluster: "llvm.nvvm.mbarrier.test.wait.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTestWaitParityScopeCta: "llvm.nvvm.mbarrier.test.wait.parity.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTestWaitParityScopeCluster: "llvm.nvvm.mbarrier.test.wait.parity.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTestWaitParityRelaxedScopeCta: "llvm.nvvm.mbarrier.test.wait.parity.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTestWaitParityRelaxedScopeCluster: "llvm.nvvm.mbarrier.test.wait.parity.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> bool,

    MbarTryWaitScopeCta: "llvm.nvvm.mbarrier.try.wait.scope.cta.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTryWaitScopeCluster: "llvm.nvvm.mbarrier.try.wait.scope.cluster.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTryWaitRelaxedScopeCta: "llvm.nvvm.mbarrier.try.wait.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTryWaitRelaxedScopeCluster: "llvm.nvvm.mbarrier.try.wait.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u64) -> bool,
    MbarTryWaitTlScopeCta: "llvm.nvvm.mbarrier.try.wait.tl.scope.cta.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,
    MbarTryWaitTlScopeCluster: "llvm.nvvm.mbarrier.try.wait.tl.scope.cluster.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,
    MbarTryWaitTlRelaxedScopeCta: "llvm.nvvm.mbarrier.try.wait.tl.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,
    MbarTryWaitTlRelaxedScopeCluster: "llvm.nvvm.mbarrier.try.wait.tl.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,

    MbarTryWaitParityScopeCta: "llvm.nvvm.mbarrier.try.wait.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityScopeCluster: "llvm.nvvm.mbarrier.try.wait.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityRelaxedScopeCta: "llvm.nvvm.mbarrier.try.wait.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityRelaxedScopeCluster: "llvm.nvvm.mbarrier.try.wait.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityTlScopeCta: "llvm.nvvm.mbarrier.try.wait.tl.scope.cta.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,
    MbarTryWaitParityTlScopeCluster: "llvm.nvvm.mbarrier.try.wait.tl.scope.cluster.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,
    MbarTryWaitParityTlRelaxedScopeCta: "llvm.nvvm.mbarrier.try.wait.tl.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,
    MbarTryWaitParityTlRelaxedScopeCluster: "llvm.nvvm.mbarrier.try.wait.tl.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u64, u32) -> bool,

    // None of these work in practice! LLVM bugs

    MbarCompleteTxScopeCta: "llvm.nvvm.mbarrier.complete.tx.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> VoidType,
    MbarCompleteTxScopeCluster: "llvm.nvvm.mbarrier.complete.tx.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> VoidType,
    ClusterMbarCompleteTxScopeCta: "llvm.nvvm.mbarrier.complete.tx.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarCompleteTxScopeCluster: "llvm.nvvm.mbarrier.complete.tx.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarExpectTxScopeCta: "llvm.nvvm.mbarrier.expect.tx.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> VoidType,
    MbarExpectTxScopeCluster: "llvm.nvvm.mbarrier.expect.tx.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> VoidType,
    ClusterMbarExpectTxScopeCta: "llvm.nvvm.mbarrier.expect.tx.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarExpectTxScopeCluster: "llvm.nvvm.mbarrier.expect.tx.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveExpectTxScopeCta: "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveExpectTxScopeCluster: "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveExpectTxScopeCta: "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveExpectTxScopeCluster: "llvm.nvvm.mbarrier.arrive.expect.tx.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveExpectTxRelaxedScopeCta: "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveExpectTxRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveExpectTxRelaxedScopeCta: "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveExpectTxRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.expect.tx.relaxed.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveScopeCta: "llvm.nvvm.mbarrier.arrive.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveScopeCluster: "llvm.nvvm.mbarrier.arrive.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveScopeCluster: "llvm.nvvm.mbarrier.arrive.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveScopeCta:  "llvm.nvvm.mbarrier.arrive.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveRelaxedScopeCta: "llvm.nvvm.mbarrier.arrive.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.relaxed.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveRelaxedScopeCta:  "llvm.nvvm.mbarrier.arrive.relaxed.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveDropExpectTxScopeCta: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveDropExpectTxScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveDropExpectTxScopeCta: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveDropExpectTxScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveDropExpectTxRelaxedScopeCta: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveDropExpectTxRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveDropExpectTxRelaxedScopeCta: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveDropExpectTxRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.expect.tx.relaxed.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveDropScopeCta: "llvm.nvvm.mbarrier.arrive.drop.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveDropScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveDropScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveDropScopeCta:  "llvm.nvvm.mbarrier.arrive.drop.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,

    MbarArriveDropRelaxedScopeCta: "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> u64,
    MbarArriveDropRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> u64,
    ClusterMbarArriveDropRelaxedScopeCluster: "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cluster.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
    ClusterMbarArriveDropRelaxedScopeCta:  "llvm.nvvm.mbarrier.arrive.drop.relaxed.scope.cta.space.cluster"(A<*mut u64, 7>, u32) -> VoidType,
);

impl CUDA {
    pub fn mbar_arrive(mbar: Val<A<*mut u64, 3>>) -> Val<u64> {
        MbarArriveCta.call((mbar,))
    }
    pub fn mbar_arrive_nocomplete(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
        MbarArriveNoCompleteShared.call((mbar, count))
    }
    pub fn mbar_init(mbar: Val<A<*mut u64, 3>>, thread_count: Val<u32>) {
        MbarInit.call((mbar, thread_count))
    }
    pub fn mbar_inval(mbar: Val<A<*mut u64, 3>>) {
        MbarInval.call((mbar,))
    }
    pub fn mbar_pending_count(mbar: Val<A<*mut u64, 3>>) -> Val<u32> {
        MbarPendingCount.call((mbar.as_u64(),))
    }
    pub fn mbar_test_wait(mbar: Val<A<*mut u64, 3>>, token: Val<u64>) -> Val<bool> {
        MbarTestWaitScopeCta.call((mbar, token))
    }
    pub fn mbar_test_wait_parity(mbar: Val<A<*mut u64, 3>>, phase: Val<bool>) -> Val<bool> {
        MbarTestWaitParityScopeCta.call((mbar, phase.cvt()))
    }
    pub fn mbar_try_wait(mbar: Val<A<*mut u64, 3>>, token: Val<u64>) -> Val<bool> {
        MbarTryWaitScopeCta.call((mbar, token))
    }
    pub fn mbar_try_wait_timed(
        mbar: Val<A<*mut u64, 3>>,
        token: Val<u64>,
        time_hint: Val<u32>,
    ) -> Val<bool> {
        MbarTryWaitTlScopeCta.call((mbar, token, time_hint))
    }
    pub fn mbar_try_wait_parity(mbar: Val<A<*mut u64, 3>>, phase: Val<bool>) -> Val<bool> {
        MbarTryWaitParityScopeCta.call((mbar, phase.cvt()))
    }
    pub fn mbar_try_wait_parity_timed(
        mbar: Val<A<*mut u64, 3>>,
        phase: Val<bool>,
        time_hint: Val<u32>,
    ) -> Val<bool> {
        MbarTryWaitParityTlScopeCta.call((mbar, phase.cvt(), time_hint))
    }

    // These crash LLVM! but we *really* want them to be top-level accessible
    // when it eventually works

    pub fn mbar_complete_tx(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>) {
        mbar_complete_tx_scope_cta(mbar, tx_bytes)
    }
    pub fn cluster_mbar_complete_tx(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
        cluster_mbar_complete_tx_scope_cluster(mbar, tx_bytes)
    }

    pub fn mbar_expect_tx(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>) {
        mbar_expect_tx_scope_cta(mbar, tx_bytes)
    }
    pub fn cluster_mbar_expect_tx(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
        cluster_mbar_expect_tx_scope_cluster(mbar, tx_bytes)
    }

    pub fn cluster_mbar_arrive(mbar: Val<A<*mut u64, 7>>) {
        let count = mbar.fn_ref().constant(1);
        ClusterMbarArriveScopeCluster.call((mbar, count))
    }
    pub fn mbar_arrive_expect_tx(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>) -> Val<u64> {
        MbarArriveExpectTxScopeCta.call((mbar, tx_bytes))
    }
    pub fn cluster_mbar_arrive_expect_tx(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
        ClusterMbarArriveExpectTxScopeCluster.call((mbar, tx_bytes))
    }
}

#[rustfmt::skip]
// These more or less all crash LLVM!
// But we want to leave a comprehensive API available
pub mod exports {
use super::*;
// mbarrier.complete_tx.*
pub fn mbar_complete_tx_scope_cta(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>) {
    MbarCompleteTxScopeCta.call((mbar, tx_bytes))
}
pub fn mbar_complete_tx_scope_cluster(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>)  {
    MbarCompleteTxScopeCluster.call((mbar, tx_bytes))
}
pub fn cluster_mbar_complete_tx_scope_cta(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
    ClusterMbarCompleteTxScopeCta.call((mbar, tx_bytes))
}
pub fn cluster_mbar_complete_tx_scope_cluster(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
    ClusterMbarCompleteTxScopeCluster.call((mbar, tx_bytes))
}

// mbarrier.expect_tx.*
pub fn mbar_expect_tx_scope_cta(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>) {
    MbarExpectTxScopeCta.call((mbar, tx_bytes))
}
pub fn mbar_expect_tx_scope_cluster(mbar: Val<A<*mut u64, 3>>, tx_bytes: Val<u32>) {
    MbarExpectTxScopeCluster.call((mbar, tx_bytes))
}
pub fn cluster_mbar_expect_tx_scope_cta(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
    ClusterMbarExpectTxScopeCta.call((mbar, tx_bytes))
}
pub fn cluster_mbar_expect_tx_scope_cluster(mbar: Val<A<*mut u64, 7>>, tx_bytes: Val<u32>) {
    ClusterMbarExpectTxScopeCluster.call((mbar, tx_bytes))
}

// mbarrier.arrive.*
pub fn mbar_arrive_expect_tx_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveExpectTxScopeCta.call((mbar, count))
}
pub fn mbar_arrive_expect_tx_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveExpectTxScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_expect_tx_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveExpectTxScopeCta.call((mbar, count))
}
pub fn cluster_mbar_arrive_expect_tx_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveExpectTxScopeCluster.call((mbar, count))
}
pub fn mbar_arrive_expect_tx_relaxed_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveExpectTxRelaxedScopeCta.call((mbar, count))
}
pub fn mbar_arrive_expect_tx_relaxed_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveExpectTxRelaxedScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_expect_tx_relaxed_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveExpectTxRelaxedScopeCta.call((mbar, count))
}
pub fn cluster_mbar_arrive_expect_tx_relaxed_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveExpectTxRelaxedScopeCluster.call((mbar, count))
}
pub fn mbar_arrive_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveScopeCta.call((mbar, count))
}
pub fn mbar_arrive_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveScopeCta.call((mbar, count))
}
pub fn mbar_arrive_relaxed_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveRelaxedScopeCta.call((mbar, count))
}
pub fn mbar_arrive_relaxed_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveRelaxedScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_relaxed_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveRelaxedScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_relaxed_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveRelaxedScopeCta.call((mbar, count))
}
pub fn mbar_arrive_drop_expect_tx_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropExpectTxScopeCta.call((mbar, count))
}
pub fn mbar_arrive_drop_expect_tx_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropExpectTxScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_expect_tx_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropExpectTxScopeCta.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_expect_tx_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropExpectTxScopeCluster.call((mbar, count))
}
pub fn mbar_arrive_drop_expect_tx_relaxed_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropExpectTxRelaxedScopeCta.call((mbar, count))
}
pub fn mbar_arrive_drop_expect_tx_relaxed_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropExpectTxRelaxedScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_expect_tx_relaxed_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropExpectTxRelaxedScopeCta.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_expect_tx_relaxed_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropExpectTxRelaxedScopeCluster.call((mbar, count))
}
pub fn mbar_arrive_drop_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropScopeCta.call((mbar, count))
}
pub fn mbar_arrive_drop_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropScopeCta.call((mbar, count))
}
pub fn mbar_arrive_drop_relaxed_scope_cta(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropRelaxedScopeCta.call((mbar, count))
}
pub fn mbar_arrive_drop_relaxed_scope_cluster(mbar: Val<A<*mut u64, 3>>, count: Val<u32>) -> Val<u64> {
    MbarArriveDropRelaxedScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_relaxed_scope_cluster(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropRelaxedScopeCluster.call((mbar, count))
}
pub fn cluster_mbar_arrive_drop_relaxed_scope_cta(mbar: Val<A<*mut u64, 7>>, count: Val<u32>) {
    ClusterMbarArriveDropRelaxedScopeCta.call((mbar, count))
}
}

pub use exports::*;
