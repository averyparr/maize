use crate::{
    backend::{LLVM, Opt, VoidType, cpu::cuda::SM},
    func::implement_ptx_kernel,
    intrinsics::{
        Intrinsic,
        cuda::{CUDA, cache},
        impl_argless_intrinsics, impl_intrinsics,
    },
    tipe::A,
    val::Val,
};

impl_argless_intrinsics!(
  CpAsyncBulkCommitGroup: "llvm.nvvm.cp.async.bulk.commit.group"() -> VoidType
);

impl_intrinsics!(
    CpAsyncBulkG2C: "llvm.nvvm.cp.async.bulk.global.to.shared.cluster"(A<*mut u8, 7>, A<*mut u64, 3>, A<*const u8, 1>, u32, u16, u64, bool, bool) -> VoidType,
    CpAsyncBulkG2S: "llvm.nvvm.cp.async.bulk.global.to.shared.cta"(A<*mut u8, 3>, A<*mut u64, 3>, A<*const u8, 1>, u32, u64, bool) -> VoidType,
    CpAsyncBulkS2C: "llvm.nvvm.cp.async.bulk.shared.cta.to.cluster"(A<*mut u8, 7>, A<*mut u64, 3>, A<*const u8, 3>, u32) -> VoidType,
    CpAsyncBulkS2G: "llvm.nvvm.cp.async.bulk.shared.cta.to.global"(A<*mut u8, 1>, A<*const u8, 3>, u32, u64, bool) -> VoidType,
);

impl CUDA {
    pub fn cp_async_bulk_commit_group(&self) {
        CpAsyncBulkCommitGroup(self.0.clone()).call(())
    }
}

pub fn cp_async_bulk_g2c(
    cluster_ptr: Val<A<*mut u8, 7>>,
    mbar: Val<A<&mut u64, 3>>,
    global_ptr: Val<A<*const u8, 1>>,
    num_bytes: Val<u32>,
    mcast_mask: Option<Val<u16>>,
    cache_hint: Option<Val<u64>>,
) {
    let fn_ref = cluster_ptr.fn_ref();
    let uses_mcast = fn_ref.constant(mcast_mask.is_some());
    let uses_cache_hint = fn_ref.constant(cache_hint.is_some());
    let mcast_mask = mcast_mask.unwrap_or(fn_ref.constant(0));
    let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));

    CpAsyncBulkG2C.call((
        cluster_ptr,
        mbar.as_mut_ptr(),
        global_ptr,
        num_bytes,
        mcast_mask,
        cache_hint,
        uses_mcast,
        uses_cache_hint,
    ))
}

pub fn cp_async_bulk_g2s(
    shared_ptr: Val<A<*mut u8, 3>>,
    mbar: Val<A<&mut u64, 3>>,
    global_ptr: Val<A<*const u8, 1>>,
    num_bytes: Val<u32>,
    cache_hint: Option<Val<u64>>,
) {
    let fn_ref = shared_ptr.fn_ref();
    let uses_cache_hint = fn_ref.constant(cache_hint.is_some());
    let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));

    CpAsyncBulkG2S.call((
        shared_ptr,
        mbar.as_mut_ptr(),
        global_ptr,
        num_bytes,
        cache_hint,
        uses_cache_hint,
    ))
}

pub fn cp_async_bulk_s2c(
    cluster_ptr: Val<A<*mut u8, 7>>,
    mbar: Val<A<&mut u64, 3>>,
    shared_ptr: Val<A<*const u8, 3>>,
    num_bytes: Val<u32>,
) {
    CpAsyncBulkS2C.call((cluster_ptr, mbar.as_mut_ptr(), shared_ptr, num_bytes))
}

pub fn cp_async_bulk_s2g(
    global_ptr: Val<A<*mut u8, 1>>,
    shared_ptr: Val<A<*const u8, 3>>,
    num_bytes: Val<u32>,
    cache_hint: Option<Val<u64>>,
) {
    let fn_ref = global_ptr.fn_ref();
    let has_cache_hint = fn_ref.constant(cache_hint.is_some());
    let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));
    CpAsyncBulkS2G.call((
        global_ptr,
        shared_ptr,
        num_bytes,
        cache_hint,
        has_cache_hint,
    ))
}

