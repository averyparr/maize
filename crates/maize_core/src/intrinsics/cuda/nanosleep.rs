use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
    val::Val,
};

impl_intrinsics!(
    Nanosleep: "llvm.nvvm.nanosleep"(u32) -> VoidType
);

impl CUDA {
    pub fn nanosleep(ns: Val<u32>) {
        Nanosleep.call((ns,))
    }
}
