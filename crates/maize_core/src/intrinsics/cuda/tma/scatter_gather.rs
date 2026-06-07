use crate::{
    backend::VoidType,
    intrinsics::{
        Intrinsic,
        cuda::{mma::tcgen05::TcGen05CTA, tma::TensorMap},
        impl_intrinsics,
    },
    tipe::A,
    val::Val,
};

impl_intrinsics!(
    CpAsyncBulkScatter4S2G: "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.scatter4.2d"(A<*const u8, 3>, *const TensorMap, u32, u32, u32, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkGather4G2S: "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.gather4.2d"(A<*mut u8, 3>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkGather4G2C: "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.gather4.2d"(A<*mut u8, 7>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u32, u32, u16, u64, bool, bool, u32) -> VoidType,
);

pub fn cp_async_bulk_scatter4(
    src: Val<A<*const u8, 3>>,
    tensormap: Val<*const TensorMap>,
    x: Val<u32>,
    y0: Val<u32>,
    y1: Val<u32>,
    y2: Val<u32>,
    y3: Val<u32>,
    cache_hint: Option<Val<u64>>,
) {
    let fn_ref = src.fn_ref();
    let use_cache_hint = fn_ref.constant(cache_hint.is_some());
    let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));
    CpAsyncBulkScatter4S2G.call((
        src,
        tensormap,
        x,
        y0,
        y1,
        y2,
        y3,
        cache_hint,
        use_cache_hint,
    ))
}

pub fn cp_async_bulk_gather4_g2s(
    dst: Val<A<*mut u8, 3>>,
    mbar: Val<A<&mut u64, 3>>,
    tensormap: Val<*const TensorMap>,
    x: Val<u32>,
    y0: Val<u32>,
    y1: Val<u32>,
    y2: Val<u32>,
    y3: Val<u32>,
    cache_hint: Option<Val<u64>>,
) {
    let fn_ref = dst.fn_ref();
    let use_cache_hint = fn_ref.constant(cache_hint.is_some());
    let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));
    CpAsyncBulkGather4G2S.call((
        dst,
        mbar.as_mut_ptr(),
        tensormap,
        x,
        y0,
        y1,
        y2,
        y3,
        cache_hint,
        use_cache_hint,
    ))
}

pub fn cp_async_bulk_gather4_g2c(
    dst: Val<A<*mut u8, 7>>,
    mbar: Val<A<&mut u64, 3>>,
    tensormap: Val<*const TensorMap>,
    x: Val<u32>,
    y0: Val<u32>,
    y1: Val<u32>,
    y2: Val<u32>,
    y3: Val<u32>,
    mcast_mask: Option<Val<u16>>,
    cache_hint: Option<Val<u64>>,
    cta_group: Option<TcGen05CTA>,
) {
    let fn_ref = dst.fn_ref();
    let use_mcast = fn_ref.constant(mcast_mask.is_some());
    let use_cache_hint = fn_ref.constant(cache_hint.is_some());
    let mcast_mask = mcast_mask.unwrap_or(fn_ref.constant(0));
    let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));
    let cta_group = fn_ref.constant(cta_group.map(|v| v as _).unwrap_or(0));
    CpAsyncBulkGather4G2C.call((
        dst,
        mbar.as_mut_ptr(),
        tensormap,
        x,
        y0,
        y1,
        y2,
        y3,
        mcast_mask,
        cache_hint,
        use_mcast,
        use_cache_hint,
        cta_group,
    ))
}

#[cfg(test)]
mod test {
    use crate::{backend::LLVM, func::implement_ptx_kernel};

    use super::*;

