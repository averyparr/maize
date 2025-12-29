use inkwell::{types::BasicType, values::BasicValue};

use crate::{
    codegen::func_with_args::Func,
    ty::{FromCtx, I32, Ty},
    val::{Holds, Val},
};

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    /// Sleep for approximately the specified number of nanoseconds.
    ///
    /// Note: This is a best-effort operation. Actual sleep time may vary
    /// based on GPU scheduling and is typically rounded to clock cycles.
    pub fn nanosleep(&self, ns: Val<'_, I32>) {
        let void_ty = self.cx_ref().ctx().void_type();
        let i32_ty = I32::new(self.cx_ref().ctx()).basic_ty();
        let fn_ty = void_ty.fn_type(&[i32_ty.into()], false);
        let fn_val = self
            .mod_ref()
            .add_function("llvm.nvvm.nanosleep", fn_ty, None);

        // Safety: nanosleep is a well-defined intrinsic with no memory effects.
        unsafe {
            self.cx_ref().with_builder(|b| {
                b.build_call(
                    fn_val,
                    &[ns.to_underlying().as_basic_value_enum().into()],
                    "",
                )
            })
        }
        .expect("Could not generate nanosleep call");
    }
}
