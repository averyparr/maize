use inkwell::{context::ContextRef, types::StructType, values::StructValue};

use crate::{
    backend::VoidType,
    intrinsics::cuda::tma::TensorMap,
    intrinsics::{Intrinsic, cuda::mma::tcgen05::TcGen05CTA, impl_intrinsics},
    tipe::{A, Ty},
    val::Val,
};

impl_intrinsics!(
    CpAsyncBulkG2CTile1D: "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.1d"(A<*mut u8, 7>, A<*mut u64, 3>, *const TensorMap, u32, u16, u64, bool, bool, u32) -> VoidType,
    CpAsyncBulkG2CTile2D: "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.2d"(A<*mut u8, 7>, A<*mut u64, 3>, *const TensorMap, u32, u32, u16, u64, bool, bool, u32) -> VoidType,
    CpAsyncBulkG2CTile3D: "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.3d"(A<*mut u8, 7>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u16, u64, bool, bool, u32) -> VoidType,
    CpAsyncBulkG2CTile4D: "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.4d"(A<*mut u8, 7>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u32, u16, u64, bool, bool, u32) -> VoidType,
    CpAsyncBulkG2CTile5D: "llvm.nvvm.cp.async.bulk.tensor.g2s.tile.5d"(A<*mut u8, 7>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u32, u32, u16, u64, bool, bool, u32) -> VoidType,

    CpAsyncBulkG2STile1D: "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.1d"(A<*mut u8, 3>, A<*mut u64, 3>, *const TensorMap, u32, u64, bool) -> VoidType,
    CpAsyncBulkG2STile2D: "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.2d"(A<*mut u8, 3>, A<*mut u64, 3>, *const TensorMap, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkG2STile3D: "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.3d"(A<*mut u8, 3>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkG2STile4D: "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.4d"(A<*mut u8, 3>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkG2STile5D: "llvm.nvvm.cp.async.bulk.tensor.g2s.cta.tile.5d"(A<*mut u8, 3>, A<*mut u64, 3>, *const TensorMap, u32, u32, u32, u32, u32, u64, bool) -> VoidType,

    CpAsyncBulkS2GTile1D: "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.1d"(A<*const u8, 3>, *const TensorMap, u32, u64, bool) -> VoidType,
    CpAsyncBulkS2GTile2D: "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.2d"(A<*const u8, 3>, *const TensorMap, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkS2GTile3D: "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.3d"(A<*const u8, 3>, *const TensorMap, u32, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkS2GTile4D: "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.4d"(A<*const u8, 3>, *const TensorMap, u32, u32, u32, u32, u64, bool) -> VoidType,
    CpAsyncBulkS2GTile5D: "llvm.nvvm.cp.async.bulk.tensor.s2g.tile.5d"(A<*const u8, 3>, *const TensorMap, u32, u32, u32, u32, u32, u64, bool) -> VoidType,
);

macro_rules! impl_mdim_cp_async_bulk_tile_g2c {
    ($name: ident => $($crds: ident),* => $intrins: ident) => {
    pub fn $name(
        dst: Val<A<*mut u8, 7>>,
        mbar: Val<A<&mut u64, 3>>,
        tensormap: Val<*const TensorMap>,
        $($crds: Val<u32>,)*
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
        $intrins.call((
            dst,
            mbar.as_mut_ptr(),
            tensormap,
            $($crds,)*
            mcast_mask,
            cache_hint,
            use_mcast,
            use_cache_hint,
            cta_group,
        ))
    }
    };
}

macro_rules! impl_mdim_cp_async_bulk_tile_g2s {
    ($name: ident => $($crds: ident),* => $intrins: ident) => {
    pub fn $name(
        dst: Val<A<*mut u8, 3>>,
        mbar: Val<A<&mut u64, 3>>,
        tensormap: Val<*const TensorMap>,
        $($crds: Val<u32>,)*
        cache_hint: Option<Val<u64>>,
    ) {
        let fn_ref = dst.fn_ref();
        let use_cache_hint = fn_ref.constant(cache_hint.is_some());
        let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));
        $intrins.call((
            dst,
            mbar.as_mut_ptr(),
            tensormap,
            $($crds,)*
            cache_hint,
            use_cache_hint,
        ))
    }
    };
}

macro_rules! impl_mdim_cp_async_bulk_tile_s2g {
    ($name: ident => $($crds: ident),* => $intrins: ident) => {
    pub fn $name(
        src: Val<A<*const u8, 3>>,
        tensormap: Val<*const TensorMap>,
        $($crds: Val<u32>,)*
        cache_hint: Option<Val<u64>>,
    ) {
        let fn_ref = src.fn_ref();
        let use_cache_hint = fn_ref.constant(cache_hint.is_some());
        let cache_hint = cache_hint.unwrap_or(fn_ref.constant(0));
        $intrins.call((
            src,
            tensormap,
            $($crds,)*
            cache_hint,
            use_cache_hint,
        ))
    }
    };
}

impl_mdim_cp_async_bulk_tile_g2c!(cp_async_bulk_tile_g2c_1d => c0 => CpAsyncBulkG2CTile1D);
impl_mdim_cp_async_bulk_tile_g2c!(cp_async_bulk_tile_g2c_2d => c0, c1 => CpAsyncBulkG2CTile2D);
impl_mdim_cp_async_bulk_tile_g2c!(cp_async_bulk_tile_g2c_3d => c0, c1, c2 => CpAsyncBulkG2CTile3D);
impl_mdim_cp_async_bulk_tile_g2c!(cp_async_bulk_tile_g2c_4d => c0, c1, c2, c3 => CpAsyncBulkG2CTile4D);
impl_mdim_cp_async_bulk_tile_g2c!(cp_async_bulk_tile_g2c_5d => c0, c1, c2, c3, c4 => CpAsyncBulkG2CTile5D);

impl_mdim_cp_async_bulk_tile_g2s!(cp_async_bulk_tile_g2s_1d => c0 => CpAsyncBulkG2STile1D);
impl_mdim_cp_async_bulk_tile_g2s!(cp_async_bulk_tile_g2s_2d => c0, c1 => CpAsyncBulkG2STile2D);
impl_mdim_cp_async_bulk_tile_g2s!(cp_async_bulk_tile_g2s_3d => c0, c1, c2 => CpAsyncBulkG2STile3D);
impl_mdim_cp_async_bulk_tile_g2s!(cp_async_bulk_tile_g2s_4d => c0, c1, c2, c3 => CpAsyncBulkG2STile4D);
impl_mdim_cp_async_bulk_tile_g2s!(cp_async_bulk_tile_g2s_5d => c0, c1, c2, c3, c4 => CpAsyncBulkG2STile5D);

impl_mdim_cp_async_bulk_tile_s2g!(cp_async_bulk_tile_s2g_1d => c0 => CpAsyncBulkS2GTile1D);
impl_mdim_cp_async_bulk_tile_s2g!(cp_async_bulk_tile_s2g_2d => c0, c1 => CpAsyncBulkS2GTile2D);
impl_mdim_cp_async_bulk_tile_s2g!(cp_async_bulk_tile_s2g_3d => c0, c1, c2 => CpAsyncBulkS2GTile3D);
impl_mdim_cp_async_bulk_tile_s2g!(cp_async_bulk_tile_s2g_4d => c0, c1, c2, c3 => CpAsyncBulkS2GTile4D);
impl_mdim_cp_async_bulk_tile_s2g!(cp_async_bulk_tile_s2g_5d => c0, c1, c2, c3, c4 => CpAsyncBulkS2GTile5D);

macro_rules! impl_cp_bulk_test_g2c {
    ($test_name: ident: $func: ident, [$($coords: literal),*], $mcast: expr, $cache: expr, $cta_group: expr => $line: literal) => {
        #[test]
        fn $test_name() {
            let ker =
                $crate::implement_ptx_kernel::<(A<*mut u8, 1>, *const TensorMap)>($crate::backend::llvm::LLVM::new(), "test_ker");
            {
                let (global_mut, tensormap) = ker.args();
                let mbar = ker.cuda().alloc_shared::<u64>();
                let dst = ker.cuda().alloc_shared::<u8>().as_mut_ptr();
                let c0 = ker.constant(15);
                let dst = dst.copy().map_address_to_cluster(ker.constant(0));
                $func(
                    dst,
                    mbar,
                    tensormap,
                    $(ker.constant($coords),)*
                    $mcast.map(|v| ker.constant(v)),
                    $cache.map(|v| ker.constant(v)),
                    $cta_group,
                );
            }
            ker.return_void();
            let bytes = ker.compile(crate::SM::SM100a, crate::Opt::O0);
            let string = str::from_utf8(&bytes).unwrap();
            let mcast: Option<u16> = $mcast;
            let mcast = if mcast.is_some() { ".multicast::cluster" } else { "" };
            let ncoords = [$($coords,)*].len();
            let cache_hint: Option<u64> = $cache;
            let cache_hint = if cache_hint.is_some() { ".L2::cache_hint" } else { "" };
            let cta_group = match $cta_group {
              None => ""  ,
              Some(TcGen05CTA::OneSM) => ".cta_group::1",
              Some(TcGen05CTA::TwoSM) => ".cta_group::2",
            };
            assert!(string.contains(&format!($line, ncoords, mcast, cache_hint, cta_group)), "{string}");
        }
    };

    (loop_cg: $mod_name: ident: $func: ident, [$($coords: literal),*], $mcast: expr, $cache: expr => $line: literal) => {
        mod $mod_name {
            use super::*;
            impl_cp_bulk_test_g2c!(no_cta_group: $func, [$($coords),*], $mcast, $cache, None => $line);
            impl_cp_bulk_test_g2c!(cta_group_1: $func, [$($coords),*], $mcast, $cache, Some(TcGen05CTA::OneSM) => $line);
            impl_cp_bulk_test_g2c!(cta_group_2: $func, [$($coords),*], $mcast, $cache, Some(TcGen05CTA::TwoSM) => $line);
        }
    };
    (loop_cache: $mod_name: ident: $func: ident, [$($coords: literal),*], $mcast: expr => $line: literal) => {
        mod $mod_name {
            use super::*;
            impl_cp_bulk_test_g2c!(loop_cg: no_cache: $func, [$($coords),*], $mcast, None => $line);
            impl_cp_bulk_test_g2c!(loop_cg: cache_hint: $func, [$($coords),*], $mcast, Some(5) => $line);
        }
    };
    (loop_mcast: $mod_name: ident: $func: ident, [$($coords: literal),*] => $line: literal) => {
        mod $mod_name {
            use super::*;
            impl_cp_bulk_test_g2c!(loop_cache: no_mcast: $func, [$($coords),*], None => $line);
            impl_cp_bulk_test_g2c!(loop_cache: with_mcast: $func, [$($coords),*], Some(0b11) => $line);
        }
    };
    ($mod_name: ident: [
            $($dim_name: ident: $func: ident, [$($coords: literal),*]),*
        ] => $line: literal) => {
        mod $mod_name {
            use super::*;
            $(
                impl_cp_bulk_test_g2c!(loop_mcast: $dim_name: $func, [$($coords),*] => $line);
            )*
        }
    };
}

macro_rules! impl_cp_bulk_test_g2s {
    ($test_name: ident: $func: ident, [$($coords: literal),*], $cache: expr => $line: literal) => {
        #[test]
        fn $test_name() {
            let ker =
                $crate::implement_ptx_kernel::<(A<*mut u8, 1>, *const TensorMap)>($crate::backend::llvm::LLVM::new(), "test_ker");
            {
                let (global_mut, tensormap) = ker.args();
                let mbar = ker.cuda().alloc_shared::<u64>();
                let dst = ker.cuda().alloc_shared::<u8>().as_mut_ptr();
                $func(
                    dst,
                    mbar,
                    tensormap,
                    $(ker.constant($coords),)*
                    $cache.map(|v| ker.constant(v)),
                );
            }
            ker.return_void();
            let bytes = ker.compile(crate::SM::SM100a, crate::Opt::O0);
            let string = str::from_utf8(&bytes).unwrap();
            let ncoords = [$($coords,)*].len();
            let cache_hint: Option<u64> = $cache;
            let cache_hint = if cache_hint.is_some() { ".L2::cache_hint" } else { "" };
            assert!(string.contains(&format!($line, ncoords, cache_hint)), "{string}");
        }
    };
    (loop_cache: $mod_name: ident: $func: ident, [$($coords: literal),*] => $line: literal) => {
        mod $mod_name {
            use super::*;
            impl_cp_bulk_test_g2s!(no_cache: $func, [$($coords),*], None => $line);
            impl_cp_bulk_test_g2s!(cache_hint: $func, [$($coords),*], Some(5) => $line);
        }
    };
    ($mod_name: ident: [
            $($dim_name: ident: $func: ident, [$($coords: literal),*]),*
        ] => $line: literal) => {
        mod $mod_name {
            use super::*;
            $(
                impl_cp_bulk_test_g2s!(loop_cache: $dim_name: $func, [$($coords),*] => $line);
            )*
        }
    };
}

macro_rules! impl_cp_bulk_test_s2g {
    ($test_name: ident: $func: ident, [$($coords: literal),*], $cache: expr => $line: literal) => {
        #[test]
        fn $test_name() {
            let ker =
                $crate::implement_ptx_kernel::<(A<*mut u8, 1>, *const TensorMap)>($crate::backend::llvm::LLVM::new(), "test_ker");
            {
                let (global_mut, tensormap) = ker.args();
                let src = ker.cuda().alloc_shared::<u8>().as_ptr();
                $func(
                    src,
                    tensormap,
                    $(ker.constant($coords),)*
                    $cache.map(|v| ker.constant(v)),
                );
            }
            ker.return_void();
            let bytes = ker.compile(crate::SM::SM100a, crate::Opt::O0);
            let string = str::from_utf8(&bytes).unwrap();
            let ncoords = [$($coords,)*].len();
            let cache_hint: Option<u64> = $cache;
            let cache_hint = if cache_hint.is_some() { ".L2::cache_hint" } else { "" };
            assert!(string.contains(&format!($line, ncoords, cache_hint)), "{string}");
        }
    };
    (loop_cache: $mod_name: ident: $func: ident, [$($coords: literal),*] => $line: literal) => {
        mod $mod_name {
            use super::*;
            impl_cp_bulk_test_s2g!(no_cache: $func, [$($coords),*], None => $line);
            impl_cp_bulk_test_s2g!(cache_hint: $func, [$($coords),*], Some(5) => $line);
        }
    };
    ($mod_name: ident: [
            $($dim_name: ident: $func: ident, [$($coords: literal),*]),*
        ] => $line: literal) => {
        mod $mod_name {
            use super::*;
            $(
                impl_cp_bulk_test_s2g!(loop_cache: $dim_name: $func, [$($coords),*] => $line);
            )*
        }
    };
}
#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! impl_cp_bulk_test_g2c {
    ($($t: tt)*) => {};
}
#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! impl_cp_bulk_test_g2s {
    ($($t: tt)*) => {};
}
#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! impl_cp_bulk_test_s2g {
    ($($t: tt)*) => {};
}

