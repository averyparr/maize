use rtc_tile::{gmem::Matrix, group::by_block::BlockY, mma::run_test_sync_mma};
use rtc_types::{
    codegen::{Func, loops::Looper, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    kernel_assert, struct_reflect,
    ty::{M, cuda::Global, raw::*},
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

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        M<&'static mut F32, Global>,
        U32,
        U32,
        M<&'static mut F32, Global>,
        U32,
        U32,
        F32,
        F32,
        F32,
        M<&mut F32, Global>,
    )>()
    .with_launch_bounds_2d((1, 128), Some(4), None);
    // let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (amat, arows, acols, bmat, brows, bcols, per_row, per_col, stored_every_loop, mut to_store) =
        kernel.get_args();
    let mut amat = Matrix::new(amat, arows.const_like(4096), acols.const_like(4096));
    let mut bmat = Matrix::new(bmat, brows.const_like(4096), bcols.const_like(4096));

    kernel_assert!(kernel.intrinsics().gdim_y().eq_const(128));
    let group = BlockY(kernel.intrinsics());

    let (a_panels, a_epilogue) = amat.collective_aligned_row_panel_iter::<16>(group);
    kernel_assert!(a_epilogue.row_size().eq_const(0));
    a_panels.for_every_value(|mut row_panel| {
        let (b_panels, b_epilogue) = bmat.collective_aligned_col_panel_iter::<16>(group);
        kernel_assert!(b_epilogue.col_size().eq_const(0));
        b_panels.for_every_value(|mut col| {
            let row_tiles = row_panel.gmem_tiles::<16>();
            let col_tiles = col.gmem_tiles::<16>();
            row_tiles
                .zip(col_tiles)
                .for_every_value(|(mut a_tile, mut b_tile)| {
                    a_tile.ptr.store(per_row);
                    b_tile.ptr.store(per_col);
                    to_store.store(stored_every_loop);
                });
        });
    });

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Aggressive,
        // print_at,
        print_at,
        |_| (),
        // |_| (),
    );

    println!("{}", &asm[..asm.len() - 1]);
}

pub fn test_mma() {
    println!("{}", run_test_sync_mma::<MMA>(SM::SM90));
}

fn main() {
    test_inner();
}
