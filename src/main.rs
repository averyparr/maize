use rtc_tile::{Tile, WarpLoadTileTy, WarpTileTy};
use rtc_types::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    ty::{M, cuda::Global},
};

type TileT = rtc_tile::BF16_16x16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Global<M<&mut <TileT as WarpTileTy>::FragT>>,)>();
    let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (mut c,) = kernel.get_args();

    let lane = kernel.intrinsics().laneid(kernel.cx());
    let ret = TileT::collective_load(&mut c_shared, lane);
    c.store(ret.vec_cast());

    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel.finalize().compile_asm_at_opt_with_hooks(
        &SM::SM90,
        OptimizationLevel::Default,
        print_at,
        |arg| {
            print_at(arg);
        },
    );
    println!("{}", asm);
}

fn main() {
    test_inner();
}
