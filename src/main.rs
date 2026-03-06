use rtc_tile::{
    Tile, WarpSmemLoadTileTy, WarpTileTy,
    gmem::{BidXGroup, Matrix},
    mma::SyncMMAOp,
};
use rtc_types::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    kernel_print,
    ty::{M, R, U32, cuda::Global, raw::*},
    val::Val,
};

type TileT = rtc_tile::bf16_tile::MmaBf16_16x16;

type MMA = rtc_tile::mma::sm80::Sm80MmaBf16F32_16x8x16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        Global<P<*const F32>>,
        U32,
        U32,
        Global<M<&mut <TileT as WarpTileTy>::FragT>>,
    )>();
    let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (ptr, nrows, ncols, mut _d) = kernel.get_args();

    let mut matrix = Matrix::new(ptr, nrows, ncols);

    let group = BidXGroup(kernel.intrinsics());
    for t in matrix.row_panel_iter_by_group(group, 16) {
        kernel_print!("We are at pointer {}", t.ptr);
    }

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Aggressive,
        print_at,
        |_| (),
        // |_| (),
    );

    println!("{}", asm);
}

fn test_mma() {
    test_inner();
}

fn main() {
    test_mma();
}
