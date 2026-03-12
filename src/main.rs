use maize_core::{
    codegen::{Func, loops::Looper, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    struct_reflect,
    ty::{M, cuda::Global, raw::*},
};
use maize_tile::{
    BF16_16x16, Tile, TilePair,
    gemm::{KTile, sm80_gemm_smem_multibuffer},
    gmem::Matrix,
    group::{
        by_block::{BlockX, BlockY},
        warp::Warp,
    },
    mma::{run_test_sync_mma, sm80_derived::Sm80MmaBf16F32_16x16x16},
};

type MMA = maize_tile::mma::sm80::Sm80MmaBf16F32_16x8x16;

struct_reflect!(
    #[repr(align(16))]
    pub struct GlobalMatrix<T: 'static> {
        pub ptr: M<&'static mut T, Global>,
        pub nrows: U32,
        pub ncols: U32,
    } => global_matrix
);

type TileT = BF16_16x16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        M<&mut BF16, Global>,
        U32,
        U32,
        M<&mut BF16, Global>,
        U32,
        U32,
    )>()
    .with_launch_bounds_2d((1, 128), None, None);
    const PIPE: u32 = 4;
    let mut pipe_shared = kernel
        .intrinsics()
        .alloc_aligned_shared::<[TilePair<Tile<TileT>, Tile<TileT>>; PIPE as _]>(16);
    kernel.use_fast_math();
    let (amat, arows, acols, bmat, brows, bcols) = kernel.get_args();
    let amat = Matrix::new(amat, arows.const_like(4096), acols.const_like(4096));
    let bmat = Matrix::new(bmat, brows.const_like(4096), bcols.const_like(4096));

    let a_panel_group = BlockX::new(kernel.cx());
    let b_panel_group = BlockY::new(kernel.cx());
    let warp = Warp::new(kernel.cx());

    let trap = || kernel.intrinsics().trap();

    let (a_panels, _a_epilogue) = amat.collective_aligned_row_panel_iter::<16>(a_panel_group);
    let (b_panels, _b_epilogue) = bmat.collective_aligned_col_panel_iter::<16>(b_panel_group);

    a_panels.for_every_value(|a_panel| {
        let mut smem = pipe_shared.reborrow_mut();
        b_panels.for_every_value(|b_panel| {
            let ret = sm80_gemm_smem_multibuffer(
                &Sm80MmaBf16F32_16x16x16,
                KTile::<16>,
                a_panel,
                b_panel,
                smem.reborrow_mut(),
                warp,
                16,
            );
            (ret.sum().eq_const(0.0)).branch(trap);
        });
    });

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Aggressive,
        |_| (),
        // print_at,
        |_| (),
        // print_at,
    );

    println!("{}", asm);
}

pub fn test_mma() {
    println!("{}", run_test_sync_mma::<MMA>(SM::SM90));
}

fn main() {
    test_inner();
}
