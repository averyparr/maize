use inkwell::{
    context::ContextRef,
    types::{BasicMetadataTypeEnum, BasicType, FunctionType},
    values::BasicValue,
};

use crate::{
    codegen::FnCodegen,
    ty::{AnyTy, IntoFuncArgs, ValTy, Void},
    val::Val,
};

pub trait FnRetTy: AnyTy {
    fn raw_fn_ty<'ctx>(
        ctx: ContextRef<'ctx>,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
        is_var_args: bool,
    ) -> FunctionType<'ctx>;

    fn fn_ty<Args: IntoFuncArgs>(ctx: ContextRef<'_>) -> FunctionType<'_> {
        let args = Args::produce_metadata_args(ctx);
        Self::raw_fn_ty(ctx, args.as_ref(), false)
    }

    fn return_from_fn(cx: &FnCodegen, val: Option<Val<'_, Self>>);
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

    fn return_from_fn(cx: &FnCodegen, val: Option<Val<'_, Self>>) {
        let val = val.expect("Returning non-empty value should always pass Some(_)");
        if cx.bb().get_terminator().is_none() {
            unsafe {
                cx.with_builder(|b| b.build_return(Some(&val.ll_typed())))
                    .expect("Return should always succeed")
            };
        }
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

    fn return_from_fn(cx: &FnCodegen, val: Option<Val<'_, Self>>) {
        assert!(val.is_none(), "Cannot return a value through a void type");
        if cx.bb().get_terminator().is_none() {
            unsafe {
                cx.with_builder(|b| b.build_return(None))
                    .expect("Function return should always succeed")
            };
        }
    }
}
