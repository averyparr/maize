use crate::{
    backend::VoidType,
    intrinsics::{impl_argless_intrinsics, impl_intrinsics},
    tipe::A,
};

impl_intrinsics!(
    TcGen05AllocCG1: "llvm.nvvm.tcgen05.alloc.shared.cg1"(A<*mut u8, 3>, u32) -> VoidType,
    TcGen05AllocCG2: "llvm.nvvm.tcgen05.alloc.shared.cg2"(A<*mut u8, 3>, u32) -> VoidType,
    TcGen05DeallocCG1: "llvm.nvvm.tcgen05.dealloc.shared.cg1"(A<*mut u8, 3>, u32) -> VoidType,
    TcGen05DeallocCG2: "llvm.nvvm.tcgen05.dealloc.shared.cg2"(A<*mut u8, 3>, u32) -> VoidType,
);

impl_argless_intrinsics!(
    TcGen05RelinquishAllocPermitCG1: "llvm.nvvm.tcgen05.relinq.alloc.permit.cg1"() -> VoidType,
    TcGen05RelinquishAllocPermitCG2: "llvm.nvvm.tcgen05.relinq.alloc.permit.cg2"() -> VoidType,
);
