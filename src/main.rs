use rtc_tile::{BF16_8x8, BF16_16x16, Tile, WarpLoadTileTy, WarpTileTy};
use rtc_types::{
    codegen::{Func, new_ptx_kernel, target::cuda::SM, typed_func::FnCodegen},
    inkwell::OptimizationLevel,
    ty::{F32, M, cuda::Global},
};

type Fl = F32;
type FlAlt = F32;

const VEC_LEN: usize = 4;

type TileT = BF16_16x16;

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(Global<M<&mut <TileT as WarpTileTy>::FragT>>,)>();
    let mut c_shared =
        kernel.intrinsic_fn(|i| i.alloc_aligned_shared::<Tile<TileT>>(kernel.cx(), 16));
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
        OptimizationLevel::Aggressive,
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
