use inkwell::{
    attributes::{Attribute, AttributeLoc},
    values::FunctionValue,
};

use crate::{
    struct_reflect,
    ty::{I32, P, U128, V, cuda::Shared},
    val::Val,
};

struct_reflect!(
    pub struct LDSMx2 {
        pub a: I32,
        pub b: I32
    } => ldsmx2
);

struct_reflect!(
    pub struct LDSMx4{
        pub a: I32,
        pub b: I32,
        pub c: I32,
        pub d: I32,
    } => ldsmx4
);

fn add_ldsm_attributes(fn_val: FunctionValue<'static>) {
    let add_attr = |attr| {
        let attr = Attribute::get_named_enum_kind_id(attr);
        let attr = fn_val
            .get_type()
            .get_context()
            .create_enum_attribute(attr, 0);
        fn_val.add_attribute(AttributeLoc::Function, attr);
    };
    add_attr("nosync");
    // This one is not quite true (ldsm is a warp-sync function iirc) but LLVM
    // currently marks the mma calls with nosync, so 90% sure this is fine
    add_attr("nosync");
    add_attr("mustprogress");
    // This one could allow LLVM to call ldsm outside of a loop checking
    // whether a pointer is valid and so is entirely unsafe
    // add_attr("speculatable");
    add_attr("willreturn");
}

pub unsafe fn call_ldsm_x1<'a>(ptr: Val<'a, P<*const U128, Shared>>) -> Val<'a, V<I32, 1>> {
    let cx = ptr.cx();
    let name = "llvm.nvvm.ldmatrix.sync.aligned.m8n8.x1.b16";
    let function = cx.get_intrinsic::<I32, (P<*const U128, Shared>,)>(name, false);
    add_ldsm_attributes(function.raw());
    let raw_ret = cx.call_fn(function, (ptr,));
    Val::from_elements([raw_ret])
}

pub unsafe fn call_ldsm_x2<'a>(ptr: Val<'a, P<*const U128, Shared>>) -> Val<'a, V<I32, 2>> {
    let cx = ptr.cx();
    let name = "llvm.nvvm.ldmatrix.sync.aligned.m8n8.x2.b16";
    let function = cx.get_intrinsic::<LDSMx2, (P<*const U128, Shared>,)>(name, false);
    add_ldsm_attributes(function.raw());
    let ret = cx.call_fn(function, (ptr,)).into_accessor();
    Val::from_elements([ret.a, ret.b])
}

pub unsafe fn call_ldsm_x4<'a>(ptr: Val<'a, P<*const U128, Shared>>) -> Val<'a, V<I32, 4>> {
    let cx = ptr.cx();
    let name = "llvm.nvvm.ldmatrix.sync.aligned.m8n8.x4.b16";
    let function = cx.get_intrinsic::<LDSMx4, (P<*const U128, Shared>,)>(name, false);
    add_ldsm_attributes(function.raw());
    let ret = cx.call_fn(function, (ptr,)).into_accessor();
    Val::from_elements([ret.a, ret.b, ret.c, ret.d])
}
