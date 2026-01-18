use crate::{
    codegen::intrinsics::{
        UnaryIntrinsic,
        cuda::{Abs, AbsFtz, Exp2Approx, Exp2ApproxFtz},
    },
    traits::vectorizable::VectorizableTy,
    ty::{
        Ty, V,
        primitive::{F16, F16x2, F32, F64, HasFundamentalVectorTy},
    },
    val::Val,
};

const MAX_BULK_SIZE: usize = 2;

pub trait UnaryOp<T: Ty> {
    type BulkT: Ty;
    fn bulk_op(val: Val<'_, Self::BulkT>) -> Val<'_, Self::BulkT>;
    fn element_op(val: Val<'_, T>) -> Val<'_, T>;

    fn convert_to_bulk(_: Val<'_, V<T, MAX_BULK_SIZE>>) -> Option<Val<'_, Self::BulkT>> {
        None
    }
    fn convert_from_bulk(_: Val<'_, Self::BulkT>) -> Option<Val<'_, V<T, MAX_BULK_SIZE>>> {
        None
    }

    fn call(val: Val<'_, T>) -> Val<'_, T> {
        Self::element_op(val)
    }

    fn call_vec<const N: usize>(val: Val<'_, V<T, N>>) -> Val<'_, V<T, N>>
    where
        T: VectorizableTy,
    {
        let ctx = val.cm().cx().ctx();
        let maybe_pair = T::vec_ty(ctx, MAX_BULK_SIZE).get_undef();
        let mock_pair = unsafe { Val::new(val.cm(), maybe_pair) };
        let ret_ty = T::vec_ty(ctx, N);
        let mut ret_val =
            unsafe { Val::<V<T, N>>::new(val.cm(), ret_ty.const_zero()) }.with_storage();
        if Self::convert_to_bulk(mock_pair).is_some() {
            // bulk math!
            let (bulk_inp, rest_inp) = val.into_chunks();
            let (bulk_ret, rest_ret) = ret_val.get_mut().chunks_mut();
            for (mut ret, inp) in bulk_ret.zip(bulk_inp) {
                let inp = Self::convert_to_bulk(inp)
                    .expect("vec->bulk should either always or never fail");
                let retval = Self::bulk_op(inp);
                let retval = Self::convert_from_bulk(retval)
                    .expect("bulk->vec should either always or never fail");
                ret.store(retval);
            }
            for (mut ret, inp) in rest_ret.zip(rest_inp) {
                ret.store(Self::element_op(inp))
            }
        } else {
            // Element-wise math
            for (mut ret, inp) in ret_val.get_mut().iter_mut().zip(val.into_iter()) {
                ret.store(Self::element_op(inp));
            }
        }
        ret_val.get()
    }
}

macro_rules! impl_unary_op_from_intrinsic {
    ($op: ident, $opfn: ident $(, $tipe: ty$(:bulk:$bulk_ty: ty)?$(|$elem_ty: ty)?)*$(,)?) => {
        $(
            impl UnaryOp<$tipe> for $op {
                $(
                    type BulkT = $bulk_ty;
                    fn convert_from_bulk(val: Val<'_, Self::BulkT>) -> Option<Val<'_, V<$tipe, MAX_BULK_SIZE>>> {
                        Some(HasFundamentalVectorTy::__from_fundamental_vector_type(val))
                    }
                    fn convert_to_bulk(val: Val<'_, V<$tipe, MAX_BULK_SIZE>>) -> Option<Val<'_, Self::BulkT>> {
                        Some(HasFundamentalVectorTy::__as_fundamental_vector_type(val))
                    }
                )?
                $(
                    type BulkT=$elem_ty;
                )?
                fn bulk_op(bulk: Val<'_, Self::BulkT>) -> Val<'_, Self::BulkT> {
                    $op::call_intrinsic(bulk)
                }
                fn element_op(elem: Val<'_, $tipe>) -> Val<'_, $tipe> {
                    $op::call_intrinsic(elem)
                }
            }
        )*
        impl<T: Ty> Val<'_, T>
        where
            $op: UnaryOp<T>,
        {
            pub fn $opfn(self) -> Self {
                $op::call(self)
            }
        }

        impl<T: Ty, const N: usize> Val<'_, V<T, N>> {
            pub fn $opfn(self) -> Self
            where
                T: VectorizableTy,
                $op: UnaryOp<T>,
            {
                $op::call_vec(self)
            }
        }
    };
}

impl_unary_op_from_intrinsic!(
    Abs,
    abs,
    F16:bulk:F16x2,
    F32|F32,
    F64|F64,
);
impl_unary_op_from_intrinsic!(
    AbsFtz,
    abs_ftz,
    F16:bulk:F16x2,
    F32|F32,
);
impl_unary_op_from_intrinsic!(Exp2Approx, ex2_approx, F32 | F32);
impl_unary_op_from_intrinsic!(Exp2ApproxFtz, ex2_approx_ftz, F32 | F32);
