pub use args::FnArgs;
pub use ret::FnRetTy;

use std::{marker::PhantomData, rc::Rc};

use crate::{
    backend::{
        FnRef, InstructionFlags, Opt, ToCPU, UntypedFunc, UntypedValue, VoidType, llvm::LLVM,
    },
    func::callconv::CallConv,
    intrinsics::CUDA,
    tipe::Ty,
    val::Val,
};

pub mod args;
pub mod callconv;
mod ret;

pub struct Func<Ret, Args>(FnRef, PhantomData<(Ret, Args)>);
pub struct CallableFunc<Ret, Args>(Func<Ret, Args>);
pub struct ExternFunc<Ret, Args> {
    func: UntypedFunc,
    _phantom: PhantomData<(Ret, Args)>,
}

impl<Ret, Args: FnArgs> Func<Ret, Args> {
    pub unsafe fn new(fn_ref: FnRef) -> Self {
        Self(fn_ref, PhantomData)
    }
    pub fn args(&self) -> Args::ArgValues {
        Args::extract_and_type_args(self.0.clone())
    }
    pub fn run_passes(&self, cpu: impl ToCPU, opt: Opt) {
        self.0.run_passes(&cpu, opt);
    }
    pub fn compile(self, cpu: impl ToCPU, opt: Opt) -> Box<[u8]> {
        let single_context = self.0.try_get().expect("Should be only owner!");
        single_context.compile(&cpu, opt)
    }
    pub fn module_string(&self) -> String {
        self.0.module_string()
    }
    pub fn set_ins_flags(&self, f: impl FnOnce(&mut InstructionFlags)) {
        self.0.set_ins_flags(f);
    }
    pub fn cuda(&self) -> CUDA {
        CUDA(self.0.clone())
    }
    pub unsafe fn assume_callable(self) -> CallableFunc<Ret, Args> {
        CallableFunc(self)
    }
    pub fn constant<T: Ty>(&self, val: T) -> Val<T>
    where
        T: Copy,
    {
        val.const_val(self.0.clone())
    }
}

impl<Args: FnArgs> Func<VoidType, Args> {
    pub fn return_void(&self) {
        unsafe { self.0.curr_bb_builder().build_return(None) }
            .expect("Return building should succeed");
    }
}

impl<Ret: Ty, Args: FnArgs> Func<Ret, Args> {
    pub fn return_value(&self, val: Val<Ret>) {
        unsafe { self.0.curr_bb_builder().build_return(Some(&val.raw().0)) }
            .expect("Return value should have succeeded");
    }
}

impl<Ret, Args> ExternFunc<Ret, Args> {
    pub unsafe fn new(func: UntypedFunc) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }
    pub fn raw_func(&self) -> UntypedFunc {
        self.func
    }
}

impl<Ret, Args> CallableFunc<Ret, Args>
where
    Args: FnArgs,
    Ret: Ty,
{
    pub fn call(&self, args: Args::ArgValues) -> Val<Ret> {
        let b = unsafe { self.0.0.curr_bb_builder() };
        let args: Vec<_> = Args::arg_arr(args)
            .into_iter()
            .map(|v| v.0.into())
            .collect();
        let call = b
            .build_call(self.0.0.raw_func().0, &args, self.0.0.raw_func().name())
            .expect("Build call failed");
        let val = call
            .try_as_basic_value()
            .expect_basic("non-void return type should be a basic value");
        unsafe { Val::new(self.0.0.clone(), UntypedValue(val)) }
    }
}

impl<Args> CallableFunc<VoidType, Args>
where
    Args: FnArgs,
{
    pub fn call(&self, args: Args::ArgValues) {
        let b = unsafe { self.0.0.curr_bb_builder() };
        println!("{}", self.0.0.module_string());
        let args: Vec<_> = Args::arg_arr(args)
            .into_iter()
            .map(|v| v.0.into())
            .collect();
        assert!(false);
        let call = b
            .build_call(self.0.0.raw_func().0, &args, "csv")
            .expect("Build call failed");
        dbg!(call);
        let _ = call
            .try_as_basic_value()
            .expect_instruction("void return type should not be a basic value");
    }
}

pub fn implement_function<Ret: FnRetTy, Args: FnArgs>(llvm: LLVM, name: &str) -> Func<Ret, Args> {
    let ctx = Ret::define_func::<Args>(Rc::new(llvm), name);
    Func(FnRef::new(ctx), PhantomData)
}

pub fn implement_ptx_device<Ret: FnRetTy, Args: FnArgs>(llvm: LLVM, name: &str) -> Func<Ret, Args> {
    let ctx = Ret::define_func::<Args>(Rc::new(llvm), name);
    ctx.set_call_convention(CallConv::PTXDevice);
    ctx.set_function_metadata("willreturn", 0);
    Func(FnRef::new(ctx), PhantomData)
}

pub fn implement_ptx_kernel<Args: FnArgs>(llvm: LLVM, name: &str) -> Func<VoidType, Args> {
    let ctx = VoidType::define_func::<Args>(Rc::new(llvm), name);
    ctx.set_call_convention(CallConv::PTXKernel);
    ctx.set_function_metadata("willreturn", 0);
    ctx.set_function_metadata("mustprogress", 0);
    Func(FnRef::new(ctx), PhantomData)
}
