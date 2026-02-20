use inkwell::builder::Builder;
use inkwell::values::AnyValue;

use crate::ty::ValTy;
use crate::ty::raw::*;
use crate::val::Val;

/// Safety: Implementing this trait requires that nothing is done
/// with the `builder` other than emit e.g. a (T, T) -> T add
/// operation. This is true for all implementations in this file,
/// but downstream implementors must be careful.
pub unsafe trait MathTy: ValTy {
    fn add<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self>;
    fn sub<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self>;
    fn mul<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self>;
    fn div<'a>(lhs: Val<'a, Self>, rhs: Val<'a, Self>) -> Val<'a, Self>;
    fn neg<'a>(val: Val<'a, Self>) -> Val<'a, Self>;
}

macro_rules! impl_math_ty {
    (
        $($tipes: ty),*:
        add => $add: ident,
        sub => $sub: ident,
        mul => $mul: ident,
        div => $div: ident,
        neg => $neg: ident$(,)?
        ) => {
        $(
unsafe impl MathTy for $tipes {
    fn add<'a>(
        lhs: Val<'a, Self>,
        rhs: Val<'a, Self>,
    ) -> Val<'a, Self> {
        let build = |b: Builder<'a>| {
            b
            .$add(lhs.ll_typed(), rhs.ll_typed(), "add")
            .expect("Typed add should always succeed")
            .as_any_value_enum()
        };
        // Safety: add is (T, T) -> T
        unsafe { Val::new(lhs.cx(), lhs.cx().with_builder(build)) }
    }
    fn sub<'a>(
        lhs: Val<'a, Self>,
        rhs: Val<'a, Self>,
    ) -> Val<'a, Self> {
        let build = |b: Builder<'a>| {
            b
            .$sub(lhs.ll_typed(), rhs.ll_typed(), "sub")
            .expect("Typed sub should always succeed")
            .as_any_value_enum()
        };
        // Safety: sub is (T, T) -> T
        unsafe { Val::new(lhs.cx(), lhs.cx().with_builder(build)) }
    }
    fn mul<'a>(
        lhs: Val<'a, Self>,
        rhs: Val<'a, Self>,
    ) -> Val<'a, Self> {
        let build = |b: Builder<'a>| {
            b
            .$mul(lhs.ll_typed(), rhs.ll_typed(), "mul")
            .expect("Typed mul should always succeed")
            .as_any_value_enum()
        };
        // Safety: mul is (T, T) -> T
        unsafe { Val::new(lhs.cx(), lhs.cx().with_builder(build)) }
    }
    fn div<'a>(
        lhs: Val<'a, Self>,
        rhs: Val<'a, Self>,
    ) -> Val<'a, Self> {
        let build = |b: Builder<'a>| {
            b
            .$div(lhs.ll_typed(), rhs.ll_typed(), "div")
            .expect("Typed div should always succeed")
            .as_any_value_enum()
        };
        // Safety: div is (T, T) -> T
        unsafe { Val::new(lhs.cx(), lhs.cx().with_builder(build)) }
    }
    fn neg<'a>(val: Val<'a, Self>) -> Val<'a, Self> {
        let build = |b: Builder<'a>| {
            b
            .$neg(val.ll_typed(), "neg")
            .expect("Typed neg should always succeed")
            .as_any_value_enum()
        };
        // Safety: neg is (T) -> T
        unsafe { Val::new(val.cx(), val.cx().with_builder(build)) }
    }

}
        )*
    };
}

impl_math_ty!(
    BF16, F16, F32, F64, F128:
    add => build_float_add,
    sub => build_float_sub,
    mul => build_float_mul,
    div => build_float_div,
    neg => build_float_neg,
);

impl_math_ty!(
    I8, I16, I32, I64, I128:
    add => build_int_add,
    sub => build_int_sub,
    mul => build_int_mul,
    div => build_int_signed_div,
    neg => build_int_neg,
);

impl_math_ty!(
    U8, U16, U32, U64, U128:
    add => build_int_add,
    sub => build_int_sub,
    mul => build_int_mul,
    div => build_int_unsigned_div,
    neg => build_int_neg,
);
