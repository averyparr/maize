use crate::{
    backend::VoidType, intrinsics::Intrinsic, intrinsics::impl_intrinsics, tipe::A, val::Val,
};

impl_intrinsics!(
    TcGen05Cp4x256bCG1: "llvm.nvvm.tcgen05.cp.4x256b.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x256bCG1: "llvm.nvvm.tcgen05.cp.128x256b.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x128bCG1: "llvm.nvvm.tcgen05.cp.128x128b.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp32x128bWarpX4CG1: "llvm.nvvm.tcgen05.cp.32x128b_warpx4.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W02W13CG1: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_02_13.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W01W23CG1: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_01_23.cg1" (A<*mut u8, 6>, u64) -> VoidType,

    TcGen05Cp4x256bPackB6x16To32CG1: "llvm.nvvm.tcgen05.cp.4x256b.b6x16_p32.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x256bPackB6x16To32CG1: "llvm.nvvm.tcgen05.cp.128x256b.b6x16_p32.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x128bPackB6x16To32CG1: "llvm.nvvm.tcgen05.cp.128x128b.b6x16_p32.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp32x128bWarpX4PackB6x16To32CG1: "llvm.nvvm.tcgen05.cp.32x128b_warpx4.b6x16_p32.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W02W13PackB6x16To32CG1: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_02_13.b6x16_p32.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W01W23PackB6x16To32CG1: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_01_23.b6x16_p32.cg1" (A<*mut u8, 6>, u64) -> VoidType,

    TcGen05Cp4x256bPackB4x16To64CG1: "llvm.nvvm.tcgen05.cp.4x256b.b4x16_p64.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x256bPackB4x16To64CG1: "llvm.nvvm.tcgen05.cp.128x256b.b4x16_p64.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x128bPackB4x16To64CG1: "llvm.nvvm.tcgen05.cp.128x128b.b4x16_p64.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp32x128bWarpX4PackB4x16To64CG1: "llvm.nvvm.tcgen05.cp.32x128b_warpx4.b4x16_p64.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W02W13PackB4x16To64CG1: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_02_13.b4x16_p64.cg1" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W01W23PackB4x16To64CG1: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_01_23.b4x16_p64.cg1" (A<*mut u8, 6>, u64) -> VoidType,

    TcGen05Cp4x256bCG2: "llvm.nvvm.tcgen05.cp.4x256b.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x256bCG2: "llvm.nvvm.tcgen05.cp.128x256b.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x128bCG2: "llvm.nvvm.tcgen05.cp.128x128b.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp32x128bWarpX4CG2: "llvm.nvvm.tcgen05.cp.32x128b_warpx4.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W02W13CG2: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_02_13.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W01W23CG2: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_01_23.cg2" (A<*mut u8, 6>, u64) -> VoidType,

    TcGen05Cp4x256bPackB6x16To32CG2: "llvm.nvvm.tcgen05.cp.4x256b.b6x16_p32.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x256bPackB6x16To32CG2: "llvm.nvvm.tcgen05.cp.128x256b.b6x16_p32.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x128bPackB6x16To32CG2: "llvm.nvvm.tcgen05.cp.128x128b.b6x16_p32.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp32x128bWarpX4PackB6x16To32CG2: "llvm.nvvm.tcgen05.cp.32x128b_warpx4.b6x16_p32.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W02W13PackB6x16To32CG2: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_02_13.b6x16_p32.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W01W23PackB6x16To32CG2: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_01_23.b6x16_p32.cg2" (A<*mut u8, 6>, u64) -> VoidType,

    TcGen05Cp4x256bPackB4x16To64CG2: "llvm.nvvm.tcgen05.cp.4x256b.b4x16_p64.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x256bPackB4x16To64CG2: "llvm.nvvm.tcgen05.cp.128x256b.b4x16_p64.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp128x128bPackB4x16To64CG2: "llvm.nvvm.tcgen05.cp.128x128b.b4x16_p64.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp32x128bWarpX4PackB4x16To64CG2: "llvm.nvvm.tcgen05.cp.32x128b_warpx4.b4x16_p64.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W02W13PackB4x16To64CG2: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_02_13.b4x16_p64.cg2" (A<*mut u8, 6>, u64) -> VoidType,
    TcGen05Cp64x128bWarpX2W01W23PackB4x16To64CG2: "llvm.nvvm.tcgen05.cp.64x128b_warpx2_01_23.b4x16_p64.cg2" (A<*mut u8, 6>, u64) -> VoidType,
);

