use crate::{
    backend::FnRef,
    intrinsics::{Intrinsic, cuda::CUDA, impl_argless_intrinsics},
    val::Val,
};

impl_argless_intrinsics!(
    DynamicSmemSize: "llvm.nvvm.read.ptx.sreg.dynamic_smem_size"() -> u32,
    StaticSmemSize: "llvm.nvvm.read.ptx.sreg.aggr_smem_size"() -> u32,
    TotalSmemSize: "llvm.nvvm.read.ptx.sreg.total_smem_size"() -> u32,

    Clock: "llvm.nvvm.read.ptx.sreg.clock"() -> u32,
    Clock64: "llvm.nvvm.read.ptx.sreg.clock64"() -> u64,

    BlockInClusterX: "llvm.nvvm.read.ptx.sreg.cluster.ctaid.x"() -> u32,
    BlockInClusterY: "llvm.nvvm.read.ptx.sreg.cluster.ctaid.y"() -> u32,
    BlockInClusterZ: "llvm.nvvm.read.ptx.sreg.cluster.ctaid.z"() -> u32,
    BlockInClusterW: "llvm.nvvm.read.ptx.sreg.cluster.ctaid.w"() -> u32,

    ClusterDimX: "llvm.nvvm.read.ptx.sreg.cluster.nctaid.x"() -> u32,
    ClusterDimY: "llvm.nvvm.read.ptx.sreg.cluster.nctaid.y"() -> u32,
    ClusterDimZ: "llvm.nvvm.read.ptx.sreg.cluster.nctaid.z"() -> u32,
    ClusterDimW: "llvm.nvvm.read.ptx.sreg.cluster.nctaid.w"() -> u32,

    ClusterRank: "llvm.nvvm.read.ptx.sreg.cluster.ctarank"() -> u32,
    ClusterSize: "llvm.nvvm.read.ptx.sreg.cluster.nctarank"() -> u32,

    ClusterIdX: "llvm.nvvm.read.ptx.sreg.clusterid.x"() -> u32,
    ClusterIdY: "llvm.nvvm.read.ptx.sreg.clusterid.y"() -> u32,
    ClusterIdZ: "llvm.nvvm.read.ptx.sreg.clusterid.z"() -> u32,
    ClusterIdW: "llvm.nvvm.read.ptx.sreg.clusterid.w"() -> u32,

    NClustersX: "llvm.nvvm.read.ptx.sreg.nclusterid.x"() -> u32,
    NClustersY: "llvm.nvvm.read.ptx.sreg.nclusterid.y"() -> u32,
    NClustersZ: "llvm.nvvm.read.ptx.sreg.nclusterid.z"() -> u32,
    NClustersW: "llvm.nvvm.read.ptx.sreg.nclusterid.w"() -> u32,

    BlockIdX: "llvm.nvvm.read.ptx.sreg.ctaid.x"() -> u32,
    BlockIdY: "llvm.nvvm.read.ptx.sreg.ctaid.y"() -> u32,
    BlockIdZ: "llvm.nvvm.read.ptx.sreg.ctaid.z"() -> u32,
    BlockIdW: "llvm.nvvm.read.ptx.sreg.ctaid.w"() -> u32,

    NBlocksX: "llvm.nvvm.read.ptx.sreg.nctaid.x"() -> u32,
    NBlocksY: "llvm.nvvm.read.ptx.sreg.nctaid.y"() -> u32,
    NBlocksZ: "llvm.nvvm.read.ptx.sreg.nctaid.z"() -> u32,
    NBlocksW: "llvm.nvvm.read.ptx.sreg.nctaid.w"() -> u32,

    GridId: "llvm.nvvm.read.ptx.sreg.gridid"() -> u32,
    LaneId: "llvm.nvvm.read.ptx.sreg.laneid"() -> u32,
    WarpId: "llvm.nvvm.read.ptx.sreg.warpid"() -> u32,
    WarpSize: "llvm.nvvm.read.ptx.sreg.warpsize"() -> u32, // why not just 32?

    LanemaskEq: "llvm.nvvm.read.ptx.sreg.lanemask.eq"() -> u32,
    LanemaskGe: "llvm.nvvm.read.ptx.sreg.lanemask.ge"() -> u32,
    LanemaskGt: "llvm.nvvm.read.ptx.sreg.lanemask.gt"() -> u32,
    LanemaskLe: "llvm.nvvm.read.ptx.sreg.lanemask.le"() -> u32,
    LanemaskLt: "llvm.nvvm.read.ptx.sreg.lanemask.lt"() -> u32,

    NSms: "llvm.nvvm.read.ptx.sreg.nsmid"() -> u32,
    SMId: "llvm.nvvm.read.ptx.sreg.smid"() -> u32,
    NWarps: "llvm.nvvm.read.ptx.sreg.nwarpid"() -> u32,

    ThreadIdX: "llvm.nvvm.read.ptx.sreg.tid.x"() -> u32,
    ThreadIdY: "llvm.nvvm.read.ptx.sreg.tid.y"() -> u32,
    ThreadIdZ: "llvm.nvvm.read.ptx.sreg.tid.z"() -> u32,
    ThreadIdW: "llvm.nvvm.read.ptx.sreg.tid.w"() -> u32,

    NThreadsX: "llvm.nvvm.read.ptx.sreg.ntid.x"() -> u32,
    NThreadsY: "llvm.nvvm.read.ptx.sreg.ntid.y"() -> u32,
    NThreadsZ: "llvm.nvvm.read.ptx.sreg.ntid.z"() -> u32,
    NThreadsW: "llvm.nvvm.read.ptx.sreg.ntid.w"() -> u32,
);

