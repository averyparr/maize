use std::{borrow::Borrow, cell::Cell, marker::PhantomData};

use inkwell::{
    OptimizationLevel,
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    module::Module,
    targets::{FileType, InitializationConfig, Target, TargetMachine, TargetTriple},
    values::FunctionValue,
};

use crate::ty::{FnRetTy, IntoFuncArgs, ValTy, VoidTy};

pub(crate) struct FnCodegen {
    module: Module<'static>,
    func: FunctionValue<'static>,
    bb: Cell<BasicBlock<'static>>,
}

impl<'ctx> FnCodegen {
    pub(crate) fn ctx<'a>(&'a self) -> ContextRef<'a> {
        self.module.get_context()
    }
    pub(crate) fn func(&self) -> FunctionValue<'ctx> {
        self.func
    }
    fn bb(&self) -> BasicBlock<'ctx> {
        self.bb.get()
    }

    /// # Safety:
    /// Giving access to the builder lets you emit very unsound code.
    /// Calling this function safely is only possible if F doesn't cause the builder
    /// to emit unsafe code.
    pub(crate) unsafe fn with_builder<'a, F: FnOnce(Builder<'a>) -> U, U>(&'a self, f: F) -> U {
        let builder = self.ctx().create_builder();
        builder.position_at_end(self.bb());
        f(builder)
    }
}

pub trait ToCPU {
    fn cpu(&self) -> &str;
    fn triple(&self) -> &str;
    fn features(&self) -> &str {
        ""
    }
}

pub trait Func: Sized {
    type Args: IntoFuncArgs;
    type Ret: FnRetTy;

    fn new(cg: FnCodegen) -> Self;
    fn cx(&self) -> &FnCodegen;
    const CALL_CONV: u32;
    type CpuConfig: ToCPU;

    fn initialize(cpu: &Self::CpuConfig) {
        let config: &InitializationConfig = &InitializationConfig::default();
        match cpu.triple() {
            "nvptx64-nvidia-cuda" => Target::initialize_nvptx(config),
            _ => panic!("Unrecognized [default-impl] target '{}'", cpu.triple()),
        }
    }

    fn from_ctx(ctx: ContextRef<'static>) -> Self
    where
        Self: Sized,
    {
        let module = ctx.create_module("func");
        let fn_ty = Self::Ret::fn_ty::<Self::Args>(ctx);
        let func = module.add_function("func", fn_ty, None);
        func.set_call_conventions(Self::CALL_CONV);
        let bb = ctx.append_basic_block(func, "entry");
        let bb = Cell::new(bb);
        let cg = FnCodegen { module, func, bb };
        Self::new(cg)
    }
    fn get_args<'val>(&'val self) -> <Self::Args as IntoFuncArgs>::ArgValues<'val> {
        Self::Args::try_extract_args(self.cx()).expect("Should match my own arg count")
    }
    fn llvm_ir(&self) -> String {
        self.cx().func.to_string()
    }

    fn finalize(self) -> PreJitFunction<Self>
    where
        Self::Ret: VoidTy,
    {
        unsafe { self.cx().with_builder(|b| b.build_return(None)) }
            .expect("Should be possible to return nothing");
        PreJitFunction(self)
    }
}

pub struct PreJitFunction<F>(F);

impl<F> PreJitFunction<F>
where
    F: Func,
{
    pub fn compile(
        self,
        cpu: &F::CpuConfig,
        optimization_level: OptimizationLevel,
        file_type: FileType,
    ) -> Box<[u8]> {
        F::initialize(cpu);

        let triple = TargetTriple::create(cpu.triple());
        let target = Target::from_triple(&triple).expect("cpu.triple() invalid for LLVM");
        let compiler = target
            .create_target_machine(
                &triple,
                cpu.cpu(),
                cpu.features(),
                optimization_level,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .expect("Could not create a compiler with the given option");

        let maybe_ret = compiler
            .write_to_memory_buffer(&self.0.cx().module, file_type)
            .expect("Unable to compile");
        maybe_ret.as_slice().to_vec().into_boxed_slice()
    }
    pub fn compile_asm_at_opt(
        self,
        cpu: impl Borrow<F::CpuConfig>,
        optimization_level: OptimizationLevel,
    ) -> String {
        String::from_utf8(Vec::from(self.compile(
            cpu.borrow(),
            optimization_level,
            FileType::Assembly,
        )))
        .expect("asm should always be utf-8")
    }
    pub fn compile_obj_at_opt(
        self,
        cpu: impl Borrow<F::CpuConfig>,
        optimization_level: OptimizationLevel,
    ) -> Box<[u8]> {
        self.compile(cpu.borrow(), optimization_level, FileType::Object)
    }
    pub fn compile_asm_quickly(self, cpu: impl Borrow<F::CpuConfig>) -> String {
        self.compile_asm_at_opt(cpu, OptimizationLevel::Less)
    }
    pub fn compile_asm(self, cpu: impl Borrow<F::CpuConfig>) -> String {
        self.compile_asm_at_opt(cpu, OptimizationLevel::Default)
    }
    pub fn compile_asm_optimized(self, cpu: impl Borrow<F::CpuConfig>) -> String {
        self.compile_asm_at_opt(cpu, OptimizationLevel::Aggressive)
    }
    pub fn compile_obj_quickly(self, cpu: impl Borrow<F::CpuConfig>) -> Box<[u8]> {
        self.compile_obj_at_opt(cpu, OptimizationLevel::Less)
    }
    pub fn compile_obj(self, cpu: impl Borrow<F::CpuConfig>) -> Box<[u8]> {
        self.compile_obj_at_opt(cpu, OptimizationLevel::Default)
    }
    pub fn compile_obj_optimized(self, cpu: impl Borrow<F::CpuConfig>) -> Box<[u8]> {
        self.compile_obj_at_opt(cpu, OptimizationLevel::Aggressive)
    }
}
