use crate::{
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
    val::Val,
};

impl_intrinsics!(
    BitmaskClamp: "llvm.nvvm.bmsk.clamp" (u32, u32) -> u32,
    BitmaskWrap: "llvm.nvvm.bmsk.wrap" (u32, u32) -> u32,
);

impl CUDA {
    pub fn bitmask(from_bit: Val<u32>, bit_width: Val<u32>) -> Val<u32> {
        BitmaskClamp.call((from_bit, bit_width))
    }
    pub fn bitmask_wrapped(from_bit: Val<u32>, bit_width: Val<u32>) -> Val<u32> {
        BitmaskWrap.call((from_bit, bit_width))
    }
}

// TODO: prmt
