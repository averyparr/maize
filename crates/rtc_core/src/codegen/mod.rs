pub mod context;
pub mod func_with_args;
pub mod intrinsics;
pub mod pre_jit_func;
pub mod target;

use std::cell::Cell;

use inkwell::{
    basic_block::BasicBlock,
    builder::{Builder, BuilderError},
    context::ContextRef,
    module::Module,
    types::BasicType,
    values::{BasicValue, BasicValueEnum, FunctionValue, InstructionValue, PointerValue},
};

use crate::codegen::context::create_context;

/// Very important that it _not_ be clone or copy!
#[derive(PartialEq)]
pub(crate) struct FnCodegen<'ctx> {
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

    /// # Safety:
    /// It must be valid to load a type `pointee_ty` through `ptr`
    /// as-if a *const T
    pub(crate) unsafe fn try_load<'a>(
        &self,
        pointee_ty: impl BasicType<'ctx>,
        ptr: PointerValue<'ctx>,
        align: Option<u32>,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> Result<BasicValueEnum<'ctx>, BuilderError>
    where
        'ctx: 'a,
    {
        // SAFETY: It's valid to emit a load
        unsafe { self.with_builder(move |b| b.build_load(pointee_ty, ptr, "try_ld")) }.map(|v| {
            if let Some(ins) = v.as_instruction_value() {
                if let Some(align) = align {
                    ins.set_alignment(align)
                        .expect("Cannot set load alignment!");
                }
                if let Some(for_ins) = for_ins {
                    for_ins(ins)
                }
            }
            v
        })
    }
    /// # Safety:
    /// It must be valid to load a type `pointee_ty` through `ptr`
    /// as-if a *const T
    pub(crate) unsafe fn load<'a>(
        &self,
        pointee_ty: impl BasicType<'ctx>,
        ptr: PointerValue<'ctx>,
        align: Option<u32>,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> BasicValueEnum<'ctx>
    where
        'ctx: 'a,
    {
        unsafe {
            // SAFETY: Identical precondition.
            self.try_load(pointee_ty, ptr, align, for_ins)
                .expect("Unable to generate load")
        }
    }
    /// # Safety:
    /// It must be valid to write a value `value` through `ptr`
    /// as-if a *mut T
    pub(crate) unsafe fn try_store<'a>(
        &self,
        ptr: PointerValue<'ctx>,
        value: impl BasicValue<'ctx>,
        align: Option<u32>,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) -> Result<(), BuilderError>
    where
        'ctx: 'a,
    {
        // SAFETY: It's valid to emit a store
        let ret = unsafe { self.with_builder(|b| b.build_store(ptr, value)) };

        ret.map(|i| {
            if let Some(align) = align {
                i.set_alignment(align);
            }
            if let Some(for_ins) = for_ins {
                for_ins(i);
            }
        })
    }
    /// # Safety:
    /// It must be valid to write a value `value` through `ptr`
    /// as-if a *mut T
    pub(crate) unsafe fn store<'a>(
        &self,
        ptr: PointerValue<'ctx>,
        value: impl BasicValue<'ctx>,
        align: Option<u32>,
        for_ins: Option<&dyn Fn(InstructionValue<'a>)>,
    ) where
        'ctx: 'a,
    {
        // SAFETY: identical precondition.
        unsafe {
            self.try_store(ptr, value, align, for_ins)
                .expect("Cannot generate store")
        }
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
    pub fn cx(&self) -> &FnCodegen<'ctx> {
        &self.codegen
    }

    pub(crate) unsafe fn create_value<
        F: FnOnce(Builder<'ctx>) -> U,
        OnIns: FnOnce(InstructionValue<'ctx>),
        U: BasicValue<'ctx>,
    >(
        &self,
        f: F,
        on_ins: OnIns,
    ) -> U {
        // SAFETY: User promised!
        let val = unsafe { self.cx().with_builder(f) };
        if let Some(ins) = val.as_instruction_value() {
            on_ins(ins);
        }
        val
    }
}
