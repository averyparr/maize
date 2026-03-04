use rtc_tile::{Tile, WarpSmemLoadTileTy, WarpTileTy};
use rtc_types::{
    codegen::{Func, new_ptx_kernel, target_cpu::cuda::SM, typed_func::FnCodegen},
    inkwell::{OptimizationLevel, intrinsics::Intrinsic},
    struct_reflect,
    ty::{BF16, F32, I32, M, R, V, cuda::Global},
    val::Val,
};

type TileT = rtc_tile::bf16_tile::MmaBf16_16x16;

struct_reflect!(
    pub struct MmaRetBf16_16x8x16 {
        pub d0: F32,
        pub d1: F32,
        pub d2: F32,
        pub d3: F32,
    } => mma_ret_bf16_16x8x16
);

pub fn test_inner() {
    let kernel = new_ptx_kernel::<(
        Global<R<&V<BF16, 8>>>,
        Global<R<&V<BF16, 4>>>,
        Global<M<&mut V<BF16, 4>>>,
        Global<M<&mut <TileT as WarpTileTy>::FragT>>,
    )>();
    let mut c_shared = kernel.intrinsics().alloc_aligned_shared::<Tile<TileT>>(16);
    kernel.use_fast_math();
    let (a_frag, b_frag, mut c_frag, mut d) = kernel.get_args();

    let lane = kernel.intrinsics().laneid();
    let a_args = TileT::collective_load(&mut c_shared, lane);

    let ret = kernel
        .cx()
        .get_intrinsic::<MmaRetBf16_16x8x16, (I32, I32, I32, I32, I32, I32, F32, F32, F32, F32)>(
            "llvm.nvvm.mma.m16n8k16.row.col.bf16",
            true,
        );
    let b_args = b_frag.load();
    let init_c = c_frag.load().zero();
    let [a0, a1, a2, a3] = unsafe { a_args.bitcast() }.elements();
    let [b0, b1] = unsafe { b_args.bitcast() }.elements();
    let [c0, c1, c2, c3] = init_c.vec_cast().elements();

    let ret = kernel
        .cx()
        .call_fn(ret, (a0, a1, a2, a3, b0, b1, c0, c1, c2, c3));
    let a = ret.as_ref().accessor();
    let v = Val::from_elements([a.d0.load(), a.d1.load(), a.d2.load(), a.d3.load()]);
    c_frag.store(v.cast());

    #[allow(unused)]
    let print_at = |cx: &FnCodegen| {
        println!("{}", cx.print_module_to_string().to_string_lossy());
    };

    let asm = kernel
        .finalize()
        .compile_asm_at_opt(&SM::SM90, OptimizationLevel::Aggressive);

    let c = Intrinsic::find("llvm.nvvm.mma.m16n8k16.row.col.bf16").expect("intrinsic should exist");
    println!("{}", asm);
}

fn main() {
    test_inner();
}
