use rtc_tile::{
    BF16_16x16, Tile, TilePair, WarpSmemLoadTileTy,
    gmem::Matrix,
    group::{by_block::BlockY, warp::Warp},
    mma::run_test_sync_mma,
};
use rtc_types::{
    codegen::{Func, loops::Looper, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    intrinsics::cuda::cp_async::{CpAsyncPipeline, CpAsyncTicket, CpAsyncToken},
    struct_reflect,
    ty::{
        Addrspace, ContiguousUniformTy, DereferencableTy, M, MutTy, RefTy, UniformTy, ValTy,
        cuda::Global, raw::*,
    },
    val::{S, Val},
};

type MMA = rtc_tile::mma::sm80::Sm80MmaBf16F32_16x8x16;

struct_reflect!(
    #[repr(align(16))]
    pub struct GlobalMatrix<T: 'static> {
        pub ptr: M<&'static mut T, Global>,
        pub nrows: U32,
        pub ncols: U32,
    } => global_matrix
);

type TileT = BF16_16x16;

pub struct Pipeline<'a, 'b, Resource, Space> {
    depth: u32,
    res_index: Val<'a, S<U32>>,
    resource: Val<'a, M<&'b mut Resource, Space>>,
}

impl<'a, 'b, ElemT, ArrayT, Space: Addrspace> Pipeline<'a, 'b, ArrayT, Space>
where
    ArrayT: ContiguousUniformTy<ElemT = ElemT> + 'static,
    ElemT: 'static,
{
    pub fn resource_at<'c>(&'c self, index: Val<'a, U32>) -> Val<'a, R<&'c ElemT, Space>> {
        ArrayT::runtime_element_ref(self.resource.reborrow(), index)
    }
    pub fn resources_at_mut<'c>(
        &'c mut self,
        index: Val<'a, U32>,
    ) -> Val<'a, M<&'c mut ElemT, Space>> {
        ArrayT::runtime_element_mut(self.resource.reborrow_mut(), index)
    }

    pub fn new(depth: u32, resource: Val<'a, M<&'b mut ArrayT, Space>>) -> Self {
        Self {
            depth,
            res_index: resource.cx().constant_from(0u32).with_storage(),
            resource: resource,
        }
    }
}

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        M<&mut BF16, Global>,
        U32,
        U32,
        M<&mut BF16, Global>,
        U32,
        U32,
        F32,
        F32,
        F32,
        M<&mut F32, Global>,
    )>()
    .with_launch_bounds_2d((1, 128), None, None);
    const PIPE: u32 = 3;
    let pipe_shared = kernel
        .intrinsics()
        .alloc_aligned_shared::<[TilePair<Tile<TileT>, Tile<TileT>>; PIPE as _]>(16);
    kernel.use_fast_math();
    let (amat, arows, acols, bmat, brows, bcols, per_row, per_col, stored_every_loop, mut to_store) =
        kernel.get_args();
    let mut amat = Matrix::new(amat, arows.const_like(4096), acols.const_like(4096));
    let mut bmat = Matrix::new(bmat, brows.const_like(4096), bcols.const_like(4096));

    let group = BlockY(kernel.intrinsics());
    let warp = Warp(kernel.intrinsics());

    let cp_async = kernel.intrinsics().cp_async();

    let (a_panels, a_epilogue) = amat.collective_aligned_row_panel_iter::<16>(group);
    let (b_panels, b_epilogue) = bmat.collective_aligned_row_panel_iter::<16>(group);

    let pipe = CpAsyncPipeline::new(PIPE, pipe_shared);

    let mut zipped_panels = a_panels.zip(b_panels);
    let (pipe, tiles) = zipped_panels.on_first(|(mut a_panel, mut b_panel)| {
        let a_tiles = a_panel.into_gmem_tiles::<16>();
        let b_tiles = b_panel.into_gmem_tiles::<16>();
        let mut tiles = a_tiles.zip(b_tiles);
        let pipe = pipe.prime_with(&mut tiles, |token, (gmem_a, gmem_b), mut smem| {
            let smem_a = smem.accessor_mut().a;
            gmem_a.collective_cp_async(&token, smem_a, warp, 16, false);
            let smem_b = smem.accessor_mut().b;
            gmem_b.collective_cp_async(&token, smem_b, warp, 16, false);
        });
        (pipe, tiles)
    });

    // let lane = kernel.intrinsics().sregs().laneid();
    // pipe.at_steady_state(|mut smem| {
    //     let smem_a = smem.accessor_mut().a;
    //     let r = TileT::collective_load(&mut smem_a, lane);
    //     let smem_b = smem.accessor_mut().b;
    // });

    // let (a_panels, b_panels) = zipped_panels.unzip();
    // a_panels.for_every_value(|mut row_panel| {
    //     let (b_panels, b_epilogue) = bmat.collective_aligned_col_panel_iter::<16>(group);
    //     b_panels.for_every_value(|mut col| {
    //         let row_tiles = row_panel.gmem_tiles::<16>();
    //         let col_tiles = col.gmem_tiles::<16>();
    //         row_tiles
    //             .zip(col_tiles)
    //             .for_every_value(|(mut a_tile, mut b_tile)| {
    //                 // let a_smem_tile = a_shared.runtime_index_mut(a_idx);
    //                 // let a_tile_smem = cp_async.async_transaction(|token| {
    //                 //     a_tile.collective_cp_async(token, a_smem_tile, warp, 16, false)
    //                 // });
    //                 // a_tile.ptr.store(per_row);
    //                 // b_tile.ptr.store(per_col);
    //                 // to_store.store(stored_every_loop);
    //             });
    //     });
    // });

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Aggressive,
        // print_at,
        |_| (),
        // print_at,
        |_| (),
    );

    println!("{}", &asm[..asm.len() - 1]);
}

pub fn test_mma() {
    println!("{}", run_test_sync_mma::<MMA>(SM::SM90));
}

fn main() {
    test_inner();
}
