use std::mem::MaybeUninit;

use crate::{
    ContextRef, IntType, IntValue,
    backend::{UntypedValue, VoidType},
    func::FnRetTy,
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
    tipe::{A, Ty},
    val::Val,
};

pub struct Mbar(u64);
#[derive(Clone, Copy)]
pub struct MbarToken(u64);
#[derive(Clone, Copy)]
pub struct TimeLimitNs(u32);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MbarScope {
    Cta,
    Cluster,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MbarArriveOrdering {
    Release,
    Relaxed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MbarWaitOrdering {
    Acquire,
    Relaxed,
}

#[derive(Clone)]
pub enum MbarArrive {
    Thread,
    Count(Val<u32>),
    ExpectTx(Val<u32>),
}

impl MbarArrive {
    fn copy(&self) -> Self {
        self.clone()
    }
}

macro_rules! impl_mbar_aliases {
    ($tipe: ty => $inner: ty) => {
        impl Ty for $tipe {
            type LLType = IntType;
            type LLVal = IntValue;

            fn raw_ty(ctx: ContextRef) -> Self::LLType {
                <$inner>::raw_ty(ctx)
            }

            fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
                val.0.into_int_value()
            }

            fn const_val(self, fn_ref: crate::backend::FnRef) -> Val<Self>
            where
                Self: Sized,
            {
                let raw = Self::raw_ty(fn_ref.ctx()).const_int(self.0 as _, false);
                unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
            }
        }
    };
}

impl_mbar_aliases!(Mbar => u64);
impl_mbar_aliases!(MbarToken => u64);
impl_mbar_aliases!(TimeLimitNs => u32);

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

    MbarTryWaitParityScopeCta: "llvm.nvvm.mbarrier.try.wait.parity.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityScopeCluster: "llvm.nvvm.mbarrier.try.wait.parity.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityRelaxedScopeCta: "llvm.nvvm.mbarrier.try.wait.parity.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityRelaxedScopeCluster: "llvm.nvvm.mbarrier.try.wait.parity.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32) -> bool,
    MbarTryWaitParityTlScopeCta: "llvm.nvvm.mbarrier.try.wait.parity.tl.scope.cta.space.cta"(A<*mut u64, 3>, u32, u32) -> bool,
    MbarTryWaitParityTlScopeCluster: "llvm.nvvm.mbarrier.try.wait.parity.tl.scope.cluster.space.cta"(A<*mut u64, 3>, u32, u32) -> bool,
    MbarTryWaitParityTlRelaxedScopeCta: "llvm.nvvm.mbarrier.try.wait.parity.tl.relaxed.scope.cta.space.cta"(A<*mut u64, 3>, u32, u32) -> bool,
    MbarTryWaitParityTlRelaxedScopeCluster: "llvm.nvvm.mbarrier.try.wait.parity.tl.relaxed.scope.cluster.space.cta"(A<*mut u64, 3>, u32, u32) -> bool,

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

macro_rules! autowrite_exports {
    (single_func: $fn_name: ident($(post: $pp: expr,)? $(|$aname: ident: $tipe: ty| $aexpr: expr),*) -> $ret: ty: $intrinsic: ident $(: $post_process: expr)?) => {
        pub fn $fn_name($($aname: $tipe),*) -> $ret {
            $(($pp))?($intrinsic.call(($($aexpr,)*)))
        }
    };
    (($sig: tt -> $ret: ty $(: $post_process: expr)?): [$($wrapper_name: ident => $intrinsic: ident);* $(;)?]) => {
        $(
            autowrite_exports!(single_func: $wrapper_name$sig -> $ret: $intrinsic);
        )*
    };
}

pub fn mbar_init(
    mut raw_mem: Val<A<&'static mut MaybeUninit<Mbar>, 3>>,
    arrive_count: Val<u32>,
) -> Val<A<&'static mut Mbar, 3>> {
    MbarInit.call((raw_mem.reborrow().as_mut_ptr().ptr_cast(), arrive_count));
    let (fn_ref, value) = raw_mem.decompose();
    unsafe { Val::new(fn_ref, value) }
}