pub struct CUDASreg(FnRef);

impl CUDA {
    pub fn sreg(&self) -> CUDASreg {
        CUDASreg(self.0.clone())
    }
}

macro_rules! sreg_passthrough {
    ($($fn_names: ident),* $(,)?) => {
        impl CUDA {
            $(
                pub fn $fn_names(&self) -> Val<u32> {
                    self.sreg().$fn_names()
                }
            )*
        }
    };
}

sreg_passthrough!(
    thread_idx_x,
    thread_idx_y,
    thread_idx_z,
    block_dim_x,
    block_dim_y,
    block_dim_z,
    block_idx_x,
    block_idx_y,
    block_idx_z,
    grid_dim_x,
    grid_dim_y,
    grid_dim_z,
);

macro_rules! read_sregs {
    ($($fn_name: ident => $intrins_name: ident),* $(,)?) => {
        $(
            pub fn $fn_name(&self) -> Val<u32> {
                $intrins_name(self.0.clone()).call(())
            }
        )*
    };
}

impl CUDASreg {
    // TODO Fill out here!

    read_sregs!(
        block_in_cluster_x => BlockInClusterX,
        block_in_cluster_y => BlockInClusterY,
        block_in_cluster_z => BlockInClusterZ,

        cluster_dim_x => ClusterDimX,
        cluster_dim_y => ClusterDimY,
        cluster_dim_z => ClusterDimZ,

        cluster_rank => ClusterRank,
        cluster_size => ClusterSize,

        cluster_idx_x => ClusterIdX,
        cluster_idx_y => ClusterIdY,
        cluster_idx_z => ClusterIdZ,

        nclusters_x => NClustersX,
        nclusters_y => NClustersY,
        nclusters_z => NClustersZ,

        block_idx_x => BlockIdX,
        block_idx_y => BlockIdY,
        block_idx_z => BlockIdZ,

        grid_dim_x => NBlocksX,
        grid_dim_y => NBlocksY,
        grid_dim_z => NBlocksZ,

        thread_idx_x => ThreadIdX,
        thread_idx_y => ThreadIdY,
        thread_idx_z => ThreadIdZ,

        block_dim_x => NThreadsX,
        block_dim_y => NThreadsY,
        block_dim_z => NThreadsZ,

        grid_id => GridId,
        warp_id => WarpId,
        warp_size => WarpSize,

        lanemask_eq => LanemaskEq,
        lanemask_ge => LanemaskGe,
        lanemask_gt => LanemaskGt,
        lanemask_le => LanemaskLe,
        lanemask_lt => LanemaskLt,

        laneid => LaneId,

        num_sms => NSms,
        sm_id => SMId,
        num_warps => NWarps,

        clock => Clock,

        static_smem_size => StaticSmemSize,
        dynamic_smem_size => DynamicSmemSize,
        total_smem_size => TotalSmemSize,
    );

    pub fn clock64(&self) -> Val<u64> {
        Clock64(self.0.clone()).call(())
    }
}
