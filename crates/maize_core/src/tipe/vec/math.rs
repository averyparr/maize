use inkwell::{
    builder::{Builder, BuilderError},
    types::BasicType,
};

use crate::{
    backend::{FnRef, UntypedValue},
    tipe::{
        V, VecTy,
        math::{DivType, MathTy, SignedTy},
    },
    val::Val,
};

macro_rules! impl_vec_math {
    ($name: literal: $op_name: ident => $float_op: ident | $signed_op: ident | $unsigned_op: ident) => {
        fn $op_name(
            b: Builder<'static>,
            fn_ref: FnRef,
            lhs: UntypedValue,
            rhs: UntypedValue,
        ) -> Result<Val<Self>, BuilderError> {
            let lhs = lhs.0.into_vector_value();
            let rhs = rhs.0.into_vector_value();
            let elem_ty = lhs.get_type().get_element_type();
            assert_eq!(elem_ty, rhs.get_type().get_element_type());
            assert_eq!(elem_ty, T::raw_ty(fn_ref.ctx()).as_basic_type_enum());

            let res = match T::DIV_TYPE {
                DivType::Signed => b.$signed_op(lhs, rhs, concat!("ivec_", $name)),
                DivType::Unsigned => b.$unsigned_op(lhs, rhs, concat!("uvec_", $name)),
                DivType::Float => b.$float_op(lhs, rhs, concat!("fvec_", $name)),
            };
            res.map(|v| unsafe { Val::new(fn_ref, UntypedValue(v.into())) })
        }
    };
}

impl<T: VecTy, const N: usize> MathTy for V<T, N>
where
    T: MathTy,
{
    impl_vec_math!("add": try_emit_add => build_float_add | build_int_add | build_int_add);
    impl_vec_math!("sub": try_emit_sub => build_float_sub | build_int_sub | build_int_sub);
    impl_vec_math!("mul": try_emit_mul => build_float_mul | build_int_mul | build_int_mul);
    impl_vec_math!("div": try_emit_div => build_float_div | build_int_signed_div | build_int_unsigned_div);

    const DIV_TYPE: DivType = T::DIV_TYPE;
}

impl<T: VecTy, const N: usize> SignedTy for V<T, N>
where
    T: SignedTy,
{
    fn try_emit_neg(
        b: Builder<'static>,
        fn_ref: FnRef,
        val: UntypedValue,
    ) -> Result<Val<Self>, BuilderError> {
        let raw = val.0.into_vector_value();
        let elem_ty = raw.get_type().get_element_type();
        assert_eq!(elem_ty, T::raw_ty(fn_ref.ctx()).as_basic_type_enum());

        match T::DIV_TYPE {
            DivType::Signed => b.build_int_neg(raw, "ineg"),
            DivType::Unsigned => panic!("Typing errors -- should never negate an unsigned"),
            DivType::Float => b.build_float_neg(raw, "fneg"),
        }
        .map(|v| unsafe { Val::new(fn_ref, UntypedValue(v.into())) })
    }
}