pub fn mbar_inval(mbar: Val<A<&'static mut Mbar, 3>>) {
    MbarInval.call((mbar.as_mut_ptr().ptr_cast(),))
}

pub fn mbar_test_wait(
    mbar: Val<A<&mut Mbar, 3>>,
    token: Val<MbarToken>,
    sem: MbarWaitOrdering,
    scope: MbarScope,
) -> Val<bool> {
    let (mbar, token) = (mbar.as_mut_ptr().ptr_cast(), token.bitcast());
    use MbarScope::{Cluster, Cta};
    use MbarWaitOrdering::{Acquire, Relaxed};
    match (sem, scope) {
        (Acquire, Cta) => MbarTestWaitScopeCta.call((mbar, token)),
        (Acquire, Cluster) => MbarTestWaitScopeCluster.call((mbar, token)),
        (Relaxed, Cta) => MbarTestWaitRelaxedScopeCta.call((mbar, token)),
        (Relaxed, Cluster) => MbarTestWaitRelaxedScopeCluster.call((mbar, token)),
    }
}

pub fn mbar_test_wait_parity(
    mbar: Val<A<&mut Mbar, 3>>,
    parity: Val<u32>,
    sem: MbarWaitOrdering,
    scope: MbarScope,
) -> Val<bool> {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    use MbarScope::{Cluster, Cta};
    use MbarWaitOrdering::{Acquire, Relaxed};
    match (sem, scope) {
        (Acquire, Cta) => MbarTestWaitParityScopeCta.call((mbar, parity)),
        (Acquire, Cluster) => MbarTestWaitParityScopeCluster.call((mbar, parity)),
        (Relaxed, Cta) => MbarTestWaitParityRelaxedScopeCta.call((mbar, parity)),
        (Relaxed, Cluster) => MbarTestWaitParityRelaxedScopeCluster.call((mbar, parity)),
    }
}

pub fn mbar_try_wait(
    mbar: Val<A<&mut Mbar, 3>>,
    token: Val<MbarToken>,
    sem: MbarWaitOrdering,
    scope: MbarScope,
    time_hint: Option<Val<TimeLimitNs>>,
) -> Val<bool> {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    let token = token.bitcast::<u64>();
    use MbarScope::{Cluster, Cta};
    use MbarWaitOrdering::{Acquire, Relaxed};
    if let Some(time_hint) = time_hint {
        let time_hint = time_hint.bitcast::<u32>();
        match (sem, scope) {
            (Acquire, Cta) => MbarTryWaitTlScopeCta.call((mbar, token, time_hint)),
            (Acquire, Cluster) => MbarTryWaitTlScopeCluster.call((mbar, token, time_hint)),
            (Relaxed, Cta) => MbarTryWaitTlRelaxedScopeCta.call((mbar, token, time_hint)),
            (Relaxed, Cluster) => MbarTryWaitTlRelaxedScopeCluster.call((mbar, token, time_hint)),
        }
    } else {
        match (sem, scope) {
            (Acquire, Cta) => MbarTryWaitScopeCta.call((mbar, token)),
            (Acquire, Cluster) => MbarTryWaitScopeCluster.call((mbar, token)),
            (Relaxed, Cta) => MbarTryWaitRelaxedScopeCta.call((mbar, token)),
            (Relaxed, Cluster) => MbarTryWaitRelaxedScopeCluster.call((mbar, token)),
        }
    }
}