macro_rules! impl_test_for_func {
    ($name: ident: $func_invocation: expr => $ptx_string: literal) => {
        #[test]
        fn $name() {
            let ker = implement_ptx_kernel::<(A<*mut u8, 1>, u32)>(LLVM::new(), "test_ker");
            {
                let (global_mut, num_bytes) = ker.args();
                let mbar = ker.cuda().alloc_shared::<u64>();
                let smem_mut = ker.cuda().alloc_shared::<u8>().as_mut_ptr();
                let smem_const = smem_mut.copy().as_const();
                let mcast_mask = ker.constant(0b11);
                let cache_hint = ker.constant(0b11);
                let global_const = global_mut.copy().as_const();
                let cluster_mut = smem_mut.copy().map_address_to_cluster(ker.constant(0));
                let cluster_const = smem_const.copy().map_address_to_cluster(ker.constant(0));
                $func_invocation(
                    global_mut,
                    global_const,
                    mbar,
                    smem_mut,
                    smem_const,
                    cluster_mut,
                    cluster_const,
                    num_bytes,
                    mcast_mask,
                    cache_hint,
                );
            }
            ker.return_void();
            let bytes = ker.compile($crate::SM::SM100, $crate::Opt::O0);
            let string = str::from_utf8(&bytes).unwrap();
            assert!(string.contains($ptx_string), "{string}");
        }
    };
}

#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! impl_test_for_func {
    ($($t: tt)*) => {};
}

#[cfg(test)]
mod test_cp_async_bulk {
    use super::*;
    impl_test_for_func!(g2s:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_g2s(smem_mut, mbar, global_const, num_bytes, None)
    => "cp.async.bulk.shared::cta.global.mbarrier::complete_tx::bytes [smem_$_1], [%rd1], %r1, [smem];");

    impl_test_for_func!(g2s_cache_hint:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_g2s(smem_mut, mbar, global_const, num_bytes, Some(cache_hint))
    => "cp.async.bulk.shared::cta.global.mbarrier::complete_tx::bytes.L2::cache_hint [smem_$_1], [%rd1], %r1, [smem], %rd2;");

    impl_test_for_func!(g2c:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_g2c(cluster_mut, mbar, global_const, num_bytes, None, None)
    => "cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes [%rd3], [%rd1], %r1, [smem];");

    impl_test_for_func!(g2c_mcast:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_g2c(cluster_mut, mbar, global_const, num_bytes, Some(mcast_mask), None)
    => "cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes.multicast::cluster [%rd3], [%rd1], %r1, [smem], %rs1;");

    impl_test_for_func!(g2c_cache_hint:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_g2c(cluster_mut, mbar, global_const, num_bytes, None, Some(cache_hint))
    => "cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes.L2::cache_hint [%rd3], [%rd1], %r1, [smem], %rd4;");

    impl_test_for_func!(g2c_mcast_cache_hint:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_g2c(cluster_mut, mbar, global_const, num_bytes, Some(mcast_mask), Some(cache_hint))
    => "cp.async.bulk.shared::cluster.global.mbarrier::complete_tx::bytes.multicast::cluster.L2::cache_hint [%rd3], [%rd1], %r1, [smem], %rs1, %rd4;");

    impl_test_for_func!(s2c:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_s2c(cluster_mut, mbar, smem_const, num_bytes)
    => "cp.async.bulk.shared::cluster.shared::cta.mbarrier::complete_tx::bytes [%rd2], [smem_$_1], %r1, [smem];");

    impl_test_for_func!(s2g:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_s2g(global_mut, smem_const, num_bytes, None)
    => "cp.async.bulk.global.shared::cta.bulk_group [%rd1], [smem_$_1], %r1;");

    impl_test_for_func!(s2g_cache_hint:
    |
        global_mut,
        global_const,
        mbar,
        smem_mut,
        smem_const,
        cluster_mut,
        cluster_const,
        num_bytes,
        mcast_mask,
        cache_hint,
    |
    cp_async_bulk_s2g(global_mut, smem_const, num_bytes, Some(cache_hint))
    => "cp.async.bulk.global.shared::cta.bulk_group.L2::cache_hint [%rd1], [smem_$_1], %r1, %rd2;");
}
