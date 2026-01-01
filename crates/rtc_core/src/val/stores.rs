use crate::{
    traits::{holder::Holds, indexes::IndexableTy, stores::Stores},
    ty::ptr::{M, R},
    val::Val,
};

impl<'lt, Storage> Val<'lt, Storage>
where
    Storage: Stores,
{
    pub fn get_ref<'a>(&'a self) -> Val<'a, R<&'a Storage::T>> {
        let ptr = Storage::get_ptr_to_value(self);
        // Safety: We are creating a shared reference with
        // lifetime tied to the shared borrow of self
        unsafe { Val::new(self.cm(), ptr) }
    }
    pub fn get_mut<'a>(&'a mut self) -> Val<'a, M<&'a mut Storage::T>> {
        let ptr = Storage::get_ptr_to_value(self);
        // Safety: We are creating a mutable reference with
        // lifetime tied to the mutable borrow of self
        unsafe { Val::new(self.cm(), ptr) }
    }
}

impl<'lt, Storage, VecT> Val<'lt, Storage>
where
    Storage: Stores<T = VecT>,
    VecT: IndexableTy,
{
    pub fn get_ref_at<'a>(&'a self, idx: usize) -> Val<'a, R<&'a VecT::ElemT>> {
        let ret = Storage::get_ptr_at_idx(self, idx);
        // SAFETY: get_ptr_at_idx returns only inbounds pointers and
        // we hold a shared reference tied to the lifetime 'a of our borrow
        unsafe { Val::new(self.cm(), ret) }
    }
    pub fn get_mut_at<'a>(&'a mut self, idx: usize) -> Val<'a, M<&'a mut VecT::ElemT>> {
        let ret = Storage::get_ptr_at_idx(self, idx);
        // SAFETY: get_ptr_at_idx returns only inbounds pointers and
        // we hold a mutable reference tied to the lifetime 'a of our borrow
        unsafe { Val::new(self.cm(), ret) }
    }
}
