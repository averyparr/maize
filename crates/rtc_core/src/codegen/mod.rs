pub mod context;
pub mod func_with_args;
pub mod intrinsics;
pub mod pre_jit_func;
pub mod target;

use std::cell::Cell;

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    module::Module,
    values::{BasicValueEnum, FunctionValue, PointerValue},
};

use crate::codegen::context::create_context;

/// Very important that it _not_ be clone or copy!
#[derive(PartialEq)]
pub struct FnCodegen<'ctx> {
    ctx: ContextRef<'ctx>,
    func: FunctionValue<'ctx>,
    bb: Cell<BasicBlock<'ctx>>,
}

/// Very important that it _not_ be clone or copy!
#[derive(PartialEq)]
pub struct CodegenModule<'ctx> {
    codegen: FnCodegen<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> FnCodegen<'ctx> {
    pub(crate) fn new(f: impl for<'a> FnOnce(ContextRef<'a>) -> FunctionValue<'a>) -> Self {
        let ctx = create_context();
        let func = f(ctx);
        let bb = ctx.append_basic_block(func, "entry");

        Self {
            ctx,
            func,
            bb: Cell::new(bb),
        }
    }
    pub fn ctx(&self) -> ContextRef<'ctx> {
        self.ctx
    }
    pub(crate) fn func(&self) -> FunctionValue<'ctx> {
        self.func
    }
    fn new_bb_with_suffix(&self, curr_name: &mut String, suffix: &str) -> BasicBlock<'ctx> {
        curr_name.push_str(suffix);
        let new_block = self.ctx().append_basic_block(self.func(), &curr_name);
        curr_name.truncate(curr_name.len() - suffix.len());
        new_block
    }
    fn bb(&self) -> BasicBlock<'ctx> {
        self.bb.get()
    }
    fn set_bb(&self, bb: BasicBlock<'ctx>) {
        self.bb.set(bb);
    }
    fn curr_bb_name(&self) -> String {
        self.bb()
            .get_name()
            .to_str()
            .expect("Should have only utc-8 basic blocks")
            .to_owned()
    }
    #[expect(dead_code, reason = "Please remove if found later")]
    pub(crate) fn with_branch(&self, if_true: impl FnOnce(), if_false: impl FnOnce()) {
        let mut curr_name = self.curr_bb_name();
        let true_block = self.new_bb_with_suffix(&mut curr_name, "_true");
        let false_block = self.new_bb_with_suffix(&mut curr_name, "_false");
        let merge_block = self.new_bb_with_suffix(&mut curr_name, "_merge");
        self.set_bb(true_block);
        if_true();
        // SAFETY: I[@averyparr] _believe_ this is unconditionally safe
        unsafe {
            self.with_builder(|b| {
                b.build_unconditional_branch(merge_block)
                    .expect("Should be able to branch to merge block")
            });
        }
        self.set_bb(false_block);
        if_false();
        // SAFETY: I[@averyparr] _believe_ this is unconditionally safe
        unsafe {
            self.with_builder(|b| {
                b.build_unconditional_branch(merge_block)
                    .expect("Should be able to branch to merge block")
            });
        }
        self.set_bb(merge_block);
    }
    /// # Safety:
    /// Giving access to the builder lets you emit very unsound code.
    /// Calling this function safely is only possible if F doesn't cause the builder
    /// to emit unsafe code.
    pub(crate) unsafe fn with_builder<F: FnOnce(Builder<'ctx>) -> U, U>(&self, f: F) -> U {
        let builder = self.ctx().create_builder();
        builder.position_at_end(self.bb());
        f(builder)
    }

    pub(crate) fn build_alloca(&self, value: BasicValueEnum<'ctx>) -> PointerValue<'ctx> {
        // SAFETY: Unconditionally safe to create allocas
        unsafe {
            self.with_builder(|b| b.build_alloca(value.get_type(), "build_alloca"))
                .expect("Should be able to build alloca")
        }
    }
}

impl<'ctx> CodegenModule<'ctx> {
    pub(crate) fn new(module: Module<'ctx>, codegen: FnCodegen<'ctx>) -> Self {
        Self { codegen, module }
    }
    pub(crate) fn extract_module_codegen(self) -> (Module<'ctx>, FnCodegen<'ctx>) {
        (self.module, self.codegen)
    }
    pub(crate) fn module(&self) -> &Module<'ctx> {
        &self.module
    }
    pub(crate) fn cx(&self) -> &FnCodegen<'ctx> {
        &self.codegen
    }
}
