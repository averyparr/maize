use std::{borrow::Borrow, cell::Cell, marker::PhantomData};

use inkwell::{
    AddressSpace, OptimizationLevel,
    attributes::{Attribute, AttributeLoc},
    basic_block::BasicBlock,
    builder::Builder,
    context::ContextRef,
    intrinsics::Intrinsic,
    module::{Linkage, Module},
    passes::PassBuilderOptions,
    support::LLVMString,
    targets::{FileType, InitializationConfig, Target, TargetTriple},
    types::{BasicType, StructType},
    values::{
        AggregateValue, AnyValue, BasicValue, BasicValueEnum, FunctionValue, InstructionValue,
        PointerValue, StructValue, VectorBaseValue,
    },
};

use crate::{
    intrinsics::{IntrinsicCodegen, IntrinsicsLibrary, StatelessIntrinsicsLibrary},
    ty::{
        Addrspace, BF16, Bool, F16, F32, F64, FnRetTy, HowToExtractElements, I8, I16, I32, I64,
        IntoFuncArgs, P, U8, U16, U32, U64, UniformTy, V, ValTy, Void, VoidTy, vec::VectorizableTy,
    },
    val::Val,
};

use super::instruction_opt::InstructionOpt;

pub struct FnCodegen {
    module: Module<'static>,
    func: FunctionValue<'static>,
    bb: Cell<BasicBlock<'static>>,
    opt: Cell<InstructionOpt>,
    intrinsics: Box<dyn IntrinsicsLibrary>,
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
                    unsafe {Val::new(cx, raw.as_basic_value_enum())}
                }
            }

            impl IntoConstVal for $raw_ty {
                type Assoc = $trace_ty;
                fn into_const_val(self, cx: &FnCodegen) -> Val<'_, Self::Assoc> {
                    let raw = cx.ctx().$ty_fn().$val_fn(self as _, $($($args,)*)?);
                    unsafe {Val::new(cx, raw.as_basic_value_enum())}
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

impl ConstValTy for BF16 {
    type Assoc = f32;
    fn to_const(assoc: impl Into<Self::Assoc>, cx: &FnCodegen) -> Val<'_, Self> {
        let raw = cx.ctx().bf16_type().const_float(assoc.into() as _);
        unsafe { Val::new(cx, raw.as_basic_value_enum()) }
    }
}

impl ConstValTy for F16 {
    type Assoc = f32;
    fn to_const(assoc: impl Into<Self::Assoc>, cx: &FnCodegen) -> Val<'_, Self> {
        let raw = cx.ctx().f16_type().const_float(assoc.into() as _);
        unsafe { Val::new(cx, raw.as_basic_value_enum()) }
    }
}

impl<C> Val<'_, C>
where
    C: ConstValTy,
{
    pub fn const_like(&self, val: C::Assoc) -> Self {
        C::to_const(val, self.cx())
    }
}

impl<C, const N: usize> Val<'_, V<C, N>>
where
    C: ConstValTy + VectorizableTy + Copy,
{
    pub fn const_like(&self, val: impl Into<C::Assoc>) -> Self {
        Val::splat(C::to_const(val, self.cx()))
    }
}

pub trait ConstValTy: ValTy {
    type Assoc: Copy;
    fn to_const(assoc: impl Into<Self::Assoc>, cx: &FnCodegen) -> Val<'_, Self>;
}

pub trait IntoConstVal: Copy {
    type Assoc: ValTy;
    fn into_const_val(self, cx: &FnCodegen) -> Val<'_, Self::Assoc>;
}

