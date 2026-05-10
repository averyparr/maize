use inkwell::{context::ContextRef, types::VoidType};

use super::{AnyTy, raw::Void};

impl AnyTy for Void {
    type AnyType<'ctx> = VoidType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        ctx.void_type()
    }
}

pub trait VoidTy: AnyTy {}
impl VoidTy for Void {}
