use inkwell::{context::ContextRef, types::StructType, values::StructValue};

use crate::{
    backend::{FnRef, UntypedValue, VoidType},
    intrinsics::{Intrinsic, cuda::CUDA, impl_intrinsics},
    tipe::{A, Ty},
    val::Val,
};

#[derive(Clone, Copy)]
pub struct CanceledLaunch {
    x: u32,
    y: u32,
    z: u32,
    _ignore: u32,
}

impl Ty for CanceledLaunch {
    type LLType = StructType<'static>;
    type LLVal = StructValue<'static>;

    fn raw_ty(ctx: ContextRef<'static>) -> Self::LLType {
        let u32_ty = u32::raw_ty(ctx).into();
        ctx.struct_type(&[u32_ty, u32_ty, u32_ty, u32_ty], false)
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        val.0.into_struct_value()
    }

    fn const_val(self, fn_ref: FnRef) -> Val<Self>
    where
        Self: Copy,
    {
        let raw = Self::raw_ty(fn_ref.ctx()).const_named_struct(&[
            self.x.const_val(fn_ref.clone()).raw().0,
            self.y.const_val(fn_ref.clone()).raw().0,
            self.z.const_val(fn_ref.clone()).raw().0,
            self._ignore.const_val(fn_ref.clone()).raw().0,
        ]);
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}

impl_intrinsics!(
    ClusterLaunchControlTryCancel: "llvm.nvvm.clusterlaunchcontrol.try_cancel.async.shared"(A<*mut u128, 3>, A<*mut u64, 3>) -> VoidType,
    ClusterLaunchControlTryCancelMulticast: "llvm.nvvm.clusterlaunchcontrol.try_cancel.async.multicast.shared"(A<*mut u128, 3>, A<*mut u64, 3>) -> VoidType,
    ClusterLaunchControlQuery: "llvm.nvvm.clusterlaunchcontrol.query_cancel.is_canceled" (u128) -> bool,
    ClusterLaunchGetBlockX: "llvm.nvvm.clusterlaunchcontrol.query_cancel.get_first_ctaid.x"(u128) -> u32,
    ClusterLaunchGetBlockY: "llvm.nvvm.clusterlaunchcontrol.query_cancel.get_first_ctaid.y"(u128) -> u32,
    ClusterLaunchGetBlockZ: "llvm.nvvm.clusterlaunchcontrol.query_cancel.get_first_ctaid.z"(u128) -> u32,
    // As of LLVM 22.1.4, `clusterlaunchcontrol.query_cancel.get_first_ctaid.v4.b32.b128 {xdim, ydim, zdim, _},  try_cancel_response;` isn't supported by intrinsics
);

impl CUDA {
    pub fn clc_try_cancel(clc_ptr: Val<A<*mut u128, 3>>, mbar: Val<A<*mut u64, 3>>) {
        ClusterLaunchControlTryCancel.call((clc_ptr, mbar))
    }
    pub fn clc_try_cancel_multicast(clc_ptr: Val<A<*mut u128, 3>>, mbar: Val<A<*mut u64, 3>>) {
        ClusterLaunchControlTryCancelMulticast.call((clc_ptr, mbar))
    }
    pub fn clc_query_cancel(clc: Val<u128>) -> Val<bool> {
        ClusterLaunchControlQuery.call((clc,))
    }
    pub fn clc_get_bid_x(clc: Val<u128>) -> Val<u32> {
        ClusterLaunchGetBlockX.call((clc,))
    }
    pub fn clc_get_bid_y(clc: Val<u128>) -> Val<u32> {
        ClusterLaunchGetBlockY.call((clc,))
    }
    pub fn clc_get_bid_z(clc: Val<u128>) -> Val<u32> {
        ClusterLaunchGetBlockZ.call((clc,))
    }
}
