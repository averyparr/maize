use inkwell::AddressSpace;
use inkwell::attributes::Attribute;
use inkwell::attributes::AttributeLoc;
use inkwell::types::AnyType;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};
use inkwell::values::InstructionValue;
use inkwell::{context::ContextRef, module::Module, types::BasicTypeEnum};

use crate::codegen::CodegenModule;
use crate::codegen::FnCodegen;
use crate::codegen::context::create_context;
use crate::codegen::pre_jit_func::PreJitFunc;
use crate::ty::{FnReturnTy, Ty, Void};
use crate::val::{AcceptsConstants, Holds, Val};

const UPPER_BOUND_ON_PARAMS: usize = 16;

pub struct Func<ArgsT, Ret = Void> {
    ret_ty: Ret,
    arg_types: ArgsT,
    codegen_module: CodegenModule<'static>,
}

impl<ArgsT, Ret> Func<ArgsT, Ret> {
    pub fn extract_module_codegen(self) -> (Module<'static>, FnCodegen<'static>) {
        self.codegen_module.extract_module_codegen()
    }
    pub(crate) fn mod_ref(&self) -> &Module<'static> {
        self.codegen_module.module()
    }
    pub(crate) fn cx_ref(&self) -> &FnCodegen<'static> {
        self.codegen_module.cx()
    }
    pub(crate) fn cm_ref(&self) -> &CodegenModule<'static> {
        &self.codegen_module
    }
    pub fn const_val<'lt, T, FromT>(&'lt self, c: FromT) -> Val<'lt, T>
    where
        T: Ty + AcceptsConstants,
        FromT: Into<T::Assoc>,
    {
        T::new_const(c.into(), &self.codegen_module)
    }
}

impl<ArgsT, Ret> Func<ArgsT, Ret>
where
    ArgsT: IntoFuncArgs,
{
    pub fn get_args(&self) -> ArgsT::ArgValues<'_> {
        ArgsT::get_args(&self.codegen_module)
    }
}

impl<ArgsT, Ret> Func<ArgsT, Ret>
where
    Ret: Ty,
{
    pub fn finalize<'lt>(self, retval: Val<'lt, Ret>) -> PreJitFunc<ArgsT, Ret> {
        // SAFETY: We are returning something of the correct type so return should be unconditionally valid
        let _: InstructionValue<'_> = unsafe {
            self.cx_ref()
                .with_builder(|b| b.build_return(Some(&retval.to_underlying())))
        }
        .expect("Should be possible to return value!");
        PreJitFunc::new(self)
    }
}

impl<ArgsT> Func<ArgsT, Void> {
    pub fn finalize(self) -> PreJitFunc<ArgsT, Void> {
        // SAFETY: We are not returning anything from the function so this should be safe
        let _: InstructionValue<'_> =
            unsafe { self.cx_ref().with_builder(|b| b.build_return(None)) }
                .expect("Should be able to return void");
        PreJitFunc::new(self)
    }
}

pub fn create_func<ArgsT, Ret>() -> Func<ArgsT, Ret>
where
    ArgsT: IntoFuncArgs,
    Ret: FnReturnTy,
{
    let ctx = create_context();
    let module = ctx.create_module("fn");

    let void_star = ctx.ptr_type(AddressSpace::default());
    let mut arr = [BasicMetadataTypeEnum::PointerType(void_star); UPPER_BOUND_ON_PARAMS];

    let arg_types = ArgsT::new_from_ctx(ctx);
    let ty_iter = arg_types.basic_ty_iter();
    let num_types = ty_iter.len();
    assert!(num_types <= arr.len());

    for (i, basic_ty) in ty_iter.enumerate() {
        arr[i] = basic_ty.as_basic_type_enum().into();
    }

    let ret_ty = Ret::new(ctx);
    let fn_ty = ret_ty.func_type(&arr[..num_types]);

    let codegen = FnCodegen::new(|_| module.add_function("func", fn_ty, None));

    codegen.func().add_attribute(
        AttributeLoc::Function,
        ctx.create_string_attribute("unsafe-fp-math", "true"),
    );

    let codegen_module = CodegenModule::new(module, codegen);

    Func {
        ret_ty,
        arg_types,
        codegen_module,
    }
}

pub fn create_kernel<ArgsT>() -> Func<ArgsT, Void>
where
    ArgsT: IntoFuncArgs,
{
    let func = create_func();
    func.cx_ref().func.set_call_conventions(71);
    func
}

pub trait IntoFuncArgs {
    type ArgValues<'lt>;
    fn get_args<'lt>(cx: &'lt CodegenModule<'static>) -> Self::ArgValues<'lt>;
    fn new_from_ctx(ctx: ContextRef<'static>) -> Self;
    fn basic_ty_iter(&self) -> impl ExactSizeIterator<Item = BasicTypeEnum<'static>>;
}

macro_rules! count {
    () => {
        0
    };
    ($first: ident $(, $rest: ident)*) => {
        1 + count!($($rest),*)
    };
}

macro_rules! impl_into_func_args {
    ($($args: ident),*; $($idx: tt),*) => {
        impl<$($args),*> IntoFuncArgs for ($($args,)*)
        where
            $($args: Ty),*
        {
            type ArgValues<'lt> = ($(Val<'lt, $args>,)*);
            fn get_args<'lt>(cx: &'lt CodegenModule<'static>) -> Self::ArgValues<'lt> {
                (
                    $(
                        $args::get_args_at_idx(cx, $idx),
                    )*
                )
            }
            fn new_from_ctx(ctx: ContextRef<'static>) -> Self {
                (
                    $(
                        $args::new(ctx),
                    )*
                )
            }
            fn basic_ty_iter(&self) -> impl ExactSizeIterator<Item = BasicTypeEnum<'static>> {
                [
                    $(
                        self.$idx.basic_ty().as_basic_type_enum(),
                    )*
                ].into_iter()
            }
        }
    };
}

impl_into_func_args!(A; 0);
impl_into_func_args!(A, B; 0, 1);
impl_into_func_args!(A, B, C; 0, 1, 2);
impl_into_func_args!(A, B, C, D; 0, 1, 2, 3);
impl_into_func_args!(A, B, C, D, E; 0, 1, 2, 3, 4);
impl_into_func_args!(A, B, C, D, E, F; 0, 1, 2, 3, 4, 5);
impl_into_func_args!(A, B, C, D, E, F, G; 0, 1, 2, 3, 4, 5, 6);
impl_into_func_args!(A, B, C, D, E, F, G, H; 0, 1, 2, 3, 4, 5, 6, 7);
impl_into_func_args!(A, B, C, D, E, F, G, H, I; 0, 1, 2, 3, 4, 5, 6, 7, 8);
