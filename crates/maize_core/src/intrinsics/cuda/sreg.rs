use crate::{
    codegen::FnCodegen,
    intrinsics::{IntrinsicCodegen, cuda::CUDA},
    ty::U32,
    val::Val,
};

pub struct CUDASpecialRegisters<'a>(IntrinsicCodegen<'a, CUDA>);

impl<'a> IntrinsicCodegen<'a, CUDA> {
    pub fn sregs(self) -> CUDASpecialRegisters<'a> {
        CUDASpecialRegisters(self)
    }

    pub fn tid_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.tid.x")
    }
    pub fn tid_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.tid.y")
    }
    pub fn tid_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.tid.z")
    }

    pub fn bdim_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.ntid.x")
    }
    pub fn bdim_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.ntid.y")
    }
    pub fn bdim_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.ntid.z")
    }

    pub fn bid_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.ctaid.x")
    }
    pub fn bid_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.ctaid.y")
    }
    pub fn bid_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.ctaid.z")
    }

    pub fn gdim_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nctaid.x")
    }
    pub fn gdim_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nctaid.y")
    }
    pub fn gdim_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nctaid.z")
    }
}

impl<'a> CUDASpecialRegisters<'a> {
    fn cx(&self) -> &'a FnCodegen {
        self.0.cx()
    }
    pub fn laneid(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.laneid")
    }
    pub fn warpid(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.warpid")
    }
    pub fn nwarpid(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nwarpid")
    }
    pub fn warpsize(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.warpsize")
    }
    pub fn clusterid_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.clusterid.x")
    }
    pub fn clusterid_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.clusterid.y")
    }
    pub fn clusterid_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.clusterid.z")
    }

    pub fn nclusterid_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nclusterid.x")
    }
    pub fn nclusterid_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nclusterid.y")
    }
    pub fn nclusterid_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nclusterid.z")
    }

    pub fn cluster_ctaid_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.ctaid.x")
    }
    pub fn cluster_ctaid_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.ctaid.y")
    }
    pub fn cluster_ctaid_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.ctaid.z")
    }

    pub fn cluster_nctaid_x(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.nctaid.x")
    }
    pub fn cluster_nctaid_y(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.nctaid.y")
    }
    pub fn cluster_nctaid_z(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.nctaid.z")
    }

    pub fn cluster_ctarank(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.ctarank")
    }
    pub fn cluster_nctarank(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.cluster.nctarank")
    }

    // === SM identity ===

    pub fn smid(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.smid")
    }
    pub fn nsmid(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.nsmid")
    }
    pub fn gridid(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.gridid")
    }

    // === Lane masks (warp-level programming) ===

    pub fn lanemask_eq(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.lanemask.eq")
    }
    pub fn lanemask_lt(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.lanemask.lt")
    }
    pub fn lanemask_le(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.lanemask.le")
    }
    pub fn lanemask_gt(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.lanemask.gt")
    }
    pub fn lanemask_ge(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.lanemask.ge")
    }

    // === Shared memory info ===
    pub fn dynamic_smem_size(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.dynamic_smem_size")
    }
    pub fn total_smem_size(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.total_smem_size")
    }
    pub fn aggr_smem_size(&self) -> Val<'a, U32> {
        CUDA::nullary_u32_intrinsic(self.cx(), "llvm.nvvm.read.ptx.sreg.aggr_smem_size")
    }
}
