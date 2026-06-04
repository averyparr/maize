pub mod backend;
pub mod control_flow;
pub mod func;
pub mod intrinsics;
pub mod tipe;
pub mod val;

#[cfg(test)]
mod test {
    use inkwell::values::FastMathFlags;

    use crate::{
        backend::{Opt, cpu::cuda::SM, llvm::LLVM},
        control_flow::If,
        func::implement_ptx_kernel,
        intrinsics::cuda::CUDA,
        tipe::{A, BF16, FromF32, V},
    };
    #[test]
    fn is_aligned() {
        let func = implement_ptx_kernel::<(
            &V<BF16, 4>,
            &V<f32, 4>,
            A<*const f32, 3>,
            A<*mut u64, 3>,
            A<*mut u64, 7>,
            A<&V<BF16, 4>, 1>,
            A<&BF16, 1>,
            A<&mut u64, 1>,
            A<&mut V<BF16, 4>, 1>,
        )>(LLVM::new(), "func");
        func.set_ins_flags(|flags| {
            flags.fast_math = FastMathFlags::all();
            flags.unsigned_wrap = true;
            flags.signed_wrap = true;
        });
        {
            let (e, f, addr, mbar, cluster_mbar, b, c, mut mbar_stor, mut d) = func.args();
            let cluster = func.constant(3);
            // mbar_stor.store(CUDA::mbar_arrive(mbar.copy()));
            // mbar_stor.store();
            // let one = func.constant(3);
            // let token = CUDA::mbar_arrive(mbar.copy());
            // let waited = CUDA::mbar_try_wait(mbar.copy(), token.copy());
            // let waited = CUDA::mbar_try_wait_timed(mbar.copy(), token.copy(), one.copy());
            // let waited = CUDA::mbar_try_wait_parity_timed(mbar.copy(), waited, one.copy());
            // CUDA::mbar_init(mbar.copy(), token.cvt());
            // If(waited).then(|| CUDA::mbar_inval(mbar.copy()));
            // // CUDA::mbar_expect_tx(mbar, one.copy());
            // // CUDA::cluster_mbar_expect_tx(mbar_cluster, one.copy());
            // let cluster_ptr = addr.mapa(cluster);
            d.store(f.load().cvt().nvvm_log2());
            // d.store(unsafe { cluster_ptr.read().tanh().splat().cvt() });
        }
        func.return_void();
        func.run_passes(SM::SM90, Opt::O3);
        println!("{}", func.module_string());
        let b = func.compile(SM::SM90, Opt::O3);
        println!("{}", str::from_utf8(&b).expect("Should be valid"));
        assert!(false);
    }
}
