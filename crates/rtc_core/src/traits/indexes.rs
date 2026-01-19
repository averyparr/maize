use std::ops::Range;

use inkwell::{
    types::VectorType,
    values::{BasicValueEnum, VectorValue},
};

use crate::{
    codegen::{CodegenModule, FnCodegen},
    traits::{
        HasCXVal,
        ptr::{MutTy, PtrTy, RefTy},
        vectorizable::VectorizableTy,
    },
    ty::{
        FromCtx, Ty, V,
        ptr::{M, P, R},
    },
    val::Val,
};

pub trait IndexableTy: Ty<Value = VectorValue<'static>> {
    const LEN: usize;
    type ElemT: Ty;

    type ParametrizedLen<const M: usize>: IndexableTy<ElemT = Self::ElemT, Value = VectorValue<'static>, Type = VectorType<'static>>;

    fn split_as_iterator<'a>(
        val: Val<'a, Self>,
    ) -> impl ExactSizeIterator<Item = Val<'a, Self::ElemT>> {
        let cm_ref = val.cm();
        (0..Self::LEN)
            .map(move |i| {
                let basic_val = extract_element(cm_ref.cx(), val.bval().into_vector_value(), i);
                Self::ElemT::get_value(basic_val)
            })
            .map(|v| unsafe { Val::new(cm_ref, v) })
    }

    fn chunk_split<'a, const CHUNK_SIZE: usize>(
        val: Val<'a, Self>,
    ) -> (
        impl ExactSizeIterator<Item = Val<'a, Self::ParametrizedLen<CHUNK_SIZE>>>,
        impl ExactSizeIterator<Item = Val<'a, Self::ElemT>>,
    ) {
        let orig_vector = val.bval().into_vector_value();
        let cm_ref = val.cm();
        let bulk_size = Self::LEN / CHUNK_SIZE;
        let rem_offset = bulk_size * CHUNK_SIZE;
        let bulk = (0..bulk_size)
            .map(|i| CHUNK_SIZE * i)
            .map(move |chunk_start| {
                extract_range(val.cx(), orig_vector, chunk_start..chunk_start + CHUNK_SIZE)
            })
            .map(|v| unsafe { Val::new(cm_ref, v) });
        let rest = (rem_offset..Self::LEN)
            .map(move |i| Self::ElemT::get_value(extract_element(cm_ref.cx(), orig_vector, i)))
            .map(|s| unsafe { Val::new(cm_ref, s) });

        (bulk, rest)
    }
}

impl<T, const N: usize> IndexableTy for V<T, N>
where
    T: VectorizableTy,
{
    const LEN: usize = N;
    type ElemT = T;

    type ParametrizedLen<const K: usize> = V<T, K>;
}

fn extract_range(
    cx: &FnCodegen<'static>,
    val: VectorValue<'static>,
    range: Range<usize>,
) -> VectorValue<'static> {
    let mask: Vec<_> = range
        .map(|i| cx.ctx().i32_type().const_int(i as u64, false))
        .collect();
    // Safety: We are passing in a vector value.
    unsafe {
        cx.with_builder(|b| {
            b.build_shuffle_vector(
                val,
                val.get_type().get_undef(),
                VectorType::const_vector(&mask),
                "vec_shuffle",
            )
            .expect("Unable to generate shufflevec")
        })
    }
}

fn extract_element(
    cx: &FnCodegen<'static>,
    val: VectorValue<'static>,
    idx: usize,
) -> BasicValueEnum<'static> {
    let idx = cx.ctx().i32_type().const_int(idx as _, false);
    unsafe { cx.with_builder(|b| b.build_extract_element(val, idx, "extract_from_vector")) }
        .expect("Should be able to extract element!")
}

