use crate::{codegen::func_with_args::Func, ty::primitive::I32, val::Val};

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn nanosleep(&self, ns: Val<'_, I32>) {
        // Safety: Nanosleep is a voidlike intrinsic taking an i32.
        unsafe {
            self.cm_ref()
                .call_voidlike_intrinsic::<(I32,)>((ns,), "llvm.nvvm.nanosleep")
        };
    }
}