pub fn mbar_try_wait_parity(
    mbar: Val<A<&mut Mbar, 3>>,
    parity: Val<u32>,
    sem: MbarWaitOrdering,
    scope: MbarScope,
    time_hint: Option<TimeLimitNs>,
) -> Val<bool> {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    use MbarScope::{Cluster, Cta};
    use MbarWaitOrdering::{Acquire, Relaxed};
    let Some(thint) = time_hint else {
        return match (sem, scope) {
            (Acquire, Cta) => MbarTryWaitParityScopeCta.call((mbar, parity)),
            (Acquire, Cluster) => MbarTryWaitParityScopeCluster.call((mbar, parity)),
            (Relaxed, Cta) => MbarTryWaitParityRelaxedScopeCta.call((mbar, parity)),
            (Relaxed, Cluster) => MbarTryWaitParityRelaxedScopeCluster.call((mbar, parity)),
        };
    };

    let thint = mbar.constant(thint).bitcast::<u32>();
    match (sem, scope) {
        (Acquire, Cta) => MbarTryWaitParityTlScopeCta.call((mbar, parity, thint)),
        (Acquire, Cluster) => MbarTryWaitParityTlScopeCluster.call((mbar, parity, thint)),
        (Relaxed, Cta) => MbarTryWaitParityTlRelaxedScopeCta.call((mbar, parity, thint)),
        (Relaxed, Cluster) => MbarTryWaitParityTlRelaxedScopeCluster.call((mbar, parity, thint)),
    }
}

pub fn mbar_arrive(
    mbar: Val<A<&mut Mbar, 3>>,
    arrive: MbarArrive,
    sem: MbarArriveOrdering,
    scope: MbarScope,
    drop: bool,
) -> Val<MbarToken> {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    use MbarArriveOrdering::{Relaxed as Rlx, Release as Rls};
    use MbarScope::{Cluster as Clu, Cta};

    let MbarArrive::ExpectTx(bytes) = arrive else {
        let count = match arrive {
            MbarArrive::Thread => mbar.constant(1),
            MbarArrive::Count(val) => val,
            MbarArrive::ExpectTx(val) => unreachable!(),
        };
        return match (sem, scope, drop) {
            (Rls, Cta, false) => MbarArriveScopeCta.call((mbar, count)),
            (Rls, Clu, false) => MbarArriveScopeCluster.call((mbar, count)),
            (Rlx, Cta, false) => MbarArriveRelaxedScopeCta.call((mbar, count)),
            (Rlx, Clu, false) => MbarArriveRelaxedScopeCluster.call((mbar, count)),

            (Rls, Cta, true) => MbarArriveDropScopeCta.call((mbar, count)),
            (Rls, Clu, true) => MbarArriveDropScopeCluster.call((mbar, count)),
            (Rlx, Cta, true) => MbarArriveDropRelaxedScopeCta.call((mbar, count)),
            (Rlx, Clu, true) => MbarArriveDropRelaxedScopeCluster.call((mbar, count)),
        }
        .bitcast::<MbarToken>();
    };

    match (sem, scope, drop) {
        (Rls, Cta, false) => MbarArriveExpectTxScopeCta.call((mbar, bytes)),
        (Rls, Clu, false) => MbarArriveExpectTxScopeCluster.call((mbar, bytes)),
        (Rlx, Cta, false) => MbarArriveExpectTxRelaxedScopeCta.call((mbar, bytes)),
        (Rlx, Clu, false) => MbarArriveExpectTxRelaxedScopeCluster.call((mbar, bytes)),

        (Rls, Cta, true) => MbarArriveDropExpectTxScopeCta.call((mbar, bytes)),
        (Rls, Clu, true) => MbarArriveDropExpectTxScopeCluster.call((mbar, bytes)),
        (Rlx, Cta, true) => MbarArriveDropExpectTxRelaxedScopeCta.call((mbar, bytes)),
        (Rlx, Clu, true) => MbarArriveDropExpectTxRelaxedScopeCluster.call((mbar, bytes)),
    }
    .bitcast::<MbarToken>()
}

