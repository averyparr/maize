use crate::{
    struct_reflect,
    ty::{I32, P, U128, V, cuda::Shared},
    val::Val,
};

struct_reflect!(
    pub struct LDSMx2(pub I32, pub I32) => ldsmx2
);

struct_reflect!(
    pub struct LDSMx4{
        pub a: I32,
        pub b: I32,
        pub c: I32,
        pub d: I32,
    } => ldsmx4
);

pub unsafe fn call_ldsm_x1<'a>(ptr: Val<'a, Shared<P<*const U128>>>) -> Val<'a, V<I32, 1>> {
    let cx = ptr.cx();
    let name = "llvm.nvvm.ldmatrix.sync.aligned.m8n8.x1.b16";
    let function = cx.get_intrinsic::<I32, (Shared<P<*const U128>>,)>(name, false);
    let raw_ret = cx.call_fn(function, (ptr,));
    Val::from_elements([raw_ret])
}

pub unsafe fn call_ldsm_x2<'a>(ptr: Val<'a, Shared<P<*const U128>>>) -> Val<'a, V<I32, 2>> {
    let cx = ptr.cx();
    let name = "llvm.nvvm.ldmatrix.sync.aligned.m8n8.x2.b16";
    let function = cx.get_intrinsic::<LDSMx2, (Shared<P<*const U128>>,)>(name, false);
    let ret = cx.call_fn(function, (ptr,)).into_accessor();
    Val::from_elements([ret.0, ret.1])
}

pub unsafe fn call_ldsm_x4<'a>(ptr: Val<'a, Shared<P<*const U128>>>) -> Val<'a, V<I32, 4>> {
    let cx = ptr.cx();
    let name = "llvm.nvvm.ldmatrix.sync.aligned.m8n8.x4.b16";
    let function = cx.get_intrinsic::<LDSMx4, (Shared<P<*const U128>>,)>(name, false);
    let ret = cx.call_fn(function, (ptr,)).into_accessor();
    Val::from_elements([ret.a, ret.b, ret.c, ret.d])
}