pub struct RawFunc<Ret, Args>(FunctionValue<'static>, PhantomData<(Ret, Args)>);
impl<Ret: FnRetTy, Args: IntoFuncArgs> RawFunc<Ret, Args> {
    pub fn new(fn_val: FunctionValue<'static>) -> Self {
        let fn_ty = fn_val.get_type();
        let ctx = fn_ty.get_context();
        let type_system_fn_ty = Ret::fn_ty::<Args>(ctx);
        assert_eq!(fn_ty, type_system_fn_ty);
        Self(fn_val, PhantomData)
    }
    pub fn raw(&self) -> FunctionValue<'static> {
        self.0
    }
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
    pub(crate) fn with_bb_as<U>(
        &self,
        bb: BasicBlock<'static>,
        f: impl FnOnce() -> U,
    ) -> (BasicBlock<'static>, U) {
        let curr_bb = self.bb();
        self.set_bb(bb);
        let ret = f();
        let end_bb = self.bb();
        self.set_bb(curr_bb);
        (end_bb, ret)
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
    pub fn intrinsics(&self) -> &dyn IntrinsicsLibrary {
        self.intrinsics.as_ref()
    }
    pub fn get_intrinsic<Ret: FnRetTy, Args: IntoFuncArgs>(
        &self,
        name: &str,
        use_raw_name: bool,
    ) -> RawFunc<Ret, Args> {
        let Some(intrinsic) = Intrinsic::find(name) else {
            panic!("Unable to find intrinsic '{name}'");
        };

        let param_types = Args::produce_args(self.ctx());
        let ret = intrinsic
            .get_declaration(self.module(), param_types.as_ref())
            .expect("Should be a function with this declaration");
        // Do this so that we get intrinsic type checking
        let should_ret = RawFunc::new(ret);
        if use_raw_name {
            self.declare_function(name)
        } else {
            should_ret
        }
    }
    pub fn store_in_alloca<'a>(&'a self, val: BasicValueEnum<'static>) -> Val<'a, P<*mut Void>> {
        let first_bb = self
            .func()
            .get_first_basic_block()
            .expect("There should be a first BB");
        let builder = self.ctx().create_builder();
        builder.position_at(
            first_bb,
            &first_bb
                .get_first_instruction()
                .expect("Should be some instruction"),
        );

        let alloca_ptr = builder
            .build_alloca(val.get_type(), "alloca")
            .expect("Alloca should succeed");
        let _store = unsafe { self.with_builder(|b| b.build_store(alloca_ptr, val)) }
            .expect("Store should have succeeded");
        // Safety: We have just moved the value into the alloca and so this
        // type cast is valid
        unsafe { Val::new(self, alloca_ptr.as_basic_value_enum()) }
    }
    pub fn store_vals_in_struct_alloca<'a>(
        &'a self,
        all_values: &[BasicValueEnum<'static>],
        packed: bool,
    ) -> Val<'a, P<*mut Void>> {
        let field_types: Vec<_> = all_values.iter().map(|v| v.get_type()).collect();
        let pointee_ty = self.ctx().struct_type(&field_types, packed);
        let alloca_ptr = unsafe { self.with_builder(|b| b.build_alloca(pointee_ty, "alloca")) }
            .expect("Alloca should succeed");
        for (field_idx, value) in all_values.iter().enumerate() {
            let index = u32::try_from(field_idx).expect("usize -> u32 overflow");
            // SAFETY: We are GEPing into something we just allocated and storing a value we
            // already have into that alloca, and the only way someone can make use
            // of the alloca directly is by dereferencing the void* ptr.
            unsafe {
                self.with_builder(|b| {
                    let ptr = b
                        .build_struct_gep(pointee_ty, alloca_ptr, index, "valist_gep")
                        .expect("VA list GEP should succeed");
                    b.build_store(ptr, *value)
                        .expect("Store to VA list should succeed");
                });
            }
        }
        unsafe { Val::new(self, alloca_ptr.as_basic_value_enum()) }
    }
    pub fn declare_function<Ret: FnRetTy, Args: IntoFuncArgs>(
        &self,
        name: &str,
    ) -> RawFunc<Ret, Args> {
        let fn_ty = Ret::fn_ty::<Args>(self.ctx());
        if let Some(fn_val) = self.module().get_function(name) {
            RawFunc::new(fn_val)
        } else {
            RawFunc::new(self.module().add_function(name, fn_ty, None))
        }
    }

    pub fn call_void_fn<Args: IntoFuncArgs>(
        &self,
        fn_val: RawFunc<Void, Args>,
        args: Args::ArgValues<'_>,
    ) {
        let raw_args = Args::args_to_raw(args);
        let raw_ret =
            unsafe { self.with_builder(|b| b.build_call(fn_val.0, raw_args.as_ref(), "call_fn")) }
                .expect("Call should have succeeded");
        assert!(raw_ret.try_as_basic_value().is_instruction());
    }

    pub fn call_fn<'a, Ret: ValTy, Args: IntoFuncArgs>(
        &'a self,
        fn_val: RawFunc<Ret, Args>,
        args: Args::ArgValues<'a>,
    ) -> Val<'a, Ret> {
        let raw_args = Args::args_to_raw(args);
        let raw_ret =
            unsafe { self.with_builder(|b| b.build_call(fn_val.0, raw_args.as_ref(), "call_fn")) }
                .expect("Call should have succeeded");
        let basic_val = raw_ret.try_as_basic_value().unwrap_basic();
        unsafe { Val::new(self, basic_val) }
    }

    pub fn extract_elem<U, T>(&self, val: &Val<'_, T>, index: u32) -> U::Value<'static>
    where
        T: UniformTy,
        U: ValTy,
    {
        fn extract_as_vec(
            cx: &FnCodegen,
            vector: impl VectorBaseValue<'static>,
            index: u32,
        ) -> BasicValueEnum<'static> {
            let index = cx.constant_from(index as u64).ll_typed();
            unsafe { cx.with_builder(|b| b.build_extract_element(vector, index, "vextract")) }
                .expect("[scalable] vector extract should have succeeded")
        }
        fn extract_as_agg(
            cx: &FnCodegen,
            agg: impl AggregateValue<'static>,
            index: u32,
        ) -> BasicValueEnum<'static> {
            unsafe { cx.with_builder(|b| b.build_extract_value(agg, index, "agg_extract")) }
                .expect("Aggregate extract should have succeeded")
        }

        let proposed_elem_ty = U::ty(self.ctx()).as_basic_type_enum();
        let raw_val = val.raw();
        let raw_val = match T::EXTRACTION_METHOD {
            HowToExtractElements::Vector => {
                let vector = raw_val.into_vector_value();
                let elem_ty = vector.get_type().get_element_type();
                assert_eq!(elem_ty, proposed_elem_ty);
                extract_as_vec(self, vector, index)
            }
            HowToExtractElements::ScalableVector => {
                let vector = raw_val.into_scalable_vector_value();
                let elem_ty = vector.get_type().get_element_type();
                assert_eq!(elem_ty, proposed_elem_ty);
                extract_as_vec(self, vector, index)
            }
            HowToExtractElements::Array => {
                let agg = raw_val.into_array_value();
                let elem_ty = agg.get_type().get_element_type();
                assert_eq!(elem_ty, proposed_elem_ty);
                extract_as_agg(self, agg, index)
            }
            HowToExtractElements::Struct => {
                let agg = raw_val.into_struct_value();
                let elem_ty = agg
                    .get_type()
                    .get_field_type_at_index(index)
                    .expect("Field out of range!");
                assert_eq!(elem_ty, proposed_elem_ty);
                extract_as_agg(self, agg, index)
            }
        };

        U::type_val(raw_val.as_any_value_enum())
    }
    pub fn insert_elem<'a, U, T>(&self, agg: Val<'a, T>, val: Val<'a, U>, index: u32) -> Val<'a, T>
    where
        T: UniformTy,
        U: ValTy,
    {
        fn insert_as_vec(
            cx: &FnCodegen,
            vector: impl VectorBaseValue<'static>,
            element: BasicValueEnum<'static>,
            index: u32,
        ) -> BasicValueEnum<'static> {
            let index = cx.constant_from(index as u64).ll_typed();
            unsafe { cx.with_builder(|b| b.build_insert_element(vector, element, index, "vins")) }
                .expect("[scalable] Vector insert element should have succeded")
                .as_basic_value_enum()
        }
        fn insert_as_agg(
            cx: &FnCodegen,
            agg: impl AggregateValue<'static>,
            value: BasicValueEnum<'static>,
            index: u32,
        ) -> BasicValueEnum<'static> {
            unsafe { cx.with_builder(|b| b.build_insert_value(agg, value, index, "agg_ins")) }
                .expect("Aggregate insert element should have succeeded")
                .as_basic_value_enum()
        }

        let element = val.raw();
        let raw_val = agg.raw();

        let raw_val = match T::EXTRACTION_METHOD {
            HowToExtractElements::Vector => {
                let vector = raw_val.into_vector_value();
                assert_eq!(vector.get_type().get_element_type(), element.get_type());
                insert_as_vec(self, vector, element, index)
            }
            HowToExtractElements::ScalableVector => {
                let vector = raw_val.into_scalable_vector_value();
                assert_eq!(vector.get_type().get_element_type(), element.get_type());
                insert_as_vec(self, vector, element, index)
            }
            HowToExtractElements::Array => {
                let agg = raw_val.into_array_value();
                assert_eq!(agg.get_type().get_element_type(), element.get_type());
                insert_as_agg(self, agg, element, index)
            }
            HowToExtractElements::Struct => {
                let agg = raw_val.into_struct_value();
                let elem_ty = agg
                    .get_type()
                    .get_field_type_at_index(index)
                    .expect("Field index error!");
                assert_eq!(elem_ty, element.get_type());
                insert_as_agg(self, agg, element, index)
            }
        };

        unsafe { Val::new(agg.cx(), raw_val) }
    }
    pub fn get_elem_ptr<U, T, Space: Addrspace>(
        &self,
        ptr: &Val<'_, P<*const T, Space>>,
        index: u32,
    ) -> PointerValue<'static>
    where
        T: UniformTy + ?Sized,
        U: ValTy,
    {
        fn extract_as_vec_array(
            cx: &FnCodegen,
            pointee_ty: impl BasicType<'static>,
            ptr: PointerValue<'static>,
            index: u32,
        ) -> PointerValue<'static> {
            let zero = cx.constant_from(0u64).ll_typed();
            let index = cx.constant_from(index as u64).ll_typed();
            unsafe {
                cx.with_builder(|b| {
                    b.build_in_bounds_gep(pointee_ty, ptr, &[zero, index], "svarr_gep")
                })
            }
            .expect("GEP for struct/vec/scalable vec should have succeeded")
        }

        let proposed_elem_ty = U::ty(self.ctx()).as_basic_type_enum();
        let raw_ptr = ptr.ll_typed();
        let pointee_ty = T::ty(self.ctx()).as_basic_type_enum();
        match T::EXTRACTION_METHOD {
            HowToExtractElements::Vector => {
                let vec_ty = pointee_ty.into_vector_type();
                assert_eq!(vec_ty.get_element_type(), proposed_elem_ty);
                extract_as_vec_array(self, vec_ty, raw_ptr, index)
            }
            HowToExtractElements::ScalableVector => {
                let scalable_vec_ty = pointee_ty.into_scalable_vector_type();
                assert_eq!(scalable_vec_ty.get_element_type(), proposed_elem_ty);
                extract_as_vec_array(self, scalable_vec_ty, raw_ptr, index)
            }
            HowToExtractElements::Array => {
                let array_ty = pointee_ty.into_array_type();
                assert_eq!(array_ty.get_element_type(), proposed_elem_ty);
                extract_as_vec_array(self, array_ty, raw_ptr, index)
            }
            HowToExtractElements::Struct => {
                let struct_ty = pointee_ty.into_struct_type();
                let field_ty = struct_ty
                    .get_field_type_at_index(index)
                    .expect("Field idx should be in range");
                assert_eq!(field_ty, proposed_elem_ty);

                let offset_ptr = unsafe {
                    self.with_builder(|b| {
                        b.build_struct_gep(struct_ty, raw_ptr, index, "struct_gep")
                    })
                }
                .expect("Struct GEP should succeed");
                offset_ptr
            }
        }
    }

    pub fn get_struct_field<U, T>(&self, val: &Val<'_, T>, index: u32) -> U::Value<'static>
    where
        for<'ctx> T: ValTy<Type<'ctx> = StructType<'ctx>, Value<'ctx> = StructValue<'ctx>>,
        U: ValTy,
    {
        let struct_ty = T::ty(self.ctx());
        let field_ty = struct_ty
            .get_field_type_at_index(index)
            .expect("Field index should be in-range");
        assert_eq!(field_ty, U::ty(self.ctx()).as_basic_type_enum());

        let raw_val = unsafe {
            self.with_builder(|b| b.build_extract_value(val.ll_typed(), index, "get_struct_field"))
        }
        .expect("Struct get field should have worked");

        U::type_val(raw_val.as_any_value_enum())
    }
    pub fn insert_struct_field<'a, U, T>(
        &self,
        agg: Val<'a, T>,
        val: Val<'a, U>,
        index: u32,
    ) -> Val<'a, T>
    where
        for<'ctx> T: ValTy<Type<'ctx> = StructType<'ctx>, Value<'ctx> = StructValue<'ctx>>,
        U: ValTy,
    {
        let struct_ty = T::ty(self.ctx());
        let field_ty = struct_ty
            .get_field_type_at_index(index)
            .expect("Field index should be in-range");
        assert_eq!(field_ty, U::ty(self.ctx()).as_basic_type_enum());

        let raw_val = unsafe {
            self.with_builder(|b| {
                b.build_insert_value(agg.ll_typed(), val.ll_typed(), index, "insert_struct")
            })
        }
        .expect("Insert to struct should have worked");

        unsafe { Val::new(agg.cx(), raw_val.as_basic_value_enum()) }
    }
    pub fn get_struct_ptr<U, T, Space: Addrspace>(
        &self,
        ptr: Val<'_, P<*const T, Space>>,
        index: u32,
    ) -> PointerValue<'static>
    where
        for<'ctx> T: ValTy<Type<'ctx> = StructType<'ctx>, Value<'ctx> = StructValue<'ctx>>,
        U: ValTy,
    {
        let pointee_ty = T::ty(self.ctx());
        let field_ty = pointee_ty
            .get_field_type_at_index(index)
            .expect("Field should be in-range");
        assert_eq!(field_ty, U::ty(self.ctx()).as_basic_type_enum());

        let raw_ptr = ptr.ll_typed();
        let offset_ptr = unsafe {
            self.with_builder(|b| b.build_struct_gep(pointee_ty, raw_ptr, index, "struct_offset"))
        }
        .expect("Struct GEP should succeed");

        offset_ptr
    }

    pub fn insert_str(
        &self,
        s: &str,
        address_space: Option<AddressSpace>,
        name: &str,
    ) -> Val<'_, P<*const U8>> {
        let ctx = self.ctx();
        let i8_ty = ctx.i8_type();
        let ty = i8_ty.array_type(s.len().try_into().expect("usize -> u32 overflow"));
        let mapped_chars = s
            .bytes()
            .map(|b| ctx.i8_type().const_int(b.into(), false))
            .collect::<Vec<_>>();
        let global = self.module().add_global(ty, address_space, name);
        global.set_initializer(&i8_ty.const_array(mapped_chars.as_slice()));
        global.set_linkage(Linkage::Internal);
        global.set_unnamed_addr(true);
        let zero = ctx.i32_type().const_zero();
        let ptr_to_char = unsafe {
            self.with_builder(|b| {
                let gep_ptr = b
                    .build_in_bounds_gep(ty, global.as_pointer_value(), &[zero, zero], "str_gep")
                    .expect("GEP for const char array should succeed");
                gep_ptr
            })
        };
        unsafe { Val::new(self, ptr_to_char.as_basic_value_enum()) }
    }

    pub fn print_module_to_string(&self) -> LLVMString {
        self.module().print_to_string()
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
    type Intrinsics: StatelessIntrinsicsLibrary + 'static;
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

    fn intrinsics(&self) -> IntrinsicCodegen<'_, Self::Intrinsics> {
        IntrinsicCodegen::new(self.cx())
    }

    fn from_ctx(ctx: ContextRef<'static>) -> Self
    where
        Self: Sized,
    {
        let module = ctx.create_module("func");
        let fn_ty = Self::Ret::fn_ty::<Self::Args>(ctx);
        let func = module.add_function("func", fn_ty, None);
        func.set_call_conventions(Self::CALL_CONV);
        let mustprogress_id = Attribute::get_named_enum_kind_id("mustprogress");
        let mustprogress = ctx.create_enum_attribute(mustprogress_id, 0);
        func.add_attribute(AttributeLoc::Function, mustprogress);
        let bb = ctx.append_basic_block(func, "entry");
        let bb = Cell::new(bb);
        let opt = Cell::default();
        let cg = FnCodegen {
            module,
            func,
            bb,
            opt,
            intrinsics: Box::new(Self::Intrinsics::new()),
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
        // self.cx().func().add_attribute(
        //     inkwell::attributes::AttributeLoc::Function,
        //     self.cx()
        //         .ctx()
        //         .create_string_attribute("denormal-fp-math-f32", "preserve-sign,preserve-sign"),
        // );
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
    pub fn compile_with_hooks(
        self,
        cpu: &F::CpuConfig,
        optimization_level: OptimizationLevel,
        file_type: FileType,
        pre_passes: impl Fn(&FnCodegen),
        post_passes: impl Fn(&FnCodegen),
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

        pre_passes(self.0.cx());

        let options = PassBuilderOptions::create();
        // options.set_debug_logging(true);

        self.0
            .cx()
            .module
            .run_passes(passes, &machine, options)
            .expect("Unable to run passes on module");

        post_passes(self.0.cx());

        let maybe_ret = machine
            .write_to_memory_buffer(&self.0.cx().module, file_type)
            .expect("Unable to compile");
        maybe_ret.as_slice().to_vec().into_boxed_slice()
    }
    pub fn compile(
        self,
        cpu: &F::CpuConfig,
        optimization_level: OptimizationLevel,
        file_type: FileType,
    ) -> Box<[u8]> {
        self.compile_with_hooks(cpu, optimization_level, file_type, |_| (), |_| ())
    }
    pub fn compile_asm_at_opt_with_hooks(
        self,
        cpu: impl Borrow<F::CpuConfig>,
        optimization_level: OptimizationLevel,
        pre_passes: impl Fn(&FnCodegen),
        post_passes: impl Fn(&FnCodegen),
    ) -> String {
        String::from_utf8(Vec::from(self.compile_with_hooks(
            cpu.borrow(),
            optimization_level,
            FileType::Assembly,
            pre_passes,
            post_passes,
        )))
        .expect("asm should always be utf-8")
    }
    pub fn compile_asm_at_opt(
        self,
        cpu: impl Borrow<F::CpuConfig>,
        optimization_level: OptimizationLevel,
    ) -> String {
        self.compile_asm_at_opt_with_hooks(cpu, optimization_level, |_| (), |_| ())
    }
    pub fn compile_obj_at_opt_with_hooks(
        self,
        cpu: impl Borrow<F::CpuConfig>,
        optimization_level: OptimizationLevel,
        pre_passes: impl Fn(&FnCodegen),
        post_passes: impl Fn(&FnCodegen),
    ) -> Box<[u8]> {
        self.compile_with_hooks(
            cpu.borrow(),
            optimization_level,
            FileType::Object,
            pre_passes,
            post_passes,
        )
    }
    pub fn compile_obj_at_opt(
        self,
        cpu: impl Borrow<F::CpuConfig>,
        optimization_level: OptimizationLevel,
    ) -> Box<[u8]> {
        self.compile_obj_at_opt_with_hooks(cpu, optimization_level, |_| (), |_| ())
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