pub fn cluster_mbar_arrive(
    mbar: Val<A<&mut Mbar, 7>>,
    arrive: MbarArrive,
    sem: MbarArriveOrdering,
    scope: MbarScope,
    drop: bool,
) {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    use MbarArriveOrdering::{Relaxed as Rlx, Release as Rls};
    use MbarScope::{Cluster as Clu, Cta};

    let MbarArrive::ExpectTx(bytes) = arrive else {
        let count = match arrive {
            MbarArrive::Thread => mbar.constant(1),
            MbarArrive::Count(val) => val,
            MbarArrive::ExpectTx(val) => unreachable!(),
        };
        return match (sem, scope, drop) {
            (Rls, Cta, false) => ClusterMbarArriveScopeCta.call((mbar, count)),
            (Rls, Clu, false) => ClusterMbarArriveScopeCluster.call((mbar, count)),
            (Rlx, Cta, false) => ClusterMbarArriveRelaxedScopeCta.call((mbar, count)),
            (Rlx, Clu, false) => ClusterMbarArriveRelaxedScopeCluster.call((mbar, count)),

            (Rls, Cta, true) => ClusterMbarArriveDropScopeCta.call((mbar, count)),
            (Rls, Clu, true) => ClusterMbarArriveDropScopeCluster.call((mbar, count)),
            (Rlx, Cta, true) => ClusterMbarArriveDropRelaxedScopeCta.call((mbar, count)),
            (Rlx, Clu, true) => ClusterMbarArriveDropRelaxedScopeCluster.call((mbar, count)),
        };
    };
    match (sem, scope, drop) {
        (Rls, Cta, false) => ClusterMbarArriveExpectTxScopeCta.call((mbar, bytes)),
        (Rls, Clu, false) => ClusterMbarArriveExpectTxScopeCluster.call((mbar, bytes)),
        (Rlx, Cta, false) => ClusterMbarArriveExpectTxRelaxedScopeCta.call((mbar, bytes)),
        (Rlx, Clu, false) => ClusterMbarArriveExpectTxRelaxedScopeCluster.call((mbar, bytes)),

        (Rls, Cta, true) => ClusterMbarArriveDropExpectTxScopeCta.call((mbar, bytes)),
        (Rls, Clu, true) => ClusterMbarArriveDropExpectTxScopeCluster.call((mbar, bytes)),
        (Rlx, Cta, true) => ClusterMbarArriveDropExpectTxRelaxedScopeCta.call((mbar, bytes)),
        (Rlx, Clu, true) => ClusterMbarArriveDropExpectTxRelaxedScopeCluster.call((mbar, bytes)),
    }
}

pub fn mbar_expect_tx(mbar: Val<A<&mut Mbar, 3>>, bytes: Val<u32>, scope: MbarScope) {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    match scope {
        MbarScope::Cta => MbarExpectTxScopeCta.call((mbar, bytes)),
        MbarScope::Cluster => MbarExpectTxScopeCluster.call((mbar, bytes)),
    }
}

pub fn mbar_complete_tx(mbar: Val<A<&mut Mbar, 3>>, bytes: Val<u32>, scope: MbarScope) {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    match scope {
        MbarScope::Cta => MbarCompleteTxScopeCta.call((mbar, bytes)),
        MbarScope::Cluster => MbarCompleteTxScopeCluster.call((mbar, bytes)),
    }
}

pub fn cluster_mbar_expect_tx(mbar: Val<A<&mut Mbar, 7>>, bytes: Val<u32>, scope: MbarScope) {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    match scope {
        MbarScope::Cta => ClusterMbarExpectTxScopeCta.call((mbar, bytes)),
        MbarScope::Cluster => ClusterMbarExpectTxScopeCluster.call((mbar, bytes)),
    }
}

pub fn cluster_mbar_complete_tx(mbar: Val<A<&mut Mbar, 7>>, bytes: Val<u32>, scope: MbarScope) {
    let mbar = mbar.as_mut_ptr().ptr_cast();
    match scope {
        MbarScope::Cta => ClusterMbarCompleteTxScopeCta.call((mbar, bytes)),
        MbarScope::Cluster => ClusterMbarCompleteTxScopeCluster.call((mbar, bytes)),
    }
}

mod test_mbar_codegen {
    use crate::{backend::LLVM, func::implement_ptx_kernel};

    use super::*;