macro_rules! test_tcgen05_cp {
    ($name: ident => $intrins: ident: $contains: literal) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            #[test]
            fn test_ptx_gen() {
                let func = $crate::func::implement_ptx_kernel::<(A<*mut u8, 6>, u64)>(
                    $crate::backend::llvm::LLVM::new(),
                    concat!("test_", stringify!($name)),
                );
                {
                    let (tmem_ptr, sdesc) = func.args();
                    $name(tmem_ptr, sdesc);
                }
                func.return_void();
                let bytes = func.compile($crate::SM::SM100a, $crate::Opt::O3);
                let string = str::from_utf8(&bytes).expect("Should be valid");
                assert!(string.contains($contains), "{string}");
            }
        }
    };
}
#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! test_tcgen05_cp {
    ($($t: tt)*) => {};
}

macro_rules! wrap_tcgen05_cp {
    ($name: ident => $intrins: ident: $contains: literal) => {
        pub fn $name(tmem_ptr: Val<A<*mut u8, 6>>, sdesc: Val<u64>) {
            $intrins.call((tmem_ptr, sdesc))
        }
        test_tcgen05_cp!($name => $intrins: $contains);
    };
}

wrap_tcgen05_cp!(tcgen05_cp_4x256b_cg1 => TcGen05Cp4x256bCG1: "tcgen05.cp.cta_group::1.4x256b ");
wrap_tcgen05_cp!(tcgen05_cp_128x256b_cg1 => TcGen05Cp128x256bCG1: "tcgen05.cp.cta_group::1.128x256b ");
wrap_tcgen05_cp!(tcgen05_cp_128x128b_cg1 => TcGen05Cp128x128bCG1: "tcgen05.cp.cta_group::1.128x128b ");
wrap_tcgen05_cp!(tcgen05_cp_32x128b_warp_x4_cg1 => TcGen05Cp32x128bWarpX4CG1: "tcgen05.cp.cta_group::1.32x128b.warpx4 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_02_warp_13_cg1 => TcGen05Cp64x128bWarpX2W02W13CG1: "tcgen05.cp.cta_group::1.64x128b.warpx2::02_13 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_01_warp_23_cg1 => TcGen05Cp64x128bWarpX2W01W23CG1: "tcgen05.cp.cta_group::1.64x128b.warpx2::01_23 ");

