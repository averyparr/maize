use std::marker::PhantomData;

use inkwell::attributes::AttributeLoc;

use crate::intrinsics::IntrinsicsLibrary;
use crate::intrinsics::cuda::CUDA;
use crate::ty::Void;
use crate::ty::{FnRetTy, IntoFuncArgs};

use super::target_cpu::cuda::SM;
use super::{FnCodegen, Func};

macro_rules! calling_conv {
    ($name: ident<Args> => $call_conv: literal | $cpu_config: ty | $intrinsics: ident $((AT_CREATION: $at_creation: expr))?) => {
        pub struct $name<Args>(FnCodegen, PhantomData<Args>);
        impl<Args: IntoFuncArgs> Func for $name<Args> {
            type Args = Args;
            type Ret = Void;
            type Intrinsics = $intrinsics;
            fn new(cx: FnCodegen) -> Self {
                $(($at_creation)(&cx);)?
                Self(cx, PhantomData)
            }
            fn cx(&self) -> &FnCodegen {
                &self.0
            }
            const CALL_CONV: u32 = $call_conv;
            type CpuConfig = $cpu_config;
        }
    };
    ($name: ident<Args, Ret> => $call_conv: literal | $cpu_config: ty | $intrinsics: ident $((AT_CREATION: $at_creation: expr))?) => {
        pub struct $name<Args, Ret>(FnCodegen, PhantomData<(Args, Ret)>);
        impl<Args: IntoFuncArgs, Ret: FnRetTy> Func for $name<Args, Ret> {
            type Args = Args;
            type Ret = Ret;
            type Intrinsics = $intrinsics;
            fn new(cx: FnCodegen) -> Self {
                $(($at_creation)(&cx);)?
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

impl<Args: IntoFuncArgs> PTXKernel<Args> {
    pub fn with_launch_bounds_1d(
        self,
        threads_per_block: u32,
        min_blocks_per_sm: Option<u32>,
        cluster_dims: Option<u32>,
    ) -> Self {
        self.with_launch_bounds(
            (threads_per_block, 1, 1),
            min_blocks_per_sm,
            cluster_dims.map(|i| (i, 1, 1)),
        )
    }
    pub fn with_launch_bounds_2d(
        self,
        threads_per_block: (u32, u32),
        min_blocks_per_sm: Option<u32>,
        cluster_dims: Option<(u32, u32)>,
    ) -> Self {
        let (bdimx, bdimy) = threads_per_block;
        self.with_launch_bounds(
            (bdimx, bdimy, 1),
            min_blocks_per_sm,
            cluster_dims.map(|(i, j)| (i, j, 1)),
        )
    }

    pub fn with_launch_bounds(
        self,
        threads_per_block: (u32, u32, u32), // .reqntid, not .maxntid
        min_blocks_per_sm: Option<u32>,     // .minnctapersm
        cluster_dims: Option<(u32, u32, u32)>, // deduces .maxclusterrank
                                            // and also .reqnctapercluster
    ) -> Self {
        // We don't allow maxnreg because this will limit the
        // compiler's ability to optimize and is as well served
        // with the preceeding options.
        //

        let ctx = self.cx().ctx();

        let add_attr = |val_as_str: String, name| {
            let local_attr = ctx.create_string_attribute(name, &val_as_str);
            self.cx()
                .func()
                .add_attribute(AttributeLoc::Function, local_attr);
        };

        let cuda = self.intrinsics();
        let (bdim_x, bdim_y, bdim_z) = threads_per_block;
        add_attr(format!("{bdim_x},{bdim_y},{bdim_z}"), "nvvm.reqntid");
        unsafe { cuda.assume(cuda.bdim_x().eq_const(bdim_x)) };
        unsafe { cuda.assume(cuda.bdim_y().eq_const(bdim_y)) };
        unsafe { cuda.assume(cuda.bdim_z().eq_const(bdim_z)) };

        if let Some(min_blocks_per_sm) = min_blocks_per_sm {
            add_attr(min_blocks_per_sm.to_string(), "nvvm.minctasm");
        }

        if let Some((clx, cly, clz)) = cluster_dims {
            add_attr(format!("{clx},{cly},{clz}"), "nvvm.cluster_dim");
            unsafe { cuda.assume(cuda.sregs().nclusterid_x().eq_const(clx)) };
            unsafe { cuda.assume(cuda.sregs().nclusterid_y().eq_const(cly)) };
            unsafe { cuda.assume(cuda.sregs().nclusterid_z().eq_const(clz)) };
            add_attr(format!("{}", clx * cly * clz), "maxclusterrank");
        }

        self
    }
}
