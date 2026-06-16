#![expect(unused)]

use std::mem::MaybeUninit;

use inkwell::values::FastMathFlags;

use crate::{
    backend::{LLVM, Opt, cpu::cuda::SM},
    control_flow::If,
    func::implement_ptx_kernel,
    intrinsics::{
        Intrinsic,
        cuda::{
            CUDA,
            mbar::{Mbar, mbar_init},
            mma::{
                mma_sync::{
                    LegacyMmaSyncM8N8K4ColColFP16FP16FP16FP16,
                    LegacyMmaSyncM8N8K4ColColFP32FP16FP16FP16,
                    LegacyMmaSyncM8N8K4ColColFP32FP16FP16FP32, MmaSyncM8N8K4RowColFP64FP64FP64FP64,
                },
                structs::{
                    LegacyMmaAccumF32M8N8, MmaAccumF32M16N8, MmaAccumF64M8N8, MmaAccumF64M16N8,
                },
                tcgen05::{TcGen05MmaSA, TcGen05MmaSAScaleD},
            },
        },
    },
    tipe::{A, F16, V},
    val::Val,
};

pub mod assert;
pub mod backend;
pub mod control_flow;
pub mod func;
pub mod intrinsics;
pub mod tipe;
pub mod val;

type InpT = V<F16, 2>;
type AccT = f64;
const A_FRG_SIZE: usize = 8;
const B_FRG_SIZE: usize = 4;
const C_FRG_SIZE: usize = 4;

pub fn is_aligned() {
    #[rustfmt::skip]
    let func = implement_ptx_kernel::<(
        &mut MmaAccumF64M8N8,
        [InpT; A_FRG_SIZE],
        [InpT; B_FRG_SIZE],
        [AccT; C_FRG_SIZE],
        f64,
        &mut f64,
        A<*mut u8, 6>,
        i64, i64, i32, bool, i32, i32, i32
    )>(LLVM::new(&SM::SM90a), "func");
    func.set_ins_flags(|flags| {
        flags.fast_math = FastMathFlags::all();
        flags.unsigned_wrap = true;
        flags.signed_wrap = true;
    });
    {
        #[rustfmt::skip]
        let (
            stor,
            aargs,
            bargs,
            cargs,
            vvec,
            mut vptr,
            tptr, adesc,bdesc, idesc, enable_input_d, m, n, k
        ) = func.args();
        let [a0, a1, a2, a3, a4, a5, a6, a7] = aargs.elements();
        let [b0, b1, b2, b3] = bargs.elements();
        let [c0, c1, c2, c3] = cargs.elements();
        let enable_input_d = func.constant(false);
        let kind_flag = func.constant(0);
        let cta_group_flag = func.constant(1);
        let collector_use_flag = func.constant(2);
        let d_scale = func.constant(5);
        let mut smem = func.cuda().alloc_shared();
        let smem = smem.write(vvec);
        vptr.store(smem.load());
        // stor.store(t);
    }
    func.return_void();
    func.run_passes(Opt::O3);
    println!("{}", func.module_string());
    let b = func.compile(Opt::O3);
    println!("{}", str::from_utf8(&b).expect("Should be valid"));
    // assert!(false);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_is_aligned() {
        super::is_aligned();
    }
}

pub type StructType = inkwell::types::StructType<'static>;
pub type StructValue = inkwell::values::StructValue<'static>;
pub type ArrayType = inkwell::types::ArrayType<'static>;
pub type ArrayValue = inkwell::values::ArrayValue<'static>;
pub type ContextRef = inkwell::context::ContextRef<'static>;
pub type IntType = inkwell::types::IntType<'static>;
pub type IntValue = inkwell::values::IntValue<'static>;
pub type FloatType = inkwell::types::FloatType<'static>;
pub type FloatValue = inkwell::values::FloatValue<'static>;
pub type VectorType = inkwell::types::VectorType<'static>;
pub type VectorValue = inkwell::values::VectorValue<'static>;
pub type PointerType = inkwell::types::PointerType<'static>;
pub type PointerValue = inkwell::values::PointerValue<'static>;

pub use inkwell::types::BasicType;
pub use inkwell::values::BasicValue;

pub fn collective_mbar_init<const N_STAGES: usize, const N_MBARS: usize>(
    mut storage: Val<A<&mut MaybeUninit<[[Mbar; N_MBARS]; N_STAGES]>, 3>>,
    arrive_count: Val<[u32; N_MBARS]>,
) -> Val<A<&mut [[Mbar; N_MBARS]; N_STAGES], 3>> {
    let total_mbars = N_STAGES * N_MBARS;
    assert!(total_mbars <= 32);
    let cuda = CUDA(storage.fn_ref().clone());
    let sregs = cuda.sreg();
    If(sregs.warp_id().eq_const(0)).then(|| {
        If(sregs.laneid().lt_const(N_STAGES as _)).then(|| {
            let local_ptr: Val<A<*mut [[MaybeUninit<Mbar>; N_MBARS]; 0], 3>> =
                storage.reborrow().as_mut_ptr().ptr_cast();
            let local_ptr = local_ptr.index(sregs.laneid().cvt());
            let count_elems = arrive_count.elements();
            for idx in 0..N_MBARS {
                let mbar_ptr = local_ptr.copy().index_static(idx);
                let mbar_mut = unsafe { mbar_ptr.assume_mut() };
                mbar_init(mbar_mut, count_elems[idx].copy());
            }
        });
    });

    cuda.sync_threads();

    unsafe { storage.as_mut_ptr().ptr_cast().assume_mut() }
}
