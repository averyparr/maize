use crate::{
    backend::{FnRef, VoidType},
    control_flow::If,
    func::callconv::CallConv,
    intrinsics::IntrinsicsLibrary,
    tipe::A,
    val::Val,
};

pub mod addr;
pub mod barrier;
pub mod bits;
pub mod cache;
pub mod clc;
pub mod cp_async;
pub mod cvt;
pub mod grid;
pub mod math;
pub mod mbar;
pub mod mma;
pub mod nanosleep;
pub mod set_max_reg;
pub mod shfl;
pub mod tma;
pub mod vote;

impl IntrinsicsLibrary for CUDA {
    unsafe fn assume(&self, cond: Val<bool>) {
        unsafe { self.assume(cond) };
    }

    fn assert(&self, cond: Val<bool>, message: &str, file: &str, line: u32, function: &str) {
        self.assert(cond, message, file, line, function);
    }
}

pub struct CUDA(pub(crate) FnRef);

impl CUDA {
    pub unsafe fn assume(&self, cond: Val<bool>) {
        let func = self
            .0
            .get_intrinsic::<VoidType, (bool,)>("llvm.assume", false)
            .expect("llvm.assume should exist");
        self.0.call_extern(func, (cond,), None)
    }
    pub fn assert(&self, cond: Val<bool>, message: &str, file: &str, line: u32, function: &str) {
        let raw_msg = self.0.insert_const_str_in_space::<1>(message, "assert_msg");
        let raw_file = self.0.insert_const_str_in_space::<1>(file, "assert_file");
        let raw_line = self.0.constant(line);
        let raw_line = self
            .0
            .insert_const_value_in_space::<1, _>(raw_line, "assert_line");
        let raw_func = self
            .0
            .insert_const_str_in_space::<1>(function, "assert_func");
        let assert_false = self
            .0
            .declare_extern::<VoidType, (A<*const u8,1>, A<*const u8,1>, u32, A<*const u8,1>, u32)>(
                "__assertfail",
            );
        let char_size: Val<u32> = self.0.constant(1);
        If(cond.copy()).then(|| ()).or_else(|| {
            let _ = self.0.call_extern(
                assert_false,
                (raw_msg, raw_file, raw_line.load(), raw_func, char_size),
                Some(CallConv::Cold),
            );
        });
        unsafe { self.assume(cond) };
    }
}
