use std::rc::Rc;

use inkwell::{
    types::{AnyType, AnyTypeEnum, BasicType},
    values::CallSiteValue,
};

use crate::{
    ContextRef,
    backend::{ErasedFuncType, ErasedType, FnCtx, FnRef, UntypedValue, VoidType, llvm::LLVM},
    func::args::FnArgs,
    tipe::Ty,
    val::Val,
};

pub trait FnRetTy {
    fn erased_func_type(ctx: ContextRef, args: &[ErasedType]) -> ErasedFuncType;
    fn define_func<Args: FnArgs>(llvm: Rc<LLVM>, name: &str) -> FnCtx;
    fn raw_inkwell_type(ctx: ContextRef) -> AnyTypeEnum<'static>;
    type RetVal;
    unsafe fn extract_call_site_value(fn_ref: FnRef, csv: CallSiteValue<'static>) -> Self::RetVal;
}

impl<T: Ty> FnRetTy for T {
    fn erased_func_type(ctx: ContextRef, args: &[ErasedType]) -> ErasedFuncType {
        ErasedType(T::raw_ty(ctx).as_basic_type_enum()).func_type(&args)
    }
    fn define_func<Args: FnArgs>(llvm: Rc<LLVM>, name: &str) -> FnCtx {
        let types = Args::raw_type_sequence(llvm.ctx());
        let ret_ty = ErasedType(T::raw_ty(llvm.ctx()).as_basic_type_enum());
        let fn_type = ret_ty.func_type(&types);
        let func = llvm.insert_untyped_func(name, fn_type);
        FnCtx::new(llvm, func)
    }
    fn raw_inkwell_type(ctx: ContextRef) -> AnyTypeEnum<'static> {
        Self::raw_ty(ctx).as_any_type_enum()
    }
    type RetVal = Val<Self>;
    unsafe fn extract_call_site_value(fn_ref: FnRef, csv: CallSiteValue<'static>) -> Self::RetVal {
        unsafe {
            Val::new(
                fn_ref,
                UntypedValue(
                    csv.try_as_basic_value()
                        .expect_basic("non-void ret should always have a value"),
                ),
            )
        }
    }
}

impl FnRetTy for VoidType {
    fn erased_func_type(ctx: ContextRef, args: &[ErasedType]) -> ErasedFuncType {
        VoidType(ctx.void_type()).func_type(args)
    }
    fn define_func<Args: FnArgs>(llvm: Rc<LLVM>, name: &str) -> FnCtx {
        let types = Args::raw_type_sequence(llvm.ctx());
        let ret_ty = VoidType(llvm.ctx().void_type());
        let fn_type = ret_ty.func_type(&types);
        let func = llvm.insert_untyped_func(name, fn_type);
        FnCtx::new(llvm, func)
    }
    fn raw_inkwell_type(ctx: ContextRef) -> AnyTypeEnum<'static> {
        ctx.void_type().into()
    }
    type RetVal = ();
    unsafe fn extract_call_site_value(_: FnRef, csv: CallSiteValue<'static>) -> Self::RetVal {
        let _ = csv
            .try_as_basic_value()
            .expect_instruction("Void ret type should never yield a value");
    }
}
