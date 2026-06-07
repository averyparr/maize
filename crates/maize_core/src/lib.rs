#![expect(unused)]

use inkwell::values::FastMathFlags;

use crate::{
    backend::{LLVM, Opt, cpu::cuda::SM},
    func::implement_ptx_kernel,
    intrinsics::{
        Intrinsic,
        cuda::mma::{
            mma_sync::{
                LegacyMmaSyncM8N8K4ColColFP16FP16FP16FP16,
                LegacyMmaSyncM8N8K4ColColFP32FP16FP16FP16,
                LegacyMmaSyncM8N8K4ColColFP32FP16FP16FP32, MmaSyncM8N8K4RowColFP64FP64FP64FP64,
            },
            structs::{LegacyMmaAccumF32M8N8, MmaAccumF32M16N8, MmaAccumF64M8N8, MmaAccumF64M16N8},
            tcgen05::{TcGen05MmaSA, TcGen05MmaSAScaleD},
        },
    },
    tipe::{A, F16, V},
};

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
    )>(LLVM::new(), "func");
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
            vptr,
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
        smem.store(vvec);
        vptr.store(smem.load());
        // stor.store(t);
    }
    func.return_void();
    func.run_passes(SM::SM90a, Opt::O3);
    println!("{}", func.module_string());
    let b = func.compile(SM::SM90a, Opt::O3);
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
