use std::ops::Index;

use crate::{
    codegen::{
        func_with_args::create_func,
        intrinsics::{
            BinaryIntrinsic, UnaryIntrinsic,
            cuda::{Exp2Fast, Log2Fast, Min},
        },
    },
    traits::{
        holder::Holds,
        indexes::{IndexableMut, IndexableRef, IndexableTy},
        stores::Stores,
        vec::BulkOps,
        vectorizable::VectorizableTy,
    },
    ty::{
        FromCtx, Ty, V, Void,
        primitive::{BF16, BF16x2, F16, F16x2, F32, F64},
        ptr::{M, R},
    },
    val::{S, Val},
};

impl<'a, VecT: IndexableTy> Val<'a, VecT> {
    pub fn elem_iter(self) -> impl ExactSizeIterator<Item = Val<'a, VecT::ElemT>> {
        IndexableTy::split_as_iterator(self)
    }
}

impl<'a, StoresVecT: Stores<T: IndexableTy>> Val<'a, StoresVecT> {
    pub const fn len(&self) -> usize {
        StoresVecT::T::LEN
    }
    pub fn elem_ref<'b>(
        &'b self,
    ) -> impl ExactSizeIterator<Item = Val<'b, R<&'b <StoresVecT::T as IndexableTy>::ElemT>>> {
        let len = self.len();
        let self_ref = self.get_ref();
        (0..len).map(move |i| IndexableRef::get_ref_at_idx(self_ref, i))
    }
    pub fn elem_mut<'b>(
        &'b mut self,
    ) -> impl ExactSizeIterator<Item = Val<'b, M<&'b mut <StoresVecT::T as IndexableTy>::ElemT>>>
    where
        'a: 'b,
    {
        let len = self.len();
        let self_mut = self.get_mut();
        (0..len).map(move |i| IndexableMut::get_mut_at_idx(self_mut, i))
    }
}

impl<'a, HoldsVecT: Holds<T = V<ElemT, N>>, ElemT: BulkOps, const N: usize> Val<'a, HoldsVecT> {
    fn map_elementwise<F, U>(self, f: F) -> Val<'a, S<V<U, N>>>
    where
        F: FnMut(Val<'a, ElemT>) -> Val<'a, U>,
        U: VectorizableTy,
    {
        let raw_vec_val = U::vec_ty(self.cm().cx().ctx(), N).const_zero();
        // Safety: This is initialized from a len-N U-vec so the cast is valid
        let mut ret = unsafe { Val::new(self.cm(), raw_vec_val) }.with_storage();
        let elem_iter = self.get().elem_iter().map(f);
        assert_eq!(elem_iter.len(), ret.len());
        ret.elem_mut()
            .zip(elem_iter)
            .for_each(|(mut r, e)| r.store(e));
        ret
    }

    fn zip_elementwise<F, U, HoldsRight>(
        self,
        other: Val<'a, HoldsRight>,
        mut f: F,
    ) -> Val<'a, S<V<U, N>>>
    where
        HoldsRight: Holds<T = V<ElemT, N>>,
        F: FnMut(Val<'a, ElemT>, Val<'a, ElemT>) -> Val<'a, U>,
        U: VectorizableTy,
    {
        let raw_vec_val = U::vec_ty(self.cm().cx().ctx(), N).const_zero();
        // Safety: This is initialized from a len-N U-vec so the cast is valid
        let mut ret = unsafe { Val::new(self.cm(), raw_vec_val) }.with_storage();
        let a_iter = self.get().elem_iter();
        let b_iter = other.get().elem_iter();
        for (mut r, (a, b)) in ret.elem_mut().zip(a_iter.zip(b_iter)) {
            r.store(f(a, b));
        }
        ret
    }

    fn map_bulk<FB, FE, U>(self, mut fe: FE, mut fb: FB) -> Val<'a, S<V<U, N>>>
    where
        FB: FnMut(Val<'a, ElemT::BulkT>) -> Val<'a, U::BulkT>,
        FE: FnMut(Val<'a, ElemT>) -> Val<'a, U>,
        U: BulkOps,
    {
        let raw_vec_val = U::vec_ty(self.cm().cx().ctx(), N).const_zero();
        // Safety: This is initialized from a len-N U-vec so the cast is valid
        let mut ret = unsafe { Val::new(self.cm(), raw_vec_val) }.with_storage();
        let (rbulk, rrest) = U::iter_bulk_mut(ret.get_mut());
        let (ebulk, erest) = ElemT::iter_bulk(self.get());
        for (mut r, e) in rbulk.zip(ebulk) {
            r.store(fb(e));
        }
        for (mut r, e) in rrest.zip(erest) {
            r.store(fe(e));
        }
        ret
    }

    fn zip_bulk<FE, FB, U, HoldsRight>(
        self,
        other: Val<'a, HoldsRight>,
        mut fe: FE,
        mut fb: FB,
    ) -> Val<'a, S<V<U, N>>>
    where
        HoldsRight: Holds<T = V<ElemT, N>>,
        FE: FnMut(Val<'a, ElemT>, Val<'a, ElemT>) -> Val<'a, U>,
        FB: FnMut(Val<'a, ElemT::BulkT>, Val<'a, ElemT::BulkT>) -> Val<'a, U::BulkT>,
        U: BulkOps,
    {
        let raw_vec_val = U::vec_ty(self.cm().cx().ctx(), N).const_zero();
        // Safety: This is initialized from a len-N U-vec so the cast is valid
        let mut ret = unsafe { Val::new(self.cm(), raw_vec_val) }.with_storage();
        let (ret_bulk, ret_rest) = U::iter_bulk_mut(ret.get_mut());
        let (a_bulk, a_rest) = ElemT::iter_bulk(self.get());
        let (b_bulk, b_rest) = ElemT::iter_bulk(other.get());
        for (mut r, (a, b)) in ret_bulk.zip(a_bulk.zip(b_bulk)) {
            r.store(fb(a, b));
        }
        for (mut r, (a, b)) in ret_rest.zip(a_rest.zip(b_rest)) {
            r.store(fe(a, b));
        }
        ret
    }
}
