use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    module::Module,
    types::{AnyType, BasicType},
    values::{BasicValue, BasicValueEnum, FunctionValue, InstructionValue, PointerValue},
};

pub mod jit_func;

#[derive(Clone, Copy)]
pub struct Codegen<'ctx> {
    ctxt: ContextRef<'ctx>,
    func: FunctionValue<'ctx>,
    bb: BasicBlock<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(
        ctxt: ContextRef<'ctx>,
        f: impl FnOnce(ContextRef<'ctx>) -> FunctionValue<'ctx>,
    ) -> Self {
        let func = f(ctxt);
        let bb = ctxt.append_basic_block(func, "entry");
        Self { ctxt, func, bb }
    }
    pub fn ctx(&self) -> ContextRef<'ctx> {
        self.ctxt
    }
    pub fn func(self) -> FunctionValue<'ctx> {
        self.func
    }
    pub fn with_builder<F: FnOnce(Builder<'ctx>) -> U, U>(&self, f: F) -> U {
        let builder = self.ctxt.create_builder();
        builder.position_at_end(self.bb);
        f(builder)
    }
    pub fn load(&self, ty: impl BasicType<'ctx>, ptr: PointerValue<'ctx>) -> BasicValueEnum<'ctx> {
        self.with_builder(|b| {
            b.build_load(ty, ptr, "ld")
                .expect("Should be able to load at pointer value")
        })
    }
    pub fn store(
        &self,
        ptr: PointerValue<'ctx>,
        value: impl BasicValue<'ctx>,
    ) -> InstructionValue<'ctx> {
        self.with_builder(|b| {
            b.build_store(ptr, value)
                .expect("Should be able to store to pointer value")
        })
    }
}
