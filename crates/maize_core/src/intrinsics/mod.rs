pub mod cuda;
pub mod llvm;

use inkwell::types::FunctionType;

use crate::{
    func::{FnArgs, FnRetTy},
    val::Val,
};

pub trait Intrinsic {
    type Args: FnArgs;
    type Ret: FnRetTy;
    fn call(self, args: <Self::Args as FnArgs>::ArgValues) -> <Self::Ret as FnRetTy>::RetVal;
}

pub trait IntrinsicsLibrary {
    unsafe fn assume(&self, cond: Val<bool>);
    fn assert(&self, cond: Val<bool>, message: &str, file: &str, line: u32, function: &str);
}

#[derive(Clone, Copy, Debug)]
pub enum IntrinsicError {
    NameNotFound,
    DeclarationNotFound,
    MismatchedType(FunctionType<'static>, FunctionType<'static>),
}

macro_rules! impl_intrinsics {
    ($($struct_name: ident : $name: literal ($($args: ty),*) -> $ret: ty),* $(,)?) => {
        $(
            #[derive(Default)]
            pub struct $struct_name;
            impl $crate::intrinsics::Intrinsic for $struct_name {
                type Args = ($($args,)*);
                type Ret = $ret;
                fn call(self, args: <Self::Args as $crate::func::FnArgs>::ArgValues) -> <Self::Ret as $crate::func::FnRetTy>::RetVal {
                    let fn_ref = args.0.fn_ref().clone();
                    let func = fn_ref
                        .get_intrinsic::<Self::Ret, Self::Args>($name, false)
                        .expect(concat!("intrinsic '", $name, "' should exist"));
                    fn_ref.call_extern(func, args, None)
                }
            }
        )*
    };
}

macro_rules! impl_argless_intrinsics {
    ($($struct_name: ident: $name: literal() -> $ret: ty),* $(,)?) => {
        $(
            pub struct $struct_name(pub(crate) $crate::backend::FnRef);
            impl $crate::intrinsics::Intrinsic for $struct_name {
                type Args = ();
                type Ret = $ret;
                fn call(self, _: ()) -> <Self::Ret as $crate::func::FnRetTy>::RetVal {
                    let fn_ref = self.0;
                    let func = fn_ref
                        .get_intrinsic::<$ret, ()>($name, false)
                        .expect(concat!("intrinsic '", $name, "' should exist"));
                    fn_ref.call_extern(func, (), None)
                }
            }
        )*
    };
}

pub(crate) use {impl_argless_intrinsics, impl_intrinsics};
