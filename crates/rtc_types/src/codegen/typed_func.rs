use std::{borrow::Borrow, cell::Cell, marker::PhantomData};

use inkwell::{
    OptimizationLevel,
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    module::Module,
    targets::{FileType, InitializationConfig, Target, TargetMachine, TargetTriple},
    types::{BasicType, IntType},
    values::{AnyValue, BasicValue, BasicValueEnum, FunctionValue, InstructionValue, IntValue},
};

use crate::{
    ty::{F32, F64, FnRetTy, I8, I16, I32, I64, IntoFuncArgs, U8, U16, U32, U64, ValTy, VoidTy},
    val::Val,
};

use super::instruction_opt::InstructionOpt;

pub(crate) struct FnCodegen {
    module: Module<'static>,
    func: FunctionValue<'static>,
    bb: Cell<BasicBlock<'static>>,
    opt: Cell<InstructionOpt>,
}

macro_rules! impl_into_constant {
    ($(
        $raw_ty: ty | $trace_ty: ty => $ty_fn: ident | $val_fn: ident $(($($args: tt),*))?;
    )*) => {
        $(
            impl IntoConstantVal for $raw_ty {
                type Assoc = $trace_ty;
                fn to_const(self, cx: &FnCodegen) -> Val<'_, Self::Assoc> {
                    let raw = cx.ctx().$ty_fn().$val_fn(self as _, $($($args,)*)?);
                    unsafe {Val::new(cx, raw.as_any_value_enum())}
                }
            }
        )*
    };
}

impl_into_constant!(
    f32 | F32 => f32_type | const_float;
    f64 | F64 => f64_type | const_float;
    u8  | U8  => i8_type  | const_int (false);
    u16 | U16 => i16_type | const_int (false);
    u32 | U32 => i32_type | const_int (false);
    u64 | U64 => i64_type | const_int (false);
    i8  | I8  => i8_type  | const_int (false);
    i16 | I16 => i16_type | const_int (false);
    i32 | I32 => i32_type | const_int (false);
    i64 | I64 => i64_type | const_int (false);
);

pub trait IntoConstantVal {
    type Assoc: ValTy;
    fn to_const(self, cx: &FnCodegen) -> Val<'_, Self::Assoc>;
}

impl FnCodegen {
    pub(crate) fn ctx(&self) -> ContextRef<'_> {
        self.module.get_context()
    }
    pub(crate) fn func(&self) -> FunctionValue<'_> {
        self.func
    }
    fn bb(&self) -> BasicBlock<'_> {
        self.bb.get()
    }
    pub fn apply_ins_opt(&self, ins: InstructionValue<'_>) {
        self.opt.get().post_process_instruction(ins);
    }
    pub fn change_opt<F: FnOnce(&mut InstructionOpt)>(&self, f: F) {
        let mut opt = self.opt.get();
        f(&mut opt);
        self.opt.set(opt);
    }
    pub fn use_fast_math(&self) {
        self.change_opt(|o| o.use_all_fast_math());
    }
    pub fn constant<C: IntoConstantVal>(&self, val: C) -> Val<'_, C::Assoc> {
        C::to_const(val, self)
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
        let opt = Cell::default();
        let cg = FnCodegen {
            module,
            func,
            bb,
            opt,
        };
        Self::new(cg)
    }
    fn get_args<'val>(&'val self) -> <Self::Args as IntoFuncArgs>::ArgValues<'val> {
        Self::Args::try_extract_args(self.cx()).expect("Should match my own arg count")
    }
    fn llvm_ir(&self) -> String {
        self.cx().func.to_string()
    }
    fn change_opt<F: FnOnce(&mut InstructionOpt)>(&self, f: F) {
        self.cx().change_opt(f);
    }
    fn use_fast_math(&self) {
        self.cx().use_fast_math();
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
