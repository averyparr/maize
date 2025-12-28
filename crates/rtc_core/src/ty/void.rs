use inkwell::context::ContextRef;

use crate::ty::{FnReturnTy, FromCtx};

pub struct Void(ContextRef<'static>);

impl FromCtx for Void {
    fn new(ctx: ContextRef<'static>) -> Self {
        Self(ctx)
    }
}

impl FnReturnTy for Void {
    fn func_type(
        &self,
        args: &[inkwell::types::BasicMetadataTypeEnum<'static>],
    ) -> inkwell::types::FunctionType<'static> {
        self.0.void_type().fn_type(args, false)
    }
}
