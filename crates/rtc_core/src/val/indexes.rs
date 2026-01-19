use std::ops::Range;

use crate::{
    traits::{
        HasCXVal,
        holder::Holds,
        indexes::{IndexableMut, IndexablePtr, IndexableRef, IndexableTy},
        stores::Stores,
        vectorizable::VectorizableTy,
    },
    ty::{
        FromCtx, Ty,
        ptr::{M, P, R},
    },
    val::{S, Val},
};

impl<'lt, VecT> Val<'lt, VecT>
where
    VecT: IndexableTy,
{
    pub fn take_lanes<const LANES: usize>(
        self,
        indices: Range<usize>,
    ) -> Val<'lt, S<VecT::ParametrizedLen<LANES>>>
    where
        VecT::ElemT: VectorizableTy,
    {
        assert!(indices.end - indices.start == LANES);
        let ctx = self.cm().cx().ctx();
        let base_ty = VecT::ElemT::vec_ty(ctx, LANES);
        // SAFETY: We have passed in a VectorType with LANES num zeros, and are treating
        // this as a strongly typed Val<_, V<LANES>>. This is OK.
        let init_zero = unsafe { Val::new(self.cm(), base_ty.const_zero()) };
        let mut ret_val = init_zero.with_storage();

        let start = indices.start;

        let mut iter = self.elem_iter();
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

impl<'lt, VecPtr, VecT> Val<'lt, VecPtr>
where
    VecPtr: IndexablePtr<Pointee = VecT>,
    VecT: IndexableTy,
{
    pub fn ptr_at<'b>(&'b self, idx: usize) -> Val<'lt, P<VecT::ElemT>> {
        assert!(idx < VecT::LEN);
        let ptr = self.to_underlying();
        let cx = self.cm().cx();
        let ctx = cx.ctx();
        let pointee_ty = VecT::ElemT::new(self.cm().cx().ctx()).basic_ty();
        let idx = ctx.i32_type().const_int(idx as u64, false);
        let ptr_to_idx = unsafe {
            cx.with_builder(|b| b.build_in_bounds_gep(pointee_ty, ptr, &[idx], "ref_at_gep"))
        }
        .expect("Should be able to build GEP");
        // Safety: We have checked that this is in-bounds.
        unsafe { Val::new(self.cm(), ptr_to_idx) }
    }
}

impl<'lt, VecRef, VecT> Val<'lt, VecRef>
where
    VecRef: IndexableRef<Pointee = VecT>,
    VecT: IndexableTy,
{
    pub fn iter(self) -> impl ExactSizeIterator<Item = Val<'lt, R<&'lt VecT::ElemT>>>
    where
        VecRef: 'lt,
        VecT::ElemT: 'lt,
    {
        (0..VecT::LEN).map(move |i| VecRef::get_ref_at_idx(self, i))
    }

    pub fn ref_at<'b>(&'b self, idx: usize) -> Val<'lt, R<&'b VecT::ElemT>> {
        let ptr = self.ptr_at(idx);
        // Safety: This is in-bounds per ptr_at, we hold a shared reference,
        // and we have tied its lifetime to the borrow of self.
        unsafe { Val::new(self.cm(), ptr.to_underlying()) }
    }
}

impl<'lt, VecMut, VecT> Val<'lt, VecMut>
where
    VecMut: IndexableMut<Pointee = VecT>,
    VecT: IndexableTy,
{
    pub fn iter_mut(self) -> impl ExactSizeIterator<Item = Val<'lt, M<&'lt mut VecT::ElemT>>>
    where
        VecT::ElemT: 'lt,
    {
        (0..VecT::LEN).map(move |i| {
            let ptr = VecMut::get_ptr_at_idx(self, i);
            // Safety: We hold an exclusive reference and are handing out only
            // exclusive references to sub-objects which are disjoint
            unsafe { Val::new(self.cm(), ptr.to_underlying()) }
        })
    }

    pub fn mut_at<'b>(&'b mut self, idx: usize) -> Val<'lt, M<&'b mut VecT::ElemT>> {
        let ptr = self.ptr_at(idx);
        // Safety: This is in-bounds per ptr_at, we hold a shared reference,
        // and we have tied its lifetime to the borrow of self.
        unsafe { Val::new(self.cm(), ptr.to_underlying()) }
    }
}
