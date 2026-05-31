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
        func::implement_ptx_kernel,
        tipe::{A, BF16, FromF32, Ty, V},
    };
    #[test]
    fn is_aligned() {
        let func = implement_ptx_kernel::<(
            A<&i32, 1>,
            i32,
            A<&V<BF16, 4>, 1>,
            A<&BF16, 1>,
            A<&mut V<BF16, 4>, 1>,
        )>(LLVM::new(), "func");
        func.set_ins_flags(|flags| {
            flags.fast_math = FastMathFlags::all();
            flags.unsigned_wrap = true;
            flags.signed_wrap = true;
        });
        {
            let (a, v, b, c, mut d) = func.args();
            let single = BF16::const_val(BF16::from_f32(5.0), a.fn_ref().clone());
            let cond = a.load().eq(v);
            // func.cuda()
            //     .assert(cond.copy(), "assert_failed", file!(), line!(), "function");
            unsafe { func.cuda().assume(cond.copy()) };
            let c = c.load_nc().splat();
            d.store(c + single.splat() * b.load_nc().shfl_idx_uniform(func.constant(0)))
        }
        func.return_void();
        func.run_passes(SM::SM90, Opt::O3);
        println!("{}", func.module_string());
        let b = func.compile(SM::SM90, Opt::O3);
        println!("{}", str::from_utf8(&b).expect("Should be valid"));
        assert!(false);
    }
}