pub trait IndexablePtr: PtrTy<Pointee: IndexableTy> {
    fn get_ptr_at_idx<'a>(
        val: Val<'a, Self>,
        idx: usize,
    ) -> Val<'a, P<<Self::Pointee as IndexableTy>::ElemT>>
    where
        Self::Pointee: IndexableTy,
    {
        assert!(idx < Self::Pointee::LEN);
        let cx = val.cx();
        let ptr = val.to_underlying();
        let pointee_ty = <Self::Pointee as IndexableTy>::ElemT::new(cx.ctx()).basic_ty();
        let idx_val = cx.ctx().i32_type().const_int(idx as _, false);

        // SAFETY:
        // We have verified that the index is in-bounds for our indexable
        // held type.
        let ptr_to_elem = unsafe {
            cx.with_builder(|b| b.build_in_bounds_gep(pointee_ty, ptr, &[idx_val], "ptr_idx"))
        }
        .expect("Unable to build in-bounds GEP");

        unsafe { Val::new(val.cm(), ptr_to_elem) }
    }
    fn chunks_ptr<'a, const CHUNK_LEN: usize>(
        val: Val<'a, Self>,
    ) -> (
        impl ExactSizeIterator<
            Item = Val<'a, P<<Self::Pointee as IndexableTy>::ParametrizedLen<CHUNK_LEN>>>,
        >,
        impl ExactSizeIterator<Item = Val<'a, P<Self::Pointee>>>,
    )
    where
        Self: 'a,
    {
        let num_chunks = Self::Pointee::LEN / CHUNK_LEN;
        let rest_offset = num_chunks * CHUNK_LEN;

        let bulk = (0..num_chunks)
            .map(|c| c * CHUNK_LEN)
            .map(move |chunk_offset| {
                let ptr = Self::get_ptr_at_idx(val, chunk_offset);
                // Safety: This is just a pointer to a sub-vector
                unsafe { Val::new(val.cm(), ptr.to_underlying()) }
            });
        let rest = (rest_offset..Self::Pointee::LEN).map(move |i| {
            let ptr = Self::get_ptr_at_idx(val, i);
            // Safety: This is just a pointer to an individual element
            unsafe { Val::new(val.cm(), ptr.to_underlying()) }
        });
        (bulk, rest)
    }
}

pub trait IndexableRef: IndexablePtr {
    fn get_ref_at_idx<'a>(
        val: Val<'a, Self>,
        idx: usize,
    ) -> Val<'a, R<&'a <Self::Pointee as IndexableTy>::ElemT>>
    where
        Self::Pointee: IndexableTy,
    {
        let ptr = Self::get_ptr_at_idx(val, idx);
        // Safety: We hold a shared reference with lifetime 'b, so treating
        // this ptr as if it were a &'b T should be valid
        unsafe { Val::new(val.cm(), ptr.to_underlying()) }
    }
    fn chunks_ref<'lt, const CHUNK_LEN: usize>(
        val: Val<'lt, Self>,
    ) -> (
        impl ExactSizeIterator<
            Item = Val<'lt, R<&'lt <Self::Pointee as IndexableTy>::ParametrizedLen<CHUNK_LEN>>>,
        >,
        impl ExactSizeIterator<Item = Val<'lt, R<&'lt <Self::Pointee as IndexableTy>::ElemT>>>,
    )
    where
        Self: 'lt,
    {
        let (bulk, rest) = Self::chunks_ptr::<CHUNK_LEN>(val);
        let bulk = bulk.map(|c| {
            // Safety: We hold a shared reference so this cast is valid
            unsafe { Val::new(c.cm(), c.to_underlying()) }
        });
        let rest = rest.map(|e| {
            // Safety: We hold a shared reference so this cast is valid
            unsafe { Val::new(e.cm(), e.to_underlying()) }
        });

        (bulk, rest)
    }
}

pub trait IndexableMut: IndexableRef {
    fn get_mut_at_idx<'a, 'b>(
        val: Val<'a, Self>,
        idx: usize,
    ) -> Val<'a, M<&'b mut <Self::Pointee as IndexableTy>::ElemT>>
    where
        Self::Pointee: IndexableTy,
    {
        let ptr = Self::get_ptr_at_idx(val, idx);
        // Safety: We hold an exclusive reference with lifetime 'b, so treating
        // this ptr as if it were a &'b mut T should be valid
        unsafe { Val::new(val.cm(), ptr.to_underlying()) }
    }

    fn chunks_mut<'lt, const CHUNK_LEN: usize>(
        val: Val<'lt, Self>,
    ) -> (
        impl ExactSizeIterator<
            Item = Val<'lt, M<&'lt mut <Self::Pointee as IndexableTy>::ParametrizedLen<CHUNK_LEN>>>,
        >,
        impl ExactSizeIterator<Item = Val<'lt, M<&'lt mut <Self::Pointee as IndexableTy>::ElemT>>>,
    )
    where
        Self: 'lt,
    {
        let (bulk, rest) = Self::chunks_ptr::<CHUNK_LEN>(val);
        let bulk = bulk.map(|c| {
            // Safety: We hold an exclusive reference, and we are only handing
            // out mutable references to disjoint sub-vectors of our value
            unsafe { Val::new(c.cm(), c.to_underlying()) }
        });
        let rest = rest.map(|e| {
            // Safety: We hold an exclusive reference, and we are only handing
            // out mutable references to disjoint elements of our value
            unsafe { Val::new(e.cm(), e.to_underlying()) }
        });

        (bulk, rest)
    }
}

impl<Ptr> IndexablePtr for Ptr
where
    Ptr: PtrTy,
    Ptr::Pointee: IndexableTy,
{
}

impl<Ref> IndexableRef for Ref
where
    Ref: RefTy,
    Ref::Pointee: IndexableTy,
{
}

impl<Mut> IndexableMut for Mut
where
    Mut: MutTy,
    Mut::Pointee: IndexableTy,
{
}
