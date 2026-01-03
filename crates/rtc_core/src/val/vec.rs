use std::ops::Range;

use inkwell::{
    context::ContextRef,
    types::VectorType,
    values::{PointerValue, VectorValue},
};

use crate::{
    codegen::intrinsics::cuda::floatlike::FloatLike,
    traits::{
        HasCXVal,
        holder::Holds,
        indexes::{IndexableMut, IndexableRef, IndexableTy},
        ptr::{MutTy, RefTy},
        vectorizable::VectorizableTy,
    },
    ty::{
        ArithmeticTy, FromCtx, Ty, V, VecTy,
        primitive::{F16, F32},
        ptr::{M, R},
    },
    val::{S, Val},
};

impl<'lt, Holder, VecT> Val<'lt, Holder>
where
    Holder: Holds<T = VecT>,
    VecT: IndexableTy,
{
    pub fn into_iter(self) -> impl ExactSizeIterator<Item = Val<'lt, VecT::ElemT>> {
        VecT::split_as_iterator(self.get())
    }

    pub fn into_chunks<const CHUNK_SIZE: usize>(
        self,
    ) -> (
        impl ExactSizeIterator<Item = Val<'lt, VecT::ParametrizedLen<CHUNK_SIZE>>>,
        impl ExactSizeIterator<Item = Val<'lt, VecT::ElemT>>,
    ) {
        VecT::chunk_split(self.get())
    }
}

impl<'lt, VecRef, VecT> Val<'lt, VecRef>
where
    VecRef: IndexableRef<Pointee = VecT> + 'lt,
    VecT: IndexableTy,
{
    pub fn chunks<const CHUNK_SIZE: usize>(
        self,
    ) -> (
        impl ExactSizeIterator<Item = Val<'lt, R<&'lt VecT::ParametrizedLen<CHUNK_SIZE>>>>,
        impl ExactSizeIterator<Item = Val<'lt, R<&'lt VecT::ElemT>>>,
    ) {
        VecRef::chunks_ref(self)
    }
}

impl<'lt, VecMut, VecT> Val<'lt, VecMut>
where
    VecMut: IndexableMut<Pointee = VecT> + 'lt,
    VecT: IndexableTy,
{
    pub fn chunks_mut<const CHUNK_SIZE: usize>(
        self,
    ) -> (
        impl ExactSizeIterator<Item = Val<'lt, M<&'lt mut VecT::ParametrizedLen<CHUNK_SIZE>>>>,
        impl ExactSizeIterator<Item = Val<'lt, M<&'lt mut VecT::ElemT>>>,
    ) {
        VecMut::chunks_mut(self)
    }
}

const SUM_REDUCE_MAX_VECTORIZATION: usize = 2;

impl<'lt, Holder, VecT> Val<'lt, Holder>
where
    Holder: Holds<T = VecT>,
    VecT: IndexableTy,
    VecT::ElemT: ArithmeticTy,
    VecT::ParametrizedLen<SUM_REDUCE_MAX_VECTORIZATION>: ArithmeticTy,
{
    pub fn sum(self) -> Val<'lt, VecT::ElemT> {
        let raw_val = self.get();
        let (bulk, rest) = raw_val.into_chunks();

        let vector_sum = bulk
            .fold(None, |accum, e| {
                if let Some(val) = accum {
                    Some(val + e)
                } else {
                    Some(e)
                }
            })
            .map(|v| {
                v.into_iter().fold(None, |accum, e| {
                    if let Some(val) = accum {
                        Some(val + e)
                    } else {
                        Some(e)
                    }
                })
            })
            .map(|v| {
                v.expect("There should be at least one accum if we have even a single vector")
            });
        let scalar_sum = if let Some(vector_sum) = vector_sum {
            rest.fold(vector_sum, |accum, e| accum + e)
        } else {
            rest.fold(None, |accum, e| {
                if let Some(val) = accum {
                    Some(val + e)
                } else {
                    Some(e)
                }
            })
            .expect("There must be at least one element in the vector")
        };
        scalar_sum
    }
}

trait BulkMathOps {
    const BULK_SIZE: usize = 1;
}

impl BulkMathOps for F32 {}
impl BulkMathOps for F16 {
    const BULK_SIZE: usize = 2;
}

impl<'lt, VecT> Val<'lt, VecT>
where
    VecT: IndexableTy,
{
    fn map_elementwise<FE>(self, fe: FE) -> Self
    where
        FE: Fn(Val<'_, VecT::ElemT>) -> Val<'_, VecT::ElemT>,
    {
        let llvm_val = self.to_underlying().get_type().const_zero();
        let mut ret_val = unsafe { Val::new(self.cm(), llvm_val).with_storage() };

        for (mut ret, inc) in ret_val.get_mut().iter_mut().zip(self.into_iter()) {
            ret.store(fe(inc))
        }

        ret_val.get()
    }
}

impl<'lt, VecT> Val<'lt, VecT>
where
    VecT: IndexableTy,
    VecT::ElemT: BulkMathOps,
{
    fn perform_op_via_bulk_function<FB, FE>(self, fb: FB, fe: FE) -> Self
    where
        FB: Fn(Val<'_, VecT::ParametrizedLen<2>>) -> Val<'_, VecT::ParametrizedLen<2>>,
        FE: Fn(Val<'_, VecT::ElemT>) -> Val<'_, VecT::ElemT>,
    {
        if VecT::ElemT::BULK_SIZE == 1 {
            self.map_elementwise(fe)
        } else {
            let llvm_val = self.to_underlying().get_type().const_zero();
            let mut ret_val = unsafe { Val::new(self.cm(), llvm_val).with_storage() };

            let (bulk_inc, rest_inc) = self.into_chunks();
            let (bulk_ret, rest_ret) = ret_val.get_mut().chunks_mut();

            for (mut ret, inc) in bulk_ret.zip(bulk_inc) {
                ret.store(fb(inc));
            }

            for (mut ret, inc) in rest_ret.zip(rest_inc) {
                ret.store(fe(inc))
            }

            ret_val.get()
        }
    }
}

impl<'lt, const N: usize> Val<'lt, V<F32, N>> {
    pub fn abs_vec(self) -> Self {
        self.perform_op_via_bulk_function(|_| panic!(), |b| b.abs())
    }
}

impl<'lt, const N: usize> Val<'lt, V<F16, N>> {
    pub fn abs_vec(self) -> Self {
        self.perform_op_via_bulk_function(|b| b.abs(), |b| b.abs())
    }
}
