use inkwell::{
    AddressSpace,
    context::ContextRef,
    module::Module,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType},
    values::FunctionValue,
};

use std::marker::PhantomData;

use crate::{
    codegen::{Codegen, jit_func::PreJitFunc},
    cuda::PTXOptions,
    ty::BasicTy,
    val::Val,
};

const UPPER_BOUND_ON_PARAMS: usize = 16;

pub struct Func<'ctx, Args> {
    args: Args,
    module: Module<'ctx>,
    codegen: Codegen<'ctx>,
}

impl<'ctx, Args> Func<'ctx, Args>
where
    Args: IntoFuncArgs<'ctx>,
{
    fn new_from_fn_ty(
        ctx: ContextRef<'ctx>,
        fn_name: &str,
        fn_ty: FunctionType<'ctx>,
        args: Args,
    ) -> Self {
        let module = ctx.create_module("mod_fn");
        let codegen = Codegen::new(ctx, |_| module.add_function(fn_name, fn_ty, None));
        Func {
            args,
            module,
            codegen,
        }
    }
    pub fn new_with_return(
        ctx: ContextRef<'ctx>,
        fn_name: &str,
        ret_ty: impl BasicTy<'ctx> + 'ctx,
        args: Args,
    ) -> Self {
        let fn_ty = args.fn_type_with_return(ctx, ret_ty.basic_ty());
        Self::new_from_fn_ty(ctx, fn_name, fn_ty, args)
    }
    pub fn new_void(ctx: ContextRef<'ctx>, fn_name: &str, args: Args) -> Self {
        let fn_ty = args.void_fn_type(ctx);
        Self::new_from_fn_ty(ctx, fn_name, fn_ty, args)
    }
    pub fn extract_args(&self) -> Args::ArgValues {
        Args::get_args(self.codegen)
    }
    pub fn get_codegen(&self) -> &Codegen<'ctx> {
        &self.codegen
    }
    pub fn extract_module(self) -> Module<'ctx> {
        self.module
    }

    pub fn finalize_with_return<T>(self, retval: Val<'ctx, T>) -> PreJitFunc<'_, Args>
    where
        T: BasicTy<'ctx>,
    {
        let _ = self.codegen.with_builder(|b| {
            b.build_return(Some(&retval.to_value()))
                .expect("Should be possible to return a value")
        });
        PreJitFunc::new(self)
    }

    pub fn finalize(self) -> PreJitFunc<'ctx, Args> {
        let _ = self.codegen.with_builder(|b| {
            b.build_return(None)
                .expect("Should be possible to create return")
        });
        PreJitFunc::new(self)
    }
}

pub trait IntoFuncArgs<'ctx>: Sized + 'ctx {
    fn basic_ty_iter(&self) -> impl ExactSizeIterator<Item = BasicTypeEnum<'ctx>>;
    fn fn_type_with_return(
        &self,
        ctx: ContextRef<'ctx>,
        ret_ty: impl BasicType<'ctx>,
    ) -> FunctionType<'ctx> {
        let void_star = ctx.ptr_type(AddressSpace::default());
        let mut arr = [BasicMetadataTypeEnum::PointerType(void_star); UPPER_BOUND_ON_PARAMS];
        let ty_iter = self.basic_ty_iter();
        let num_types = ty_iter.len();
        assert!(num_types <= arr.len());
        for (i, basic_ty) in ty_iter.enumerate() {
            arr[i] = basic_ty.as_basic_type_enum().into();
        }
        ret_ty.fn_type(&arr[..num_types], false)
    }
    fn void_fn_type(&self, ctx: ContextRef<'ctx>) -> FunctionType<'ctx> {
        let void_star = ctx.ptr_type(AddressSpace::default());
        let mut arr = [BasicMetadataTypeEnum::PointerType(void_star); UPPER_BOUND_ON_PARAMS];
        let ty_iter = self.basic_ty_iter();
        let num_types = ty_iter.len();
        assert!(num_types <= arr.len());
        for (i, basic_ty) in ty_iter.enumerate() {
            arr[i] = basic_ty.as_basic_type_enum().into();
        }
        ctx.void_type().fn_type(&arr[..num_types], false)
    }
    type ArgValues;
    fn get_args(cx: Codegen<'ctx>) -> Self::ArgValues;
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
        impl<'ctx, $($args),*> IntoFuncArgs<'ctx> for ($($args,)*)
        where
            $($args: BasicTy<'ctx> + 'ctx),*
        {
            fn basic_ty_iter(&self) -> impl ExactSizeIterator<Item = BasicTypeEnum<'ctx>> {
                [
                    $(
                        self.$idx.basic_ty().as_basic_type_enum(),
                    )*
                ].into_iter()
            }
            type ArgValues = ($(Val<'ctx, $args>,)*);
            fn get_args(cx: Codegen<'ctx>) -> Self::ArgValues {
                (
                    $(
                        Val {
                            cx,
                            val: cx.func().get_nth_param($idx).expect("Param number mismatch!"),
                            phantom: PhantomData
                        },
                    )*
                )
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
