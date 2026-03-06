use rtc_tile::{
    WarpTileTy,
    gmem::Matrix,
    group::by_block::{BlockX, BlockY},
    mma::run_test_sync_mma,
};
use rtc_types::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    ty::{M, R, cuda::Global, raw::*},
};

type TileT = rtc_tile::bf16_tile::MmaBf16_16x16;

type MMA = rtc_tile::mma::sm80::Sm80MmaBf16F32_16x8x16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        Global<P<*const F32>>,
        U32,
        U32,
        Global<R<&<TileT as WarpTileTy>::FragT>>,
        Global<M<&mut <TileT as WarpTileTy>::FragT>>,
    )>();
    // let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (ptr, nrows, ncols, c_val, mut d_val) = kernel.get_args();
    // let (ptr, nrows, ncols) = kernel.get_args();

    let mut matrix = Matrix::new(ptr, nrows, ncols);

    let group = BlockY(kernel.intrinsics());

    for panel in matrix.collective_row_panel_iter::<16>(group) {
        // kernel_print!("We are at pointer {}", );
        let r = panel.ptr.to_mut_ptr().ptr_cast_mut();
        unsafe { r.write(c_val.load().vec_cast::<F32>()) };
        d_val.store(c_val.load());
    }

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Aggressive,
        |_| (),
        // print_at,
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
