use inkwell::{
    intrinsics::Intrinsic,
    types::BasicType,
    values::{AnyValue, BasicValue},
};

use crate::{ty::ValTy, val::Val};

pub mod abs;
pub mod cuda;
mod transcendental;
pub mod vector;

pub trait IntrinsicsLibrary {}

pub unsafe trait UnaryIntrinsic<Intrins>: ValTy {
    const INTRINSIC_NAME: &str;

    fn call_intrinsic(val: Val<'_, Self>, literal_name: bool) -> Val<'_, Self> {
        call_unary_intrinsic(val, Self::INTRINSIC_NAME, literal_name)
    }
}

pub unsafe trait BinaryIntrinsic<Intrins>: ValTy {
    const INTRINSIC_NAME: &str;

    fn call_intrinsic<'a>(
        lhs: Val<'a, Self>,
        rhs: Val<'a, Self>,
        literal_name: bool,
    ) -> Val<'a, Self> {
        call_binary_intrinsic(lhs, rhs, Self::INTRINSIC_NAME, literal_name)
    }
}

fn call_unary_intrinsic<'a, T: ValTy + ?Sized>(
    val: Val<'a, T>,
    intrinsic_name: &str,
    literal_name: bool,
) -> Val<'a, T> {
    let ty = T::ty(val.ctx()).as_basic_type_enum();
    let fn_val = if literal_name {
        let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
        val.cx().module().add_function(intrinsic_name, fn_ty, None)
    } else {
        let intrinsic = Intrinsic::find(intrinsic_name).expect("Passed an invalid intrinsic name");
        intrinsic
            .get_declaration(val.cx().module(), &[ty.into()])
            .expect("There should have been an intrinsic of this type")
    };

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

fn call_unary_intrinsic_with_fast_math<'a, T: ValTy + ?Sized>(
    val: Val<'a, T>,
    intrinsic_name: &str,
    literal_name: bool,
) -> Val<'a, T> {
    let res = call_unary_intrinsic(val, intrinsic_name, literal_name);
    if let Some(ins) = res.ll_typed().as_basic_value_enum().as_instruction_value() {
        res.cx().apply_ins_opt(ins);
    }
    res
}

fn call_binary_intrinsic<'a, T: ValTy + ?Sized>(
    lhs: Val<'a, T>,
    rhs: Val<'a, T>,
    intrinsic_name: &str,
    literal_name: bool,
) -> Val<'a, T> {
    let ty = T::ty(lhs.ctx()).as_basic_type_enum();
    let fn_val = if literal_name {
        let fn_ty = ty.fn_type(&[ty.into(), ty.into()], false);
        lhs.cx().module().add_function(intrinsic_name, fn_ty, None)
    } else {
        let intrinsic = Intrinsic::find(intrinsic_name).expect("Passed an invalid intrinsic name");
        intrinsic
            .get_declaration(lhs.cx().module(), &[ty.into(), ty.into()])
            .expect("There should have been an intrinsic of this type")
    };

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
