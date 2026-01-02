use std::ops::Range;

use inkwell::{
    types::VectorType,
    values::{PointerValue, VectorValue},
};

use crate::{
    traits::{
        holder::Holds,
        indexes::{IndexableMut, IndexableRef, IndexableTy},
        ptr::{MutTy, RefTy},
    },
    ty::{
        ArithmeticTy,
        ptr::{M, R},
    },
    val::Val,
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
