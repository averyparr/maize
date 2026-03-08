use rtc_tile::{gmem::Matrix, group::by_block::BlockY, mma::run_test_sync_mma};
use rtc_types::{
    codegen::{Func, loops::Looper, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    struct_reflect,
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
        M<&'static mut F32>,
        U32,
        U32,
        M<&'static mut F32>,
        U32,
        U32,
        F32,
        F32,
        F32,
        M<&mut F32, Global>,
    )>();
    // let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (amat, arows, acols, bmat, brows, bcols, per_row, per_col, stored_every_loop, mut to_store) =
        kernel.get_args();
    let mut amat = Matrix::new(amat, arows, acols);
    let mut bmat = Matrix::new(bmat, brows, bcols);

    let group = BlockY(kernel.intrinsics());

    let row_iter = amat.collective_row_panel_iter::<16>(group);
    row_iter.for_each(|mut row| {
        let col_iter = bmat.collective_col_panel_iter::<16>(group);
        col_iter.for_each(|mut col| {
            row.ptr.store(per_row);
            col.ptr.store(per_col);
            to_store.store(stored_every_loop);
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
        |_| (),
        print_at,
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
