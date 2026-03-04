use std::marker::PhantomData;

use crate::intrinsics::cuda::CUDA;
use crate::ty::Void;
use crate::ty::{FnRetTy, IntoFuncArgs};

use super::target_cpu::cuda::SM;
use super::{FnCodegen, Func};

macro_rules! calling_conv {
    ($name: ident<Args> => $call_conv: literal | $cpu_config: ty | $intrinsics: ident) => {
        pub struct $name<Args>(FnCodegen, PhantomData<Args>);
        impl<Args: IntoFuncArgs> Func for $name<Args> {
            type Args = Args;
            type Ret = Void;
            type Intrinsics = $intrinsics;
            fn new(cx: FnCodegen) -> Self {
                Self(cx, PhantomData)
            }
            fn cx(&self) -> &FnCodegen {
                &self.0
            }
            const CALL_CONV: u32 = $call_conv;
            type CpuConfig = $cpu_config;
        }
    };
    ($name: ident<Args, Ret> => $call_conv: literal | $cpu_config: ty | $intrinsics: ident) => {
        pub struct $name<Args, Ret>(FnCodegen, PhantomData<(Args, Ret)>);
        impl<Args: IntoFuncArgs, Ret: FnRetTy> Func for $name<Args, Ret> {
            type Args = Args;
            type Ret = Ret;
            type Intrinsics = $intrinsics;
            fn new(cx: FnCodegen) -> Self {
                Self(cx, PhantomData)
            }
            fn cx(&self) -> &FnCodegen {
                &self.0
            }
            const CALL_CONV: u32 = $call_conv;
            type CpuConfig = $cpu_config;
        }
    };
}

calling_conv!(PTXKernel<Args> => 71 | SM | CUDA);
calling_conv!(PTXDevice<Args, Ret> => 72 | SM | CUDA);
