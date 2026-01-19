use crate::{
    traits::{
        indexes::{IndexableMut, IndexableRef, IndexableTy},
        vectorizable::VectorizableTy,
    },
    ty::{
        Ty,
        primitive::{BF16, BF16x2, F16, F16x2, F32, F64},
        ptr::{M, R},
        vec::V,
    },
    val::Val,
};

pub unsafe trait BulkOps: VectorizableTy {
    type BulkT: Ty;
    const BULK_SIZE: usize = 1;
    fn from_bulk(val: Val<'_, Self::BulkT>) -> impl ExactSizeIterator<Item = Val<'_, Self>>;
    fn iter_bulk<VecT>(
        val: Val<'_, VecT>,
    ) -> (
        impl ExactSizeIterator<Item = Val<'_, Self::BulkT>>,
        impl ExactSizeIterator<Item = Val<'_, Self>>,
    )
    where
        VecT: IndexableTy<ElemT = Self>;
    fn iter_bulk_ref<'a, Ref: 'a + IndexableRef<Pointee: IndexableTy<ElemT = Self>>>(
        val: Val<'a, Ref>,
    ) -> (
        impl ExactSizeIterator<Item = Val<'a, R<&'a Self::BulkT>>>,
        impl ExactSizeIterator<Item = Val<'a, R<&'a Self>>>,
    )
    where
        Self::BulkT: 'a,
        Self: 'a;

    fn iter_bulk_mut<'a, Mut: 'a + IndexableMut<Pointee: IndexableTy<ElemT = Self>>>(
        val: Val<'a, Mut>,
    ) -> (
        impl ExactSizeIterator<Item = Val<'a, M<&'a mut Self::BulkT>>>,
        impl ExactSizeIterator<Item = Val<'a, M<&'a mut Self>>>,
    )
    where
        Self::BulkT: 'a,
        Self: 'a;
}

macro_rules! impl_bulk_ops {
    ($scalar: ty) => {
        unsafe impl BulkOps for $scalar {
            type BulkT = Self;
            const BULK_SIZE: usize = 1;
            fn from_bulk(
                val: Val<'_, Self::BulkT>,
            ) -> impl ExactSizeIterator<Item = Val<'_, Self>> {
                std::iter::once(val)
            }
            fn iter_bulk<VecT>(
                val: Val<'_, VecT>,
            ) -> (
                impl ExactSizeIterator<Item = Val<'_, Self::BulkT>>,
                impl ExactSizeIterator<Item = Val<'_, Self>>,
            )
            where
                VecT: IndexableTy<ElemT = Self>,
            {
                (std::iter::empty(), IndexableTy::split_as_iterator(val))
            }
            fn iter_bulk_ref<'a, Ref: 'a + IndexableRef<Pointee: IndexableTy<ElemT = Self>>>(
                val: Val<'a, Ref>,
            ) -> (
                impl ExactSizeIterator<Item = Val<'a, R<&'a Self::BulkT>>>,
                impl ExactSizeIterator<Item = Val<'a, R<&'a Self>>>,
            )
            where
                Self::BulkT: 'a,
                Self: 'a,
            {
                (
                    std::iter::empty(),
                    (0..Ref::Pointee::LEN).map(move |i| IndexableRef::get_ref_at_idx(val, i)),
                )
            }
            fn iter_bulk_mut<'a, Mut: 'a + IndexableMut<Pointee: IndexableTy<ElemT = Self>>>(
                val: Val<'a, Mut>,
            ) -> (
                impl ExactSizeIterator<Item = Val<'a, M<&'a mut Self::BulkT>>>,
                impl ExactSizeIterator<Item = Val<'a, M<&'a mut Self>>>,
            )
            where
                Self::BulkT: 'a,
                Self: 'a,
            {
                (
                    std::iter::empty(),
                    (0..Mut::Pointee::LEN).map(move |i| IndexableMut::get_mut_at_idx(val, i)),
                )
            }
        }
    };
    ($scalar: ty => $vec: ty) => {
        unsafe impl BulkOps for $scalar {
            type BulkT = $vec;
            const BULK_SIZE: usize = <$vec>::SIZE / <$scalar>::SIZE;
            fn from_bulk(
                val: Val<'_, Self::BulkT>,
            ) -> impl ExactSizeIterator<Item = Val<'_, Self>> {
                IndexableTy::split_as_iterator(val)
            }
            fn iter_bulk<VecT>(
                val: Val<'_, VecT>,
            ) -> (
                impl ExactSizeIterator<Item = Val<'_, Self::BulkT>>,
                impl ExactSizeIterator<Item = Val<'_, Self>>,
            )
            where
                VecT: IndexableTy<ElemT = Self>,
            {
                let (bulk, rest) = IndexableTy::chunk_split::<{ <$vec>::LEN }>(val);
                let bulk = bulk.map(|v| {
                    let raw_val = v.to_underlying();
                    // Safety: The vector type and its scalar equivalent have identical bit
                    // patterns so the cast is valid.
                    unsafe { Val::new(v.cm(), raw_val) }
                });
                (bulk, rest)
            }
            fn iter_bulk_ref<'a, Ref: 'a + IndexableRef<Pointee: IndexableTy<ElemT = Self>>>(
                val: Val<'a, Ref>,
            ) -> (
                impl ExactSizeIterator<Item = Val<'a, R<&'a Self::BulkT>>>,
                impl ExactSizeIterator<Item = Val<'a, R<&'a Self>>>,
            )
            where
                Self::BulkT: 'a,
                Self: 'a,
            {
                let (bulk, rest) = IndexableRef::chunks_ref::<{ <$vec>::LEN }>(val);
                let bulk = bulk.map(|v| {
                    let raw_val = v.to_underlying();
                    unsafe { Val::new(v.cm(), raw_val) }
                });
                (bulk, rest)
            }
            fn iter_bulk_mut<'a, Mut: 'a + IndexableMut<Pointee: IndexableTy<ElemT = Self>>>(
                val: Val<'a, Mut>,
            ) -> (
                impl ExactSizeIterator<Item = Val<'a, M<&'a mut Self::BulkT>>>,
                impl ExactSizeIterator<Item = Val<'a, M<&'a mut Self>>>,
            )
            where
                Self::BulkT: 'a,
                Self: 'a,
            {
                let (bulk, rest) = IndexableMut::chunks_mut::<{ <$vec>::LEN }>(val);
                let bulk = bulk.map(|v| {
                    let raw_val = v.to_underlying();
                    unsafe { Val::new(v.cm(), raw_val) }
                });
                (bulk, rest)
            }
        }
    };
}

impl_bulk_ops!(F64);
impl_bulk_ops!(F32);
impl_bulk_ops!(F16 => F16x2);
impl_bulk_ops!(BF16 => BF16x2);
