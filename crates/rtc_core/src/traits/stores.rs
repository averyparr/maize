use inkwell::values::PointerValue;

use crate::{
    traits::{HasCXVal, holder::Holds, indexes::IndexableTy},
    ty::{
        FromCtx, Ty,
        ptr::{M, P, R},
    },
    val::{S, Val},
};

pub trait Stores: Holds {
    fn get_ptr_to_value<'a>(val: &Val<'a, Self>) -> Val<'a, P<Self::T>> {
        let ptr = val.bval().into_pointer_value();
        unsafe { Val::new(val.cm(), ptr) }
    }

    fn get_ref_to_value<'a, 'b>(val: &'b Val<'a, Self>) -> Val<'a, R<&'b Self::T>> {
        let ptr = val.bval().into_pointer_value();
        unsafe { Val::new(val.cm(), ptr) }
    }

    fn get_mut_to_value<'a, 'b>(val: &'b mut Val<'a, Self>) -> Val<'a, M<&'b mut Self::T>> {
        let ptr = val.bval().into_pointer_value();
        unsafe { Val::new(val.cm(), ptr) }
    }
}

impl<T> Stores for S<T> where T: Ty {}

pub trait StoresIndexable: Stores<T: IndexableTy> + Sized {
    fn get_ptr_at<'b, 'a>(
        val: &'b Val<'a, Self>,
        idx: usize,
    ) -> Val<'a, P<<Self::T as IndexableTy>::ElemT>> {
        val.get_ptr().ptr_at(idx)
    }
    fn get_ref_at<'a, 'b>(
        val: &'b Val<'a, Self>,
        idx: usize,
    ) -> Val<'a, R<&'b <Self::T as IndexableTy>::ElemT>> {
        let ptr_at = Self::get_ptr_at(val, idx);
        // Safety: ptr_at checks in-bounds, and we have
        // tied the reference type lifetime to the borrow
        // of `val`.
        unsafe { Val::new(val.cm(), ptr_at.to_underlying()) }
    }

    fn get_mut_at<'a, 'b>(
        val: &'b mut Val<'a, Self>,
        idx: usize,
    ) -> Val<'a, M<&'b mut <Self::T as IndexableTy>::ElemT>> {
        let ptr_at = Self::get_ptr_at(val, idx);
        // Safety: ptr_at checks in-bounds, and we have
        // tied the reference type lifetime to the borrow
        // of `val`.
        unsafe { Val::new(val.cm(), ptr_at.to_underlying()) }
    }
}

impl<Storage> StoresIndexable for Storage
where
    Storage: Stores,
    Storage::T: IndexableTy,
{
}