wrap_tcgen05_cp!(tcgen05_cp_4x256b_pack_b6x16_p32_cg1 => TcGen05Cp4x256bPackB6x16To32CG1: "tcgen05.cp.cta_group::1.4x256b.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_128x256b_pack_b6x16_p32_cg1 => TcGen05Cp128x256bPackB6x16To32CG1: "tcgen05.cp.cta_group::1.128x256b.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_128x128b_pack_b6x16_p32_cg1 => TcGen05Cp128x128bPackB6x16To32CG1: "tcgen05.cp.cta_group::1.128x128b.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_32x128b_warp_x4_pack_b6x16_p32_cg1 => TcGen05Cp32x128bWarpX4PackB6x16To32CG1: "tcgen05.cp.cta_group::1.32x128b.warpx4.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_02_warp_13_pack_b6x16_p32_cg1 => TcGen05Cp64x128bWarpX2W02W13PackB6x16To32CG1: "tcgen05.cp.cta_group::1.64x128b.warpx2::02_13.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_01_warp_23_pack_b6x16_p32_cg1 => TcGen05Cp64x128bWarpX2W01W23PackB6x16To32CG1: "tcgen05.cp.cta_group::1.64x128b.warpx2::01_23.b8x16.b6x16_p32 ");

wrap_tcgen05_cp!(tcgen05_cp_4x256b_pack_b4x16_p64_cg1 => TcGen05Cp4x256bPackB4x16To64CG1: "tcgen05.cp.cta_group::1.4x256b.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_128x256b_pack_b4x16_p64_cg1 => TcGen05Cp128x256bPackB4x16To64CG1: "tcgen05.cp.cta_group::1.128x256b.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_128x128b_pack_b4x16_p64_cg1 => TcGen05Cp128x128bPackB4x16To64CG1: "tcgen05.cp.cta_group::1.128x128b.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_32x128b_warp_x4_pack_b4x16_p64_cg1 => TcGen05Cp32x128bWarpX4PackB4x16To64CG1: "tcgen05.cp.cta_group::1.32x128b.warpx4.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_02_warp_13_pack_b4x16_p64_cg1 => TcGen05Cp64x128bWarpX2W02W13PackB4x16To64CG1: "tcgen05.cp.cta_group::1.64x128b.warpx2::02_13.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_01_warp_23_pack_b4x16_p64_cg1 => TcGen05Cp64x128bWarpX2W01W23PackB4x16To64CG1: "tcgen05.cp.cta_group::1.64x128b.warpx2::01_23.b8x16.b4x16_p64 ");

wrap_tcgen05_cp!(tcgen05_cp_4x256b_cg2 => TcGen05Cp4x256bCG2: "tcgen05.cp.cta_group::2.4x256b ");
wrap_tcgen05_cp!(tcgen05_cp_128x256b_cg2 => TcGen05Cp128x256bCG2: "tcgen05.cp.cta_group::2.128x256b ");
wrap_tcgen05_cp!(tcgen05_cp_128x128b_cg2 => TcGen05Cp128x128bCG2: "tcgen05.cp.cta_group::2.128x128b ");
wrap_tcgen05_cp!(tcgen05_cp_32x128b_warp_x4_cg2 => TcGen05Cp32x128bWarpX4CG2: "tcgen05.cp.cta_group::2.32x128b.warpx4 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_02_warp_13_cg2 => TcGen05Cp64x128bWarpX2W02W13CG2: "tcgen05.cp.cta_group::2.64x128b.warpx2::02_13 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_01_warp_23_cg2 => TcGen05Cp64x128bWarpX2W01W23CG2: "tcgen05.cp.cta_group::2.64x128b.warpx2::01_23 ");

wrap_tcgen05_cp!(tcgen05_cp_4x256b_pack_b6x16_p32_cg2 => TcGen05Cp4x256bPackB6x16To32CG2: "tcgen05.cp.cta_group::2.4x256b.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_128x256b_pack_b6x16_p32_cg2 => TcGen05Cp128x256bPackB6x16To32CG2: "tcgen05.cp.cta_group::2.128x256b.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_128x128b_pack_b6x16_p32_cg2 => TcGen05Cp128x128bPackB6x16To32CG2: "tcgen05.cp.cta_group::2.128x128b.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_32x128b_warp_x4_pack_b6x16_p32_cg2 => TcGen05Cp32x128bWarpX4PackB6x16To32CG2: "tcgen05.cp.cta_group::2.32x128b.warpx4.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_02_warp_13_pack_b6x16_p32_cg2 => TcGen05Cp64x128bWarpX2W02W13PackB6x16To32CG2: "tcgen05.cp.cta_group::2.64x128b.warpx2::02_13.b8x16.b6x16_p32 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_01_warp_23_pack_b6x16_p32_cg2 => TcGen05Cp64x128bWarpX2W01W23PackB6x16To32CG2: "tcgen05.cp.cta_group::2.64x128b.warpx2::01_23.b8x16.b6x16_p32 ");

wrap_tcgen05_cp!(tcgen05_cp_4x256b_pack_b4x16_p64_cg2 => TcGen05Cp4x256bPackB4x16To64CG2: "tcgen05.cp.cta_group::2.4x256b.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_128x256b_pack_b4x16_p64_cg2 => TcGen05Cp128x256bPackB4x16To64CG2: "tcgen05.cp.cta_group::2.128x256b.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_128x128b_pack_b4x16_p64_cg2 => TcGen05Cp128x128bPackB4x16To64CG2: "tcgen05.cp.cta_group::2.128x128b.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_32x128b_warp_x4_pack_b4x16_p64_cg2 => TcGen05Cp32x128bWarpX4PackB4x16To64CG2: "tcgen05.cp.cta_group::2.32x128b.warpx4.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_02_warp_13_pack_b4x16_p64_cg2 => TcGen05Cp64x128bWarpX2W02W13PackB4x16To64CG2: "tcgen05.cp.cta_group::2.64x128b.warpx2::02_13.b8x16.b4x16_p64 ");
wrap_tcgen05_cp!(tcgen05_cp_64x128b_warp_01_warp_23_pack_b4x16_p64_cg2 => TcGen05Cp64x128bWarpX2W01W23PackB4x16To64CG2: "tcgen05.cp.cta_group::2.64x128b.warpx2::01_23.b8x16.b4x16_p64 ");
