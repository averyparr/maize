use inkwell::{
    types::BasicType,
    values::{AnyValue, BasicValue},
};

use crate::{ty::ValTy, val::Val};

pub mod cuda;

pub trait IntrinsicsLibrary {}

pub unsafe trait UnaryIntrinsic<Intrins>: ValTy {
    const INTRINSIC_NAME: &str;

    fn call_intrinsic(val: Val<'_, Self>) -> Val<'_, Self> {
        call_unary_intrinsic(val, Self::INTRINSIC_NAME)
    }
}

pub unsafe trait BinaryIntrinsic<Intrins>: ValTy {
    const INTRINSIC_NAME: &str;

    fn call_intrinsic<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self> {
        call_binary_intrinsic(lhs, rhs, Self::INTRINSIC_NAME)
    }
}

fn call_unary_intrinsic<'a, T: ValTy + ?Sized>(
    val: Val<'a, T>,
    intrinsic_name: &str,
) -> Val<'a, T> {
    let ty = T::ty(val.ctx()).as_basic_type_enum();
    let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
    let fn_val = val.cx().module().add_function(intrinsic_name, fn_ty, None);

    let call_site = unsafe {
        val.cx()
            .with_builder(|b| {
                b.build_call(
                    fn_val,
                    &[val.ll_typed().as_basic_value_enum().into()],
                    intrinsic_name,
                )
            })
            .expect("Unary call should succeed")
    };
    unsafe { Val::new(val.cx(), call_site.as_any_value_enum()) }
}

fn call_binary_intrinsic<'a, T: ValTy + ?Sized>(
    lhs: Val<'a, T>,
    rhs: Val<'a, T>,
    intrinsic_name: &str,
) -> Val<'a, T> {
    let ty = T::ty(lhs.ctx()).as_basic_type_enum();
    let fn_ty = ty.fn_type(&[ty.into(), ty.into()], false);
    let fn_val = lhs.cx().module().add_function(intrinsic_name, fn_ty, None);

    let call_site = unsafe {
        lhs.cx().with_builder(|b| {
            b.build_call(
                fn_val,
                &[
                    lhs.ll_typed().as_basic_value_enum().into(),
                    rhs.ll_typed().as_basic_value_enum().into(),
                ],
                intrinsic_name,
            )
        })
    }
    .expect("Binary call should succeed");
    unsafe { Val::new(lhs.cx(), call_site.as_any_value_enum()) }
}