    #[test]
    #[cfg(feature = "ptx-gen-tests")]
    fn gather4_g2c_codegen() {
        for cta_group in [None, Some(TcGen05CTA::OneSM), Some(TcGen05CTA::TwoSM)] {
            for mcast_mask in [None, Some(0b11)] {
                for cache_hint in [None, Some(512)] {
                    let ker = implement_ptx_kernel::<(*const TensorMap, u32, u32, u32, u32, u32)>(
                        LLVM::new(),
                        "test_gather4_g2c",
                    );
                    {
                        let (tensormap, x, y0, y1, y2, y3) = ker.args();
                        let mbar = ker.cuda().alloc_shared();
                        let dst = ker.cuda().alloc_shared().as_mut_ptr();
                        let dst = dst.map_address_to_cluster(ker.constant(3));
                        let mcast_mask = mcast_mask.map(|v| ker.constant(v));
                        let cache_hint = cache_hint.map(|v| ker.constant(v));
                        cp_async_bulk_gather4_g2c(
                            dst, mbar, tensormap, x, y0, y1, y2, y3, mcast_mask, cache_hint,
                            cta_group,
                        );
                    }
                    ker.return_void();
                    let bytes = ker.compile(crate::SM::SM100a, crate::Opt::O0);
                    let string = str::from_utf8(&bytes).unwrap();
                    let mcast = if mcast_mask.is_some() {
                        ".multicast::cluster"
                    } else {
                        ""
                    };
                    let cache_hint = if cache_hint.is_some() {
                        ".L2::cache_hint"
                    } else {
                        ""
                    };
                    let cta_group = match cta_group {
                        None => "",
                        Some(TcGen05CTA::OneSM) => ".cta_group::1",
                        Some(TcGen05CTA::TwoSM) => ".cta_group::2",
                    };

                    assert!(string.contains(&format!("cp.async.bulk.tensor.2d.shared::cluster.global.tile::gather4.mbarrier::complete_tx::bytes{}{}{} ", mcast, cache_hint, cta_group)), "{string}");
                }
            }
        }
    }

    #[test]
    #[cfg(feature = "ptx-gen-tests")]
    fn gather4_g2s_codegen() {
        for cache_hint in [None, Some(512)] {
            let ker = implement_ptx_kernel::<(*const TensorMap, u32, u32, u32, u32, u32)>(
                LLVM::new(),
                "test_gather4_g2s",
            );
            {
                let (tensormap, x, y0, y1, y2, y3) = ker.args();
                let mbar = ker.cuda().alloc_shared();
                let dst = ker.cuda().alloc_shared().as_mut_ptr();
                let cache_hint = cache_hint.map(|v| ker.constant(v));
                cp_async_bulk_gather4_g2s(dst, mbar, tensormap, x, y0, y1, y2, y3, cache_hint);
            }
            ker.return_void();
            let bytes = ker.compile(crate::SM::SM100a, crate::Opt::O0);
            let string = str::from_utf8(&bytes).unwrap();
            let cache_hint = if cache_hint.is_some() {
                ".L2::cache_hint"
            } else {
                ""
            };

            assert!(string.contains(&format!("cp.async.bulk.tensor.2d.shared::cta.global.tile::gather4.mbarrier::complete_tx::bytes{} ",  cache_hint)), "{string}");
        }
    }

    #[test]
    #[cfg(feature = "ptx-gen-tests")]
    fn scatter4_s2g_codegen() {
        for cache_hint in [None, Some(512)] {
            let ker = implement_ptx_kernel::<(*const TensorMap, u32, u32, u32, u32, u32)>(
                LLVM::new(),
                "test_scatter4_s2g",
            );
            {
                let (tensormap, x, y0, y1, y2, y3) = ker.args();
                let src = ker.cuda().alloc_shared().as_ptr();
                let cache_hint = cache_hint.map(|v| ker.constant(v));
                cp_async_bulk_scatter4(src, tensormap, x, y0, y1, y2, y3, cache_hint);
            }
            ker.return_void();
            let bytes = ker.compile(crate::SM::SM100a, crate::Opt::O0);
            let string = str::from_utf8(&bytes).unwrap();
            let cache_hint = if cache_hint.is_some() {
                ".L2::cache_hint"
            } else {
                ""
            };

            assert!(
                string.contains(&format!(
                    "cp.async.bulk.tensor.2d.global.shared::cta.tile::scatter4.bulk_group{} ",
                    cache_hint
                )),
                "{string}"
            );
        }
    }
}
