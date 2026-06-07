use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, impl_intrinsics},
    tipe::A,
    val::Val,
};

#[derive(Clone, Copy)]
pub enum TcGen05Kind {
    F16 = 0,
    TF32 = 1,
    F8F6F4 = 2,
    I8 = 3,
}

#[derive(Clone, Copy)]
pub enum TcGen05CTA {
    OneSM = 1,
    TwoSM = 2,
}

#[derive(Clone, Copy)]
pub enum TcGen05CollectA {
    Discard = 0,
    LastUse = 1,
    Use = 2,
    Fill = 3,
}

impl_intrinsics!(
  TcGen05MmaSA: "llvm.nvvm.tcgen05.mma.shared"(A<*mut u8, 6>, i64, i64, i32, bool, i32, i32, i32) -> VoidType,
  TcGen05MmaSAScaleD: "llvm.nvvm.tcgen05.mma.shared.scale_d"(A<*mut u8, 6>, i64, i64, i32, bool, u64, i32, i32, i32) -> VoidType,
  TcGen05MmaTA: "llvm.nvvm.tcgen05.mma.tensor"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, i32, i32, i32) -> VoidType,
  TcGen05MmaTAScaleD: "llvm.nvvm.tcgen05.mma.tensor.scale_d"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, u64, i32, i32, i32) -> VoidType,

  TcGen05MmaSAMxF8F6F4BlockScale: "llvm.nvvm.tcgen05.mma.shared.mxf8f6f4.block_scale"(A<*mut u8, 6>, i64, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaSAMxF8F6F4BlockScaleBlock32: "llvm.nvvm.tcgen05.mma.shared.mxf8f6f4.block_scale.block32"(A<*mut u8, 6>, i64, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaSAMxF4BlockScale: "llvm.nvvm.tcgen05.mma.shared.mxf4.block_scale"(A<*mut u8, 6>, i64, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaSAMxF4BlockScaleBlock32: "llvm.nvvm.tcgen05.mma.shared.mxf4.block_scale.block32"(A<*mut u8, 6>, i64, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaSAMxF4NvF4BlockScaleBlock16: "llvm.nvvm.tcgen05.mma.shared.mxf4nvf4.block_scale.block16"(A<*mut u8, 6>, i64, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaSAMxF4NvF4BlockScaleBlock32: "llvm.nvvm.tcgen05.mma.shared.mxf4nvf4.block_scale.block32"(A<*mut u8, 6>, i64, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,

  TcGen05MmaTAMxF8F6F4BlockScale: "llvm.nvvm.tcgen05.mma.tensor.mxf8f6f4.block_scale"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaTAMxF8F6F4BlockScaleBlock32: "llvm.nvvm.tcgen05.mma.tensor.mxf8f6f4.block_scale.block32"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaTAMxF4BlockScale: "llvm.nvvm.tcgen05.mma.tensor.mxf4.block_scale"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaTAMxF4BlockScaleBlock32: "llvm.nvvm.tcgen05.mma.tensor.mxf4.block_scale.block32"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaTAMxF4NvF4BlockScaleBlock16: "llvm.nvvm.tcgen05.mma.tensor.mxf4nvf4.block_scale.block16"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
  TcGen05MmaTAMxF4NvF4BlockScaleBlock32: "llvm.nvvm.tcgen05.mma.tensor.mxf4nvf4.block_scale.block32"(A<*mut u8, 6>, A<*mut u8, 6>, i64, i32, bool, A<*mut u8, 6>, A<*mut u8, 6>, i32, i32) -> VoidType,
);

pub fn tcgen05_mma_shared_a(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    kind: TcGen05Kind,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let kind = d_ptr.fn_ref().constant(kind as _);
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSA.call((d_ptr, adesc, bdesc, idesc, enable_d, kind, sm, collect_a))
}

pub fn tcgen05_mma_shared_a_scale_d(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    d_scale: u64,
    kind: TcGen05Kind,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    assert!(d_scale <= 15, "Scale must be in [0, 15]");
    let d_scale = d_ptr.fn_ref().constant(d_scale);
    let kind = d_ptr.fn_ref().constant(kind as _);
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAScaleD.call((
        d_ptr, adesc, bdesc, idesc, enable_d, d_scale, sm, kind, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    kind: TcGen05Kind,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let kind = d_ptr.fn_ref().constant(kind as _);
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTA.call((d_ptr, a_ptr, bdesc, idesc, enable_d, kind, sm, collect_a))
}

pub fn tcgen05_mma_tensor_a_scale_d(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    d_scale: u64,
    kind: TcGen05Kind,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    assert!(d_scale <= 15, "Scale must be in [0, 15]");
    let d_scale = d_ptr.fn_ref().constant(d_scale);
    let kind = d_ptr.fn_ref().constant(kind as _);
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAScaleD.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, d_scale, sm, kind, collect_a,
    ))
}

pub fn tcgen05_mma_shared_a_mxf8f6f4_block_scale(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAMxF8F6F4BlockScale.call((
        d_ptr, adesc, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a_mxf8f6f4_block_scale(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAMxF8F6F4BlockScale.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_shared_a_mxf8f6f4_block_scale_block32(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAMxF8F6F4BlockScaleBlock32.call((
        d_ptr, adesc, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a_mxf8f6f4_block_scale_block32(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAMxF8F6F4BlockScaleBlock32.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_shared_a_mxf4_block_scale(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAMxF4BlockScale.call((
        d_ptr, adesc, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a_mxf4_block_scale(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAMxF4BlockScale.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_shared_a_mxf4_block_scale_block32(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAMxF4BlockScaleBlock32.call((
        d_ptr, adesc, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a_mxf4_block_scale_block32(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAMxF4BlockScaleBlock32.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_shared_a_mxf4nvf4_block_scale_block16(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAMxF4NvF4BlockScaleBlock16.call((
        d_ptr, adesc, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a_mxf4nvf4_block_scale_block16(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAMxF4NvF4BlockScaleBlock16.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_shared_a_mxf4nvf4_block_scale_block32(
    d_ptr: Val<A<*mut u8, 6>>,
    adesc: Val<i64>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaSAMxF4NvF4BlockScaleBlock32.call((
        d_ptr, adesc, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}

pub fn tcgen05_mma_tensor_a_mxf4nvf4_block_scale_block32(
    d_ptr: Val<A<*mut u8, 6>>,
    a_ptr: Val<A<*mut u8, 6>>,
    bdesc: Val<i64>,
    idesc: Val<i32>,
    enable_d: Val<bool>,
    scale_a: Val<A<*mut u8, 6>>,
    scale_b: Val<A<*mut u8, 6>>,
    sm: TcGen05CTA,
    collect_a: TcGen05CollectA,
) {
    let sm = d_ptr.fn_ref().constant(sm as _);
    let collect_a = d_ptr.fn_ref().constant(collect_a as _);
    TcGen05MmaTAMxF4NvF4BlockScaleBlock32.call((
        d_ptr, a_ptr, bdesc, idesc, enable_d, scale_a, scale_b, sm, collect_a,
    ))
}
