use std::cell::Cell;
use std::marker::PhantomData;

use crate::ty::Void;
use crate::ty::{FnRetTy, IntoFuncArgs};

use super::instruction_opt::InstructionOpt;
use super::target::cuda::SM;
use super::{FnCodegen, Func};

macro_rules! calling_conv {
    ($name: ident<Args> => $call_conv: literal | $cpu_config: ty) => {
        pub struct $name<Args>(FnCodegen, Cell<InstructionOpt>, PhantomData<Args>);
        impl<Args: IntoFuncArgs> Func for $name<Args> {
            type Args = Args;
            type Ret = Void;
            fn new(cx: FnCodegen) -> Self {
                Self(cx, Cell::default(), PhantomData)
            }
            fn cx(&self) -> &FnCodegen {
                &self.0
            }
            const CALL_CONV: u32 = $call_conv;
            type CpuConfig = $cpu_config;
        }
    };
    ($name: ident<Args, Ret> => $call_conv: literal | $cpu_config: ty) => {
        pub struct $name<Args, Ret>(FnCodegen, Cell<InstructionOpt>, PhantomData<(Args, Ret)>);
        impl<Args: IntoFuncArgs, Ret: FnRetTy> Func for $name<Args, Ret> {
            type Args = Args;
            type Ret = Ret;
            fn new(cx: FnCodegen) -> Self {
                Self(cx, Cell::default(), PhantomData)
            }
            fn cx(&self) -> &FnCodegen {
                &self.0
            }
            const CALL_CONV: u32 = $call_conv;
            type CpuConfig = $cpu_config;
        }
    };
}

calling_conv!(PTXKernel<Args> => 71 | SM);
calling_conv!(PTXDevice<Args, Ret> => 72 | SM);
