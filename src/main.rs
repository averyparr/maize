use std::iter::Sum;

use rtc_core::traits::HasCXVal;
use rtc_core::traits::constants::C;
use rtc_core::ty::primitive::{F16, F16x2, F32, F64};
use rtc_core::ty::ptr::{Global, M, R};
use rtc_core::ty::{FromCtx, Ty, V};
use rtc_core::val::Val;
use rtc_core::{
    FileType, OptimizationLevel,
    codegen::{
        func_with_args::create_kernel,
        target::{PTXOptions, SM, TargetMachine},
    },
};

type Fl = F32;

fn main() {
    /* Lets our runtime know we're going to be writing a CUDA kernel */
    let func = create_kernel::<(
        /* It takes arguments... */
        /* Global memory [immutable, shared] R[eference] to V[ector] of Fl[oat 16], length 8 */
        Global<R<&F16>>,
        Global<R<&F16>>,
        /* Global memory M[utable reference] to V[ector] of Fl[oat 16], length 8  */
        Global<M<&mut F16>>,
        /* Global memory M[utable reference] to Fl[oat 32] */
        Global<M<&mut Fl>>,
    )>();
    // We can write down the function by asking for its arguments, which begins symbolic evaluation
    let (a_ptr, b_ptr, mut c_ptr, mut d_ptr) = func.get_args();

    // We support both regular loads and 'nc' loads (when safe) which are faster, assuming that
    // the memory is never going to be read
    let a_val = a_ptr.load_nc();
    /* These loads are always vectorized (see the .v4.b32 == .v8.f16!) */
    let b_val = b_ptr.load();

    /* Summation is done between vectors in a vectorized manner (see add.f16x2) */
    let sum_val = a_val + b_val;

    // Stores are done in a vectorized way
    c_ptr.store(sum_val.__intrinsic_ex2_approx());

    // // Reductions from a full vector to a single value are done in a vectorized way -- notice e.g.
    // // r0 = [l0, l1] * [l0, l1]         {mul.f16x2}
    // // r1 = [l2, l3] * [l2, l3] + r0    {fma.rn.f16x2}
    // // r2 = [l4, l5] * [l4, l5] + r1    {fma.rn.f16x2}
    // // r3 = [l6, l7] * [l6, l7] + r2    {fma.rn.f16x2}
    // // [p1, p2] = r3                    {mov.b32}
    // // final = p1 + p2                  {add.f16}
    // // final_f32 = f32(final)           {cvt.f32.f16}
    // d_ptr.store(
    //     sum_val
    //         .as_primitive()
    //         .__intrinsic_abs()
    //         .as_vec()
    //         .cast::<V<F32, 2>>()
    //         .square()
    //         .sum(),
    // );

    // After this, `func` has taken in our symbolic evaluation and we can directly just-in-time-compile it
    let to_jit = func.finalize();
    // We extract the raw bytes of the thing as if it were compiled in PTX text (assembly for NVIDIA GPUs)
    let out = to_jit.compile(
        TargetMachine::PTX(PTXOptions { sm: SM::SM90a }),
        OptimizationLevel::Aggressive,
        FileType::Assembly,
    );
    let s = str::from_utf8(&out).expect("Should be valid UTF-8");
    println!("{}", s);
    // println!("{}", to_jit.as_llvm_ir());
}
