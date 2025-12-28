use std::marker::PhantomData;

use inkwell::{
    AddressSpace,
    builder::Builder,
    context::ContextRef,
    types::BasicType,
    values::{BasicValueEnum, FloatValue, IntValue, PointerValue},
};

use crate::ty::{AddableTy, BasicTy, Ty};

#[derive(Clone, Copy)]
pub struct Void<'ctx>(ContextRef<'ctx>);
#[derive(Clone, Copy)]
pub struct I32<'ctx>(ContextRef<'ctx>);
#[derive(Clone, Copy)]
pub struct F32<'ctx>(ContextRef<'ctx>);

#[derive(Clone, Copy)]
pub struct PtrT<'ctx, Inner>(ContextRef<'ctx>, PhantomData<Inner>, AddressSpace);

impl<'ctx, Inner> PtrT<'ctx, Inner> {
    pub fn in_addrspace(ctx: ContextRef<'ctx>, addrspace: u16) -> Self {
        Self(ctx, PhantomData, addrspace.into())
    }
}

impl<'ctx> Ty<'ctx> for Void<'ctx> {
    fn new(ctx: ContextRef<'ctx>) -> Self {
        Void(ctx)
    }
}

impl<'ctx> Ty<'ctx> for I32<'ctx> {
    fn new(ctx: ContextRef<'ctx>) -> Self {
        I32(ctx)
    }
}

impl<'ctx> Ty<'ctx> for F32<'ctx> {
    fn new(ctx: ContextRef<'ctx>) -> Self {
        F32(ctx)
    }
}

impl<'ctx, Inner> Ty<'ctx> for PtrT<'ctx, Inner> {
    fn new(ctx: ContextRef<'ctx>) -> Self {
        Self(ctx, PhantomData, AddressSpace::default())
    }
}

impl<'ctx> BasicTy<'ctx> for I32<'ctx> {
    fn ctx(&self) -> ContextRef<'ctx> {
        self.0
    }
    type Value = IntValue<'ctx>;
    fn basic_ty(&self) -> impl BasicType<'ctx>
    where
        Self: 'ctx,
    {
        self.0.i32_type()
    }
    fn get_value(basic_val: BasicValueEnum<'ctx>) -> Self::Value {
        basic_val.into_int_value()
    }
}

impl<'ctx> BasicTy<'ctx> for F32<'ctx> {
    fn ctx(&self) -> ContextRef<'ctx> {
        self.0
    }
    type Value = FloatValue<'ctx>;
    fn basic_ty(&self) -> impl BasicType<'ctx>
    where
        Self: 'ctx,
    {
        self.0.f32_type()
    }
    fn get_value(basic_val: BasicValueEnum<'ctx>) -> Self::Value {
        basic_val.into_float_value()
    }
}

impl<'ctx, Inner> BasicTy<'ctx> for PtrT<'ctx, Inner> {
    fn ctx(&self) -> ContextRef<'ctx> {
        self.0
    }
    type Value = PointerValue<'ctx>;
    fn basic_ty(&self) -> impl BasicType<'ctx>
    where
        Self: 'ctx,
    {
        self.ctx().ptr_type(self.2)
    }
    fn get_value(basic_val: BasicValueEnum<'ctx>) -> Self::Value {
        basic_val.into_pointer_value()
    }
}

impl<'ctx> AddableTy<'ctx> for I32<'ctx> {
    fn emit_add(builder: Builder<'ctx>, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        builder
            .build_int_add(lhs, rhs, "add_int")
            .expect("Could not emit int add")
    }
}

impl<'ctx> AddableTy<'ctx> for F32<'ctx> {
    fn emit_add(builder: Builder<'ctx>, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        builder
            .build_float_add(lhs, rhs, "add_int")
            .expect("Could not emit int add")
    }
}

#[test]
fn test_works() {
    use super::create_context;
    use crate::{
        codegen::jit_func::{FileType, TargetMachine},
        cuda::{PTXOptions, SM},
        func::Func,
    };

    let ctx = create_context();

    let f32_t = F32::new(ctx);
    let f32_ptr_t = f32_t.ptr_ty_in(1);
    let f32_mut_ptr_t = f32_t.ptr_mut_ty_in(1);

    let f = Func::new_void(ctx, "add_fn", (f32_ptr_t, f32_ptr_t, f32_mut_ptr_t));

    let (a_ptr, b_ptr, c_ptr) = f.extract_args();

    let a = a_ptr.load();
    let b = b_ptr.load();

    c_ptr.store(a + b + c_ptr.load());

    let t = f.finalize();
    let c = t.compile(
        TargetMachine::PTX(PTXOptions { sm: SM::SM80 }),
        inkwell::OptimizationLevel::Aggressive,
        FileType::Assembly,
    );

    let t = String::from_utf8_lossy(&c);
    println!("{}", t);
    assert!(false);
}
