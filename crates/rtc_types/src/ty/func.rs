use inkwell::{
    context::ContextRef,
    types::{BasicMetadataTypeEnum, BasicType, FunctionType},
    values::BasicValue,
};

use crate::ty::{AnyTy, IntoFuncArgs, ValTy, Void};

pub trait FnRetTy {
    fn raw_fn_ty<'ctx>(
        ctx: ContextRef<'ctx>,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
        is_var_args: bool,
    ) -> FunctionType<'ctx>;

    fn fn_ty<Args: IntoFuncArgs>(ctx: ContextRef<'_>) -> FunctionType<'_> {
        let args = Args::produce_args(ctx);
        Self::raw_fn_ty(ctx, args.as_ref(), false)
    }
}

impl<T> FnRetTy for T
where
    T: ValTy,
    for<'ctx> T::Value<'ctx>: BasicValue<'ctx>,
{
    fn raw_fn_ty<'ctx>(
        ctx: ContextRef<'ctx>,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
        is_var_args: bool,
    ) -> FunctionType<'ctx> {
        T::ty(ctx).fn_type(param_types, is_var_args)
    }
}

impl FnRetTy for Void {
    fn raw_fn_ty<'ctx>(
        ctx: ContextRef<'ctx>,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
        is_var_args: bool,
    ) -> FunctionType<'ctx> {
        Self::any_ty(ctx).fn_type(param_types, is_var_args)
    }
}
