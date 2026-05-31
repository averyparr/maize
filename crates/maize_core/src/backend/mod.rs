pub mod cpu;
mod ins;
pub mod llvm;
mod opt;
mod untyped;

pub use untyped::{ErasedFuncType, ErasedType, UntypedFunc, UntypedValue};

use std::{cell::Cell, ops::Deref, rc::Rc};

use inkwell::{
    AddressSpace,
    attributes::{Attribute, AttributeLoc},
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    intrinsics::Intrinsic,
    passes::PassBuilderOptions,
    targets::{FileType, InitializationConfig, Target, TargetMachine, TargetTriple},
    types::{AnyTypeEnum, BasicType},
    values::BasicValue,
};

pub use crate::backend::ins::InstructionFlags;
pub use crate::backend::{cpu::ToCPU, llvm::LLVM, opt::Opt};
use crate::{
    func::{ExternFunc, FnArgs, FnRetTy, callconv::CallConv},
    intrinsics::IntrinsicError,
    tipe::{A, Ty},
    val::{S, Val},
};

type BB = BasicBlock<'static>;

#[derive(PartialEq, Debug)]
pub struct FnCtx {
    llvm: Rc<LLVM>,
    func: UntypedFunc,
    bb: Cell<BB>,
    ins_flags: Cell<InstructionFlags>,
}

