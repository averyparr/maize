pub mod calling_convention;
pub mod if_stmt;
pub mod instruction_opt;
pub mod loops;
pub mod target;
pub mod typed_func;

pub(crate) use typed_func::FnCodegen;
pub use typed_func::Func;

use inkwell::context::{Context, ContextRef};

use crate::ty::{FnRetTy, IntoFuncArgs};

pub struct Codegen(Context);

impl Codegen {
    pub fn new() -> Self {
        Self(Context::create())
    }
    fn function_from_cached<Function: Func>() -> Function {
        thread_local! {
            static CG: &'static Codegen = Box::leak(Box::new(Codegen::new()));
        }
        CG.with(|cg| Func::from_ctx(cg.ctx()))
    }
    pub fn ctx<'a>(&'a self) -> ContextRef<'a> {
        // Safety: We only give out the raw pointer for the duration
        // of the borrow, so its lifetime is tied to the lifetime
        // of the borrow.
        unsafe { ContextRef::new(self.0.raw()) }
    }
}

macro_rules! accessible_func_creator {
    ($fn_name: ident => $tipe: ident<Args>) => {
        pub fn $fn_name<Args: IntoFuncArgs>() -> $tipe<Args> {
            Codegen::function_from_cached::<$tipe<Args>>()
        }
    };

    ($fn_name: ident => $tipe: ident<Args, Ret>) => {
        pub fn $fn_name<Args: IntoFuncArgs, Ret: FnRetTy>() -> $tipe<Args, Ret> {
            Codegen::function_from_cached::<$tipe<Args, Ret>>()
        }
    };
}

use calling_convention::{PTXDevice, PTXKernel};
accessible_func_creator!(new_ptx_kernel => PTXKernel<Args>);
accessible_func_creator!(new_ptx_device => PTXDevice<Args, Ret>);