    #[test]
    #[cfg(feature = "ptx-gen-tests")]
    fn test_mbar_ptx_gen() {
        for sem in ["Relaxed", "Standard"] {
            for scope in [MbarScope::Cta, MbarScope::Cluster] {
                for arrive_type in ["Thread", "Count", "ExpectTx"] {
                    for drop in [false, true] {
                        for time_hint in [None, Some(TimeLimitNs(500))] {
                            let ker =
                                implement_ptx_kernel::<(&mut bool,)>(LLVM::new(), "mbar_test");
                            {
                                let mbar_stor = ker.cuda().alloc_shared();
                                let mut mbar = mbar_init(mbar_stor, ker.constant(1));
                                let arrive = match arrive_type {
                                    "Thread" => MbarArrive::Thread,
                                    "Count" => MbarArrive::Count(ker.constant(5)),
                                    "ExpectTx" => MbarArrive::ExpectTx(ker.constant(1024)),
                                    _ => unreachable!(),
                                };
                                let arrive_sem = match sem {
                                    "Relaxed" => MbarArriveOrdering::Relaxed,
                                    "Standard" => MbarArriveOrdering::Release,
                                    _ => unreachable!(),
                                };
                                let wait_sem = match sem {
                                    "Relaxed" => MbarWaitOrdering::Relaxed,
                                    "Standard" => MbarWaitOrdering::Acquire,
                                    _ => unreachable!(),
                                };
                                let token = mbar_arrive(
                                    mbar.reborrow(),
                                    MbarArrive::Thread,
                                    arrive_sem,
                                    scope,
                                    drop,
                                );
                                let parity = ker.constant(1);
                                let is_done =
                                    mbar_test_wait(mbar.reborrow(), token.copy(), wait_sem, scope);
                                let is_done_parity = mbar_test_wait_parity(
                                    mbar.reborrow(),
                                    parity.copy(),
                                    wait_sem,
                                    scope,
                                );
                                let time_hint = time_hint.map(|v| ker.constant(v));
                                let is_done_waited = mbar_try_wait(
                                    mbar.reborrow(),
                                    token.copy(),
                                    wait_sem,
                                    scope,
                                    time_hint.clone(),
                                );
                                let is_done_waited_parity = mbar_try_wait_parity(
                                    mbar.reborrow(),
                                    parity.copy(),
                                    wait_sem,
                                    scope,
                                    time_hint.clone(),
                                );
                                let (mut ret,) = ker.args();
                                ret.store(is_done | is_done_parity | is_done_waited);
                                mbar_inval(mbar);
                            }
                            ker.return_void();
                            let bytes = ker.compile(crate::SM::SM100, crate::Opt::O0);
                            let string = str::from_utf8(&bytes).unwrap();

                            let arrive = if drop { "arrive_drop" } else { "arrive" };
                            let (scope, arrive_sem, wait_sem) = match (scope, sem) {
                                (MbarScope::Cta, "Standard") => ("", "", ""),
                                (MbarScope::Cta, "Relaxed") => (".cta", ".relaxed", ".relaxed"),
                                (MbarScope::Cluster, "Standard") => {
                                    (".cluster", ".release", ".acquire")
                                }
                                (MbarScope::Cluster, "Relaxed") => {
                                    (".cluster", ".relaxed", ".relaxed")
                                }
                                _ => unreachable!(),
                            };

                            assert!(string.contains("mbarrier.init.shared.b64 "), "{string}");
                            assert!(string.contains("mbarrier.inval.shared.b64 "), "{string}");
                            assert!(
                                string.contains(&format!(
                                    "mbarrier.{arrive}{arrive_sem}{scope}.shared.b64 "
                                )),
                                "{string}"
                            );
                            assert!(
                                string.contains(&format!(
                                    "mbarrier.test_wait{wait_sem}{scope}.shared.b64 "
                                )),
                                "{string}"
                            );
                            assert!(
                                string.contains(&format!(
                                    "mbarrier.test_wait.parity{wait_sem}{scope}.shared.b64 "
                                )),
                                "{string}"
                            );
                            assert!(
                                string.contains(&format!(
                                    "mbarrier.try_wait{wait_sem}{scope}.shared.b64 %p3, "
                                )),
                                "{string}"
                            );
                            assert!(
                                string.contains(&format!(
                                    "mbarrier.try_wait.parity{wait_sem}{scope}.shared.b64 "
                                )),
                                "{string}"
                            );
                        }
                    }
                }
            }
        }
    }
}
