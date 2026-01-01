use std::ops::Range;

use crate::{traits::{indexes::IndexableTy, stores::Stores, vectorizable::VectorizableTy}, val::{S, Val}};
use crate::ty::FromCtx;

impl<'lt, VecT> Val<'lt, VecT>
where
    VecT: IndexableTy,
{
    pub const fn len() -> usize {
        VecT::LEN
    }

    pub fn as_iterator(&self) -> impl ExactSizeIterator<Item = Val<'lt, VecT::ElemT>> {
        let cm = self.cm();
        VecT::split_as_iterator(self).map(|v| 
            // SAFETY: We have just split our vector components
            // into sub-components
            unsafe { Val::new(cm, v) })
    }
}

impl<'lt, VecT> Val<'lt, VecT> where
VecT: IndexableTy,
VecT::ElemT: VectorizableTy {
    pub fn take_lanes<const LANES: usize>(self, indices: Range<usize>) -> Val<'lt, S<VecT::ParametrizedLen<LANES>>> {
        assert!(indices.end - indices.start == LANES);
        let ctx = self.cm().cx().ctx();
        let base_ty = VecT::ElemT::vec_ty(ctx, LANES);
        // SAFETY: We have passed in a VectorType with LANES num zeros, and are treating
        // this as a strongly typed Val<_, V<LANES>>. This is OK. 
        let init_zero = unsafe {Val::new(self.cm(), base_ty.const_zero())};
        let mut ret_val = init_zero.with_storage();

        let start = indices.start;

        let mut iter = self.as_iterator();
        for _ in 0..start {
            iter.next();
        }

        for i in indices {
            let new_val = iter.next().expect("Should be a value here!");
            ret_val.get_mut_at(i - start).store(new_val);
        }

        ret_val
    }
}