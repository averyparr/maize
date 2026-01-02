use crate::{
    traits::{
        indexes::{IndexablePtr, IndexableRef, IndexableTy},
        stores::{Stores, StoresIndexable},
    },
    ty::ptr::{M, P, R},
    val::Val,
};

impl<'lt, Storage> Val<'lt, Storage>
where
    Storage: Stores,
{
    pub fn get_ptr<'a>(&self) -> Val<'a, P<Storage::T>>
    where
        'lt: 'a,
    {
        Storage::get_ptr_to_value(self)
    }
    pub fn get_ref<'a>(&'a self) -> Val<'lt, R<&'a Storage::T>> {
        Storage::get_ref_to_value(self)
    }
    pub fn get_mut<'a>(&'a mut self) -> Val<'a, M<&'a mut Storage::T>> {
        Storage::get_mut_to_value(self)
    }
}

impl<'lt, Storage, VecT> Val<'lt, Storage>
where
    Storage: StoresIndexable<T = VecT>,
    VecT: IndexableTy,
{
    pub fn get_ptr_at<'a>(&'a self, idx: usize) -> Val<'lt, P<<Storage::T as IndexableTy>::ElemT>>
    where
        'lt: 'a,
    {
        Storage::get_ptr_at(self, idx)
    }

    pub fn get_ref_at<'b>(
        &'b self,
        idx: usize,
    ) -> Val<'lt, R<&'b <Storage::T as IndexableTy>::ElemT>> {
        Storage::get_ref_at(self, idx)
    }

    pub fn get_mut_at<'b>(
        &'b mut self,
        idx: usize,
    ) -> Val<'lt, M<&'b mut <Storage::T as IndexableTy>::ElemT>> {
        Storage::get_mut_at(self, idx)
    }

    pub fn get_chunks<'b, const CHUNK_SIZE: usize>(
        &'b self,
    ) -> (
        impl ExactSizeIterator<
            Item = Val<'b, R<&'b <Storage::T as IndexableTy>::ParametrizedLen<CHUNK_SIZE>>>,
        >,
        impl ExactSizeIterator<Item = Val<'b, R<&'b <Storage::T as IndexableTy>::ElemT>>>,
    )
    where
        VecT: 'b,
    {
        R::chunks_ref(Storage::get_ref_to_value(self))
    }
}
