use std::marker::PhantomData;

use maize_core::{
    ContextRef, StructType, StructValue,
    backend::{ErasedType, FnCtx, FnRef, UntypedValue},
    jit_assert,
    tipe::{
        A, Ty,
        indexing::Indexable,
        reflection::{Extractor, insert_struct_val_at_idx},
    },
    val::Val,
};

pub struct GlobalSlice<T>(PhantomData<T>);
pub struct GlobalSliceMut<T>(PhantomData<T>);

impl<T> Clone for GlobalSlice<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
impl<T> Clone for GlobalSliceMut<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
impl<T> Copy for GlobalSlice<T> {}
impl<T> Copy for GlobalSliceMut<T> {}

impl<T: Ty> Ty for GlobalSlice<T> {
    type LLType = StructType;
    type LLVal = StructValue;

    fn raw_ty(ctx: maize_core::ContextRef) -> Self::LLType {
        ctx.struct_type(
            &[
                <A<*const T, 1>>::raw_ty(ctx).into(),
                u64::raw_ty(ctx).into(),
            ],
            true,
        )
    }

    fn type_val(val: maize_core::backend::UntypedValue) -> Self::LLVal {
        val.0.into_struct_value()
    }

    fn const_val(self, _: maize_core::backend::FnRef) -> maize_core::val::Val<Self>
    where
        Self: Sized,
    {
        panic!("Pass global slices to kernels; don't bake them in");
    }

    fn function_arg_types(ctx: ContextRef) -> impl ExactSizeIterator<Item = ErasedType> {
        [
            ErasedType::new(<A<*const T, 1>>::raw_ty(ctx)),
            ErasedType::new(u64::raw_ty(ctx)),
        ]
        .into_iter()
    }
    fn type_metadata_on_function(func: &FnCtx, param_indices: &mut impl Iterator<Item = u32>) {
        let ptr_idx = param_indices.next().expect("should be in range");
        let _ = param_indices.next();
        func.set_param_metadata(ptr_idx, "noalias", 0);
        func.set_param_metadata(ptr_idx, "readonly", 0);
        func.set_param_metadata(ptr_idx, "nonnull", 0);
    }
    unsafe fn extract_arg(fn_ref: FnRef, iter: &mut impl Iterator<Item = UntypedValue>) -> Val<Self>
    where
        Self: Sized,
    {
        let ptr = iter.next().expect("should be in bounds");
        let len = iter.next().expect("Should be in bounds");

        let ptr: Val<A<*const T, 1>> = unsafe { Val::new(fn_ref.clone(), ptr) };
        let len: Val<u64> = unsafe { Val::new(fn_ref.clone(), len) };

        let mut ret = Self::undef_val(fn_ref);
        ret = unsafe { insert_struct_val_at_idx(ret, 0, ptr) };
        ret = unsafe { insert_struct_val_at_idx(ret, 1, len) };

        ret
    }
}

impl<T: Ty> Ty for GlobalSliceMut<T> {
    type LLType = StructType;
    type LLVal = StructValue;

    fn raw_ty(ctx: maize_core::ContextRef) -> Self::LLType {
        ctx.struct_type(
            &[<A<*mut T, 1>>::raw_ty(ctx).into(), u64::raw_ty(ctx).into()],
            true,
        )
    }

    fn type_val(val: maize_core::backend::UntypedValue) -> Self::LLVal {
        val.0.into_struct_value()
    }

    fn const_val(self, _: maize_core::backend::FnRef) -> maize_core::val::Val<Self>
    where
        Self: Sized,
    {
        panic!("Pass global slices to kernels; don't bake them in");
    }

    fn function_arg_types(ctx: ContextRef) -> impl ExactSizeIterator<Item = ErasedType> {
        [
            ErasedType::new(<A<*mut T, 1>>::raw_ty(ctx)),
            ErasedType::new(u64::raw_ty(ctx)),
        ]
        .into_iter()
    }
    fn type_metadata_on_function(func: &FnCtx, param_indices: &mut impl Iterator<Item = u32>) {
        let ptr_idx = param_indices.next().expect("should be in range");
        let _ = param_indices.next();
        func.set_param_metadata(ptr_idx, "noalias", 0);
        func.set_param_metadata(ptr_idx, "nonnull", 0);
    }
    unsafe fn extract_arg(fn_ref: FnRef, iter: &mut impl Iterator<Item = UntypedValue>) -> Val<Self>
    where
        Self: Sized,
    {
        let ptr = iter.next().expect("should be in bounds");
        let len = iter.next().expect("Should be in bounds");

        let ptr: Val<A<*mut T, 1>> = unsafe { Val::new(fn_ref.clone(), ptr) };
        let len: Val<u64> = unsafe { Val::new(fn_ref.clone(), len) };

        let mut ret = Self::undef_val(fn_ref);
        ret = unsafe { insert_struct_val_at_idx(ret, 0, ptr) };
        ret = unsafe { insert_struct_val_at_idx(ret, 1, len) };

        ret
    }
}

impl<T: Ty + 'static> Indexable for GlobalSlice<T> {
    type IndexT = u64;
    type Target = A<&'static T, 1>;

    fn index(val: &Val<Self>, index: Val<Self::IndexT>) -> Val<Self::Target> {
        let ptr = Extractor::<Self, A<*const [T; 0], 1>>::new(0).field(val.copy());
        let len = Self::len(val);
        let inbounds = index.copy().lt(len);
        jit_assert!(inbounds);
        unsafe { ptr.index(index).assume_ref() }
    }

    fn len(val: &Val<Self>) -> Val<Self::IndexT> {
        Extractor::<Self, u64>::new(1).field(val.copy())
    }
}

impl<T: Ty + 'static> Indexable for GlobalSliceMut<T> {
    type IndexT = u64;
    type Target = A<&'static mut T, 1>;

    fn index(val: &Val<Self>, index: Val<Self::IndexT>) -> Val<Self::Target> {
        let ptr = Extractor::<Self, A<*mut [T; 0], 1>>::new(0).field(val.copy());
        let len = Self::len(val);
        let inbounds = index.copy().lt(len);
        jit_assert!(inbounds);
        unsafe { ptr.index(index).assume_mut() }
    }

    fn len(val: &Val<Self>) -> Val<Self::IndexT> {
        Extractor::<Self, u64>::new(1).field(val.copy())
    }
}