impl FnCtx {
    pub(crate) fn raw_func(&self) -> UntypedFunc {
        self.func
    }
    pub(crate) fn args(&self) -> impl Iterator<Item = UntypedValue> {
        self.func.0.get_param_iter().map(UntypedValue)
    }
    pub(crate) fn set_param_alignment(&self, param_index: u32, alignment: usize) {
        self.func.0.set_param_alignment(
            param_index,
            alignment.try_into().expect("usize -> u32 overflow"),
        );
    }
    pub(crate) fn set_param_metadata(&self, param_index: u32, metadata: &str, val: u64) {
        let kind_id = Attribute::get_named_enum_kind_id(metadata);
        let attribute = self.llvm.ctx().create_enum_attribute(kind_id, val);
        self.func
            .0
            .add_attribute(AttributeLoc::Param(param_index), attribute);
    }
    pub(crate) fn set_function_metadata(&self, metadata: &str, val: u64) {
        let kind_id = Attribute::get_named_enum_kind_id(metadata);
        let attribute = self.llvm.ctx().create_enum_attribute(kind_id, val);
        self.func.0.add_attribute(AttributeLoc::Function, attribute);
    }
    pub(crate) fn get_current_bb(&self) -> BasicBlock<'static> {
        self.bb.get()
    }
    pub(crate) fn append_bb(&self, name: &str) -> BasicBlock<'static> {
        let next_bb = self.ctx().append_basic_block(self.func.0, name);
        self.bb.set(next_bb);
        next_bb
    }
    pub(crate) fn set_call_convention(&self, call_conventions: CallConv) {
        self.func.0.set_call_conventions(call_conventions.to_llvm());
    }
    pub fn void_ty(&self) -> VoidType {
        VoidType(self.ctx().void_type())
    }
    pub(crate) fn ctx(&self) -> ContextRef<'static> {
        self.llvm.ctx()
    }
    pub(crate) fn with_bb_as<U>(&self, bb: BB, f: impl FnOnce() -> U) -> U {
        let init = self.bb.get();
        self.bb.set(bb);
        let ret = f();
        self.bb.set(init);
        ret
    }

    pub fn initialize(cpu: &impl ToCPU) {
        let config = &InitializationConfig::default();
        match cpu.triple() {
            "nvptx64-nvidia-cuda" => Target::initialize_nvptx(config),
            _ => panic!("Unrecognized triple '{}'", cpu.triple()),
        }
    }

    pub fn get_single_llvm(self) -> Result<LLVM, Self> {
        Rc::try_unwrap(self.llvm).map_err(|llvm| Self {
            llvm,
            func: self.func,
            bb: self.bb,
            ins_flags: Cell::default(),
        })
    }

    fn validate_all_function_bbs_terminated(&self) {
        let inner = self.llvm.inner();
        for func in inner.get_functions() {
            for (i, bb) in func.get_basic_block_iter().enumerate() {
                if bb.get_terminator().is_none() {
                    panic!(
                        "Function {:?}'s {i}th basic block {}\n\ndoes not end in a terminator",
                        func.get_name(),
                        bb.get_instructions().fold(
                            format!("{:?}:", bb.get_name()),
                            |acc, ins| format!(
                                "{acc}\n{}",
                                &ins.to_string()[1..].strip_suffix("\"").unwrap()
                            )
                        )
                    );
                }
            }
        }
    }

    fn create_machine(&self, cpu: &impl ToCPU, opt: Opt) -> TargetMachine {
        FnCtx::initialize(cpu);
        let triple = TargetTriple::create(cpu.triple());
        let target = Target::from_triple(&triple).expect("cpu.triple() invalid for LLVM");
        target
            .create_target_machine(
                &triple,
                cpu.cpu(),
                cpu.features(),
                opt.as_optimization_level(),
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .expect("Could not create a compiler with the given option")
    }

    pub fn run_passes(&self, cpu: &impl ToCPU, opt: Opt) {
        let passes = opt.default_passes();
        let options = PassBuilderOptions::create();
        let machine = self.create_machine(cpu, opt);
        self.llvm
            .module()
            .run_passes(passes, &machine, options)
            .expect("Was unable to run passes");
    }

    pub fn compile(self, cpu: &impl ToCPU, opt: Opt) -> Box<[u8]> {
        Self::initialize(cpu);
        let inner = self.llvm.inner();

        self.validate_all_function_bbs_terminated();

        self.run_passes(cpu, opt);

        let machine = self.create_machine(cpu, opt);

        let maybe_ret = machine
            .write_to_memory_buffer(&inner, FileType::Assembly)
            .expect("Unable to compile");
        maybe_ret.as_slice().to_vec().into_boxed_slice()
    }

    pub fn module_string(&self) -> String {
        self.llvm.module().print_to_string().to_string()
    }

    pub unsafe fn curr_bb_builder(&self) -> Builder<'static> {
        let b = self.ctx().create_builder();
        b.position_at_end(self.bb.get());
        b
    }

    pub(crate) fn new(llvm: Rc<LLVM>, func: UntypedFunc) -> Self {
        let bb = llvm.ctx().append_basic_block(func.0, "entry");
        let bb = Cell::new(bb);
        Self {
            llvm,
            func,
            bb,
            ins_flags: Cell::default(),
        }
    }

    pub fn apply_ins_flags(&self, val: &mut UntypedValue) {
        self.ins_flags.get().apply_to_ins(val);
    }
    pub fn set_ins_flags(&self, f: impl FnOnce(&mut InstructionFlags)) {
        let mut flags = self.ins_flags.get();
        f(&mut flags);
        self.ins_flags.set(flags);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct FnRef(Rc<FnCtx>);

impl FnRef {
    pub fn new(ctx: FnCtx) -> Self {
        Self(Rc::new(ctx))
    }
    pub fn try_get(self) -> Result<FnCtx, FnRef> {
        Rc::try_unwrap(self.0).map_err(Self)
    }
    pub fn llvm(&self) -> &LLVM {
        &self.0.llvm
    }
    pub fn constant<C: Ty>(&self, c: C) -> Val<C>
    where
        C: Copy,
    {
        C::const_val(c, self.clone())
    }
    pub fn alloca<T: Ty>(&self) -> Val<S<T>> {
        let bb = self
            .func
            .0
            .get_first_basic_block()
            .expect("Must have an entrypoint to call alloca");
        let b = self.llvm.ctx().create_builder();
        if let Some(ins) = bb.get_first_instruction() {
            b.position_at(bb, &ins);
        } else {
            b.position_at_end(bb);
        }
        let alloca = b
            .build_alloca(T::raw_ty(self.llvm.ctx()), "alloca")
            .expect("alloca should not fail");
        unsafe { Val::new_untyped(self.clone(), UntypedValue(alloca.as_basic_value_enum())) }
    }
    pub fn get_intrinsic<Ret: FnRetTy, Args: FnArgs>(
        &self,
        name: &str,
        overload_with_ret: bool,
    ) -> Result<ExternFunc<Ret, Args>, IntrinsicError> {
        let intrins = Intrinsic::find(name);
        let Some(intrins) = intrins else {
            return Err(IntrinsicError::NameNotFound);
        };
        let args = Args::raw_type_sequence(self.llvm.ctx());
        let expected_type = Ret::erased_func_type(self.llvm.ctx(), &args).0;
        let mut overload_sig = if overload_with_ret {
            let ret_ty = match Ret::raw_inkwell_type(self.ctx()) {
                AnyTypeEnum::ArrayType(array_type) => array_type.as_basic_type_enum(),
                AnyTypeEnum::FloatType(float_type) => float_type.as_basic_type_enum(),
                AnyTypeEnum::IntType(int_type) => int_type.as_basic_type_enum(),
                AnyTypeEnum::PointerType(pointer_type) => pointer_type.as_basic_type_enum(),
                AnyTypeEnum::StructType(struct_type) => struct_type.as_basic_type_enum(),
                AnyTypeEnum::VectorType(vector_type) => vector_type.as_basic_type_enum(),
                AnyTypeEnum::ScalableVectorType(scalable_vector_type) => {
                    scalable_vector_type.as_basic_type_enum()
                }
                AnyTypeEnum::FunctionType(_) => {
                    unreachable!("Should never have function type returned")
                }
                AnyTypeEnum::VoidType(_) => {
                    unreachable!("Should never be overloaded on void type")
                }
            };
            vec![ret_ty]
        } else {
            vec![]
        };
        overload_sig.extend(args.into_iter().map(|e| e.0));
        let _args = if intrins.is_overloaded() {
            println!("Intrinsic {name} was overloaded");
            overload_sig.as_slice()
        } else {
            println!("Intrinsic {name} was not overloaded");
            &[]
        };
        let Some(intrins) = intrins.get_declaration(self.llvm.module(), _args) else {
            return Err(IntrinsicError::DeclarationNotFound);
        };
        println!("Found {intrins} for {name}");
        let intrinsic_type = intrins.get_type();
        println!("Found intrinsic type {intrinsic_type} for {name}");
        if intrinsic_type != expected_type {
            return Err(IntrinsicError::MismatchedType(
                intrinsic_type,
                expected_type,
            ));
        }
        Ok(unsafe { ExternFunc::new(UntypedFunc(intrins)) })
    }
    pub fn declare_extern<Ret: FnRetTy, Args: FnArgs>(&self, name: &str) -> ExternFunc<Ret, Args> {
        let args = Args::raw_type_sequence(self.llvm.ctx());
        let fn_type = Ret::erased_func_type(self.llvm.ctx(), &args).0;
        let new_func = self.0.llvm.module().add_function(name, fn_type, None);
        unsafe { ExternFunc::new(UntypedFunc(new_func)) }
    }
    pub fn call_extern<Ret, Args>(
        &self,
        extern_func: ExternFunc<Ret, Args>,
        args: Args::ArgValues,
        callconv: Option<CallConv>,
    ) -> Ret::RetVal
    where
        Args: FnArgs,
        Ret: FnRetTy,
    {
        let args: Vec<_> = Args::arg_arr(args)
            .into_iter()
            .map(|v| v.0.into())
            .collect();
        let b = unsafe { self.curr_bb_builder() };
        let csv = b
            .build_call(
                extern_func.raw_func().0,
                &args,
                extern_func.raw_func().name(),
            )
            .expect("Build call should have worked");
        if let Some(callconv) = callconv {
            csv.set_call_convention(callconv.to_llvm());
        }
        unsafe { Ret::extract_call_site_value(self.clone(), csv) }
    }

    pub fn insert_const_str(&self, s: &str, name: &str) -> Val<*const u8> {
        let global = self.llvm.insert_str(s, name, None);
        let b = unsafe { self.curr_bb_builder() };
        let zero = self.ctx().i32_type().const_zero();
        let ptr_out = unsafe {
            b.build_gep(
                global.get_value_type().into_array_type(),
                global.as_pointer_value(),
                &[zero],
                "str_gep",
            )
            .expect("GEP should have succeeded")
        };
        unsafe { Val::new(self.clone(), UntypedValue(ptr_out.as_basic_value_enum())) }
    }
    pub fn insert_const_str_in_space<const ADDRSPACE: u16>(
        &self,
        s: &str,
        name: &str,
    ) -> Val<A<*const u8, ADDRSPACE>> {
        let global = self
            .llvm
            .insert_str(s, name, Some(AddressSpace::from(ADDRSPACE)));
        let b = unsafe { self.curr_bb_builder() };
        let zero = self.ctx().i32_type().const_zero();
        let ptr_out = unsafe {
            b.build_gep(
                global.get_value_type().into_array_type(),
                global.as_pointer_value(),
                &[zero],
                "str_gep",
            )
            .expect("GEP should have succeeded")
        };
        unsafe { Val::new(self.clone(), UntypedValue(ptr_out.as_basic_value_enum())) }
    }
    pub fn insert_const_value_in_space<const ADDRSPACE: u16, T: Ty>(
        &self,
        v: Val<T>,
        name: &str,
    ) -> Val<A<&'static T, ADDRSPACE>> {
        let global = self
            .llvm
            .insert_const(v, name, Some(AddressSpace::from(ADDRSPACE)));
        let b = unsafe { self.curr_bb_builder() };
        let zero = self.ctx().i32_type().const_zero();
        let ptr_out = unsafe {
            b.build_gep(T::ty(self).0, global.as_pointer_value(), &[zero], "str_gep")
                .expect("GEP should have succeeded")
        };
        unsafe { Val::new(self.clone(), UntypedValue(ptr_out.as_basic_value_enum())) }
    }
    pub fn insert_const_value<T: Ty>(&self, v: Val<T>, name: &str) -> Val<&'static T> {
        let global = self.llvm.insert_const(v, name, None);
        let b = unsafe { self.curr_bb_builder() };
        let zero = self.ctx().i32_type().const_zero();
        let ptr_out = unsafe {
            b.build_gep(T::ty(self).0, global.as_pointer_value(), &[zero], "str_gep")
                .expect("GEP should have succeeded")
        };
        unsafe { Val::new(self.clone(), UntypedValue(ptr_out.as_basic_value_enum())) }
    }
}

impl AsRef<FnCtx> for FnRef {
    fn as_ref(&self) -> &FnCtx {
        &self.0
    }
}

impl Deref for FnRef {
    type Target = FnCtx;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct VoidType(pub(crate) inkwell::types::VoidType<'static>);

impl VoidType {
    pub(crate) fn func_type(&self, args: &[ErasedType]) -> ErasedFuncType {
        // Safety: repr-transparent
        let param_types = unsafe { std::mem::transmute(args) };
        ErasedFuncType(self.0.fn_type(param_types, false))
    }
}
