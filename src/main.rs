use rtc_tile::{Tile, WarpSmemLoadTileTy, WarpTileTy, mma::SyncMMAOp};
use rtc_types::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    kernel_print,
    ty::{M, R, cuda::Global},
    val::Val,
};

type TileT = rtc_tile::bf16_tile::MmaBf16_16x16;

type MMA = rtc_tile::mma::sm80::Sm80MmaBf16F32_16x8x16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        Global<R<&<MMA as SyncMMAOp>::AFrag>>,
        Global<R<&<MMA as SyncMMAOp>::BFrag>>,
        Global<M<&mut <MMA as SyncMMAOp>::CFrag>>,
        Global<M<&mut <TileT as WarpTileTy>::FragT>>,
    )>();
    let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (_a_frag, b_frag, mut d_frag, mut _d) = kernel.get_args();

    let lane = kernel.intrinsics().laneid();
    let a_args = TileT::collective_load(&mut c_shared, lane);
    let c_frag = Val::zeros(a_args.cx());

    kernel_print!("abcd" => a_args.cx());

    let c_res = MMA::call(a_args, b_frag.load(), c_frag);
    kernel_print!(
        "Vector formatter?: \nFor A: '{}'\nFor B: '{}'\nFor C: '{}'",
        a_args,
        b_frag.load(),
        c_res,
    );
    d_frag.store(c_res);

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Aggressive,
        |_| (),
        // |_| (),
        print_at,
    );

    println!("{}", asm);
}

fn test_mma() {
    test_inner();
}

fn main() {
    test_mma();
}