#[cfg(test)]
mod test_cp_async_tile_ptx_gen {
    use super::*;

    impl_cp_bulk_test_g2c!(
        test_cp_async_tile_g2c: [
            tile_1d: cp_async_bulk_tile_g2c_1d, [15],
            tile_2d: cp_async_bulk_tile_g2c_2d, [15, 32],
            tile_3d: cp_async_bulk_tile_g2c_3d, [15, 32, 1],
            tile_4d: cp_async_bulk_tile_g2c_4d, [15, 32, 1, 8],
            tile_5d: cp_async_bulk_tile_g2c_5d, [15, 32, 1, 8, 9]
        ]
        => "cp.async.bulk.tensor.{}d.shared::cluster.global.tile.mbarrier::complete_tx::bytes{}{}{} "
    );

    impl_cp_bulk_test_g2s!(
        test_cp_async_tile_g2s: [
            tile_1d: cp_async_bulk_tile_g2s_1d, [15],
            tile_2d: cp_async_bulk_tile_g2s_2d, [15, 32],
            tile_3d: cp_async_bulk_tile_g2s_3d, [15, 32, 1],
            tile_4d: cp_async_bulk_tile_g2s_4d, [15, 32, 1, 8],
            tile_5d: cp_async_bulk_tile_g2s_5d, [15, 32, 1, 8, 9]
        ]
        => "cp.async.bulk.tensor.{}d.shared::cta.global.tile.mbarrier::complete_tx::bytes{} "
    );

    impl_cp_bulk_test_s2g!(
        test_cp_async_tile_s2g: [
            tile_1d: cp_async_bulk_tile_s2g_1d, [15],
            tile_2d: cp_async_bulk_tile_s2g_2d, [15, 32],
            tile_3d: cp_async_bulk_tile_s2g_3d, [15, 32, 1],
            tile_4d: cp_async_bulk_tile_s2g_4d, [15, 32, 1, 8],
            tile_5d: cp_async_bulk_tile_s2g_5d, [15, 32, 1, 8, 9]
        ]
        => "cp.async.bulk.tensor.{}d.global.shared::cta.tile.bulk_group{} "
    );
}
