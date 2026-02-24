use std::{borrow::Borrow, cell::Cell};

use inkwell::{
    OptimizationLevel,
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    module::Module,
    passes::PassBuilderOptions,
    targets::{FileType, InitializationConfig, Target, TargetTriple},
    values::{AnyValue, FunctionValue, InstructionValue},
};

use crate::{
    intrinsics::IntrinsicsLibrary,
    ty::{
        Bool, F32, F64, FnRetTy, I8, I16, I32, I64, IntoFuncArgs, U8, U16, U32, U64, ValTy, VoidTy,
    },
    val::Val,
};

use super::instruction_opt::InstructionOpt;

pub struct FnCodegen {
    module: Module<'static>,
    func: FunctionValue<'static>,
    bb: Cell<BasicBlock<'static>>,
    opt: Cell<InstructionOpt>,
}

macro_rules! impl_into_constant {
    ($(
        $trace_ty: ty | $raw_ty: ty => $ty_fn: ident | $val_fn: ident $(($($args: tt),*))?;
    )*) => {
        $(
            impl ConstValTy for $trace_ty {
                type Assoc = $raw_ty;
                fn to_const(assoc: impl Into<Self::Assoc>, cx: &FnCodegen) -> Val<'_, Self> {
                    let val_as_assoc = assoc.into();
                    let raw = cx.ctx().$ty_fn().$val_fn(val_as_assoc as _, $($($args,)*)?);
                    unsafe {Val::new(cx, raw.as_any_value_enum())}
                }
            }

            impl IntoConstVal for $raw_ty {
                type Assoc = $trace_ty;
                fn into_const_val(self, cx: &FnCodegen) -> Val<'_, Self::Assoc> {
                    let raw = cx.ctx().$ty_fn().$val_fn(self as _, $($($args,)*)?);
                    unsafe {Val::new(cx, raw.as_any_value_enum())}
                }
            }
        )*
    };
}

impl_into_constant!(
    F32 | f32 => f32_type | const_float;
    F64 | f64 => f64_type | const_float;
    U8  | u8  => i8_type  | const_int (false);
    U16 | u16 => i16_type | const_int (false);
    U32 | u32 => i32_type | const_int (false);
    U64 | u64 => i64_type | const_int (false);
    I8  | i8  => i8_type  | const_int (false);
    I16 | i16 => i16_type | const_int (false);
    I32 | i32 => i32_type | const_int (false);
    I64 | i64 => i64_type | const_int (false);
    Bool | bool => bool_type | const_int (false);
);

pub trait ConstValTy: ValTy {
    type Assoc;
    fn to_const(assoc: impl Into<Self::Assoc>, cx: &FnCodegen) -> Val<'_, Self>;
}

pub trait IntoConstVal {
    type Assoc: ValTy;
    fn into_const_val(self, cx: &FnCodegen) -> Val<'_, Self::Assoc>;
}

impl FnCodegen {
    pub(crate) fn ctx(&self) -> ContextRef<'static> {
        self.module.get_context()
    }
    pub(crate) fn func(&self) -> FunctionValue<'static> {
        self.func
    }
    pub(crate) fn bb(&self) -> BasicBlock<'static> {
        self.bb.get()
    }
    pub(crate) fn module(&self) -> &Module<'static> {
        &self.module
    }
    pub(crate) fn with_bb_as<U>(&self, bb: BasicBlock<'static>, f: impl FnOnce() -> U) -> U {
        let curr_bb = self.bb();
        self.set_bb(bb);
        let ret = f();
        self.set_bb(curr_bb);
        ret
    }
    pub(crate) fn set_bb(&self, bb: BasicBlock<'static>) {
        self.bb.set(bb);
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
    pub fn constant<C: ConstValTy>(&self, val: impl Into<C::Assoc>) -> Val<'_, C> {
        C::to_const(val, self)
    }
    pub fn constant_from<CVal: IntoConstVal>(&self, val: CVal) -> Val<'_, CVal::Assoc> {
        CVal::into_const_val(val, self)
    }
    /// # Safety:
    /// Giving access to the builder lets you emit very unsound code.
    /// Calling this function safely is only possible if F doesn't cause the builder
    /// to emit unsafe code.
    pub(crate) unsafe fn with_builder<F: FnOnce(Builder<'static>) -> U, U>(&self, f: F) -> U {
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
    type Intrinsics: IntrinsicsLibrary;
    type Args: IntoFuncArgs;
    type Ret: FnRetTy;

    fn new(cg: FnCodegen) -> Self;
    fn cx(&self) -> &FnCodegen;
    fn intrinsics(&self) -> &Self::Intrinsics;
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
        self.cx().func().add_attribute(
            inkwell::attributes::AttributeLoc::Function,
            self.cx()
                .ctx()
                .create_string_attribute("denormal-fp-math-f32", "preserve-sign,preserve-sign"),
        );
    }

    fn return_with<'a>(&self, retval: Val<'_, Self::Ret>)
    where
        Self::Ret: ValTy,
    {
        Self::Ret::return_from_fn(self.cx(), Some(retval))
    }

    fn return_void(&self)
    where
        Self::Ret: VoidTy,
    {
        Self::Ret::return_from_fn(self.cx(), None);
    }

    fn finalize_with<'a>(self, val: Val<'a, Self::Ret>) -> PreJitFunction<Self>
    where
        Self::Ret: ValTy,
    {
        self.return_with(val);
        PreJitFunction(self)
    }

    fn finalize(self) -> PreJitFunction<Self>
    where
        Self::Ret: VoidTy,
    {
        self.return_void();
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
        let machine = target
            .create_target_machine(
                &triple,
                cpu.cpu(),
                cpu.features(),
                optimization_level,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .expect("Could not create a compiler with the given option");

        let passes = match optimization_level {
            OptimizationLevel::None => "default<O0>",
            OptimizationLevel::Less => "default<O1>",
            OptimizationLevel::Default => "default<O2>",
            OptimizationLevel::Aggressive => "default<O3>",
        };

        self.0
            .cx()
            .module
            .run_passes(passes, &machine, PassBuilderOptions::create())
            .expect("Unable to run passes on module");

        let maybe_ret = machine
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
