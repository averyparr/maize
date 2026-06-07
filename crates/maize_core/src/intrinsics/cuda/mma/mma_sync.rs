use crate::{
    intrinsics::{
        cuda::mma::structs::{
            LegacyMmaAccumF16M8N8, LegacyMmaAccumF32M8N8, MmaAccumF16M16N8, MmaAccumF32M16N8,
            MmaAccumF64M8N8, MmaAccumF64M16N8,
        },
        impl_intrinsics,
    },
    tipe::{F16, V},
};

type F16x2 = V<F16, 2>;

impl_intrinsics!(
  // [16, 8, 16]
  MmaSyncM16N8K16RowColFP32BF16BF16FP32: "llvm.nvvm.mma.m16n8k16.row.col.bf16"(i32, i32, i32, i32, i32, i32,f32, f32, f32, f32) -> MmaAccumF32M16N8,
  MmaSyncM16N8K16RowColFP16FP16FP16FP16: "llvm.nvvm.mma.m16n8k16.row.col.f16.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> MmaAccumF16M16N8,
  MmaSyncM16N8K16RowColFP32FP16FP16FP32: "llvm.nvvm.mma.m16n8k16.row.col.f32.f32"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, f32, f32, f32, f32) -> MmaAccumF32M16N8,
  MmaSyncM16N8K16RowColFP64FP64FP64FP64: "llvm.nvvm.mma.m16n8k16.row.col.f64"(f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) -> MmaAccumF64M16N8,
  // TODO all fp8 ones! And integral! And sparse!

  // [8, 8, 4]
  // Operates on only 8 threads, hence the wider accumulator
  LegacyMmaSyncM8N8K4ColColFP16FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f16.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF16M8N8,
  LegacyMmaSyncM8N8K4ColRowFP16FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f16.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF16M8N8,
  LegacyMmaSyncM8N8K4RowColFP16FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f16.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF16M8N8,
  LegacyMmaSyncM8N8K4RowRowFP16FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f16.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF16M8N8,
  LegacyMmaSyncM8N8K4ColColFP32FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f32.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4ColRowFP32FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f32.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4RowColFP32FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f32.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4RowRowFP32FP16FP16FP16: "llvm.nvvm.mma.m8n8k4.col.col.f32.f16"(F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2, F16x2) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4ColColFP32FP16FP16FP32: "llvm.nvvm.mma.m8n8k4.col.col.f32.f32"(F16x2, F16x2, F16x2, F16x2, f32, f32, f32, f32, f32, f32, f32, f32) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4ColRowFP32FP16FP16FP32: "llvm.nvvm.mma.m8n8k4.col.col.f32.f32"(F16x2, F16x2, F16x2, F16x2, f32, f32, f32, f32, f32, f32, f32, f32) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4RowColFP32FP16FP16FP32: "llvm.nvvm.mma.m8n8k4.col.col.f32.f32"(F16x2, F16x2, F16x2, F16x2, f32, f32, f32, f32, f32, f32, f32, f32) -> LegacyMmaAccumF32M8N8,
  LegacyMmaSyncM8N8K4RowRowFP32FP16FP16FP32: "llvm.nvvm.mma.m8n8k4.col.col.f32.f32"(F16x2, F16x2, F16x2, F16x2, f32, f32, f32, f32, f32, f32, f32, f32) -> LegacyMmaAccumF32M8N8,

  MmaSyncM8N8K4RowColFP64FP64FP64FP64: "llvm.nvvm.mma.m8n8k4.row.col.f64"(f64, f64, f64, f64) -> MmaAccumF64M8N8,
);
