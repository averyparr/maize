use crate::{
    ContextRef, StructType, StructValue,
    backend::{ErasedType, FnRef, UntypedValue},
    intrinsics::cuda::cuda,
    jit_assert,
    tipe::{
        Ty,
        indexing::Indexable,
        reflection::{Extractor, insert_struct_val_at_idx},
    },
    val::Val,
};

impl<'a, T: Ty> Ty for &'a [T] {
    type LLType = StructType;
    type LLVal = StructValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.struct_type(
            &[<*const T>::raw_ty(ctx).into(), u64::raw_ty(ctx).into()],
            true,
        )
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        val.0.into_struct_value()
    }

    fn const_val(self, _: FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        panic!("We should not be baking pointer values into JIT'ed functions");
    }
    fn function_arg_types(
        ctx: ContextRef,
    ) -> impl ExactSizeIterator<Item = crate::backend::ErasedType> {
        [
            ErasedType(<*const T>::raw_ty(ctx).into()),
            ErasedType(u64::raw_ty(ctx).into()),
        ]
        .into_iter()
    }
    fn type_metadata_on_function(
        func: &crate::backend::FnCtx,
        param_indices: &mut impl Iterator<Item = u32>,
    ) {
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

        let ptr: Val<*const T> = unsafe { Val::new(fn_ref.clone(), ptr) };
        let len: Val<u64> = unsafe { Val::new(fn_ref.clone(), len) };

        let mut ret = Self::undef_val(fn_ref);
        ret = unsafe { insert_struct_val_at_idx(ret, 0, ptr) };
        ret = unsafe { insert_struct_val_at_idx(ret, 1, len) };

        ret
    }
}

impl<'a, T: Ty> Ty for &'a mut [T] {
    type LLType = StructType;
    type LLVal = StructValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.struct_type(
            &[<*mut T>::raw_ty(ctx).into(), u64::raw_ty(ctx).into()],
            true,
        )
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        val.0.into_struct_value()
    }

    fn const_val(self, _: FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        panic!("We should not be baking pointer values into JIT'ed functions");
    }
    fn function_arg_types(
        ctx: ContextRef,
    ) -> impl ExactSizeIterator<Item = crate::backend::ErasedType> {
        [
            ErasedType(<*mut T>::raw_ty(ctx).into()),
            ErasedType(u64::raw_ty(ctx).into()),
        ]
        .into_iter()
    }
    fn type_metadata_on_function(
        func: &crate::backend::FnCtx,
        param_indices: &mut impl Iterator<Item = u32>,
    ) {
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

        let ptr: Val<*mut T> = unsafe { Val::new(fn_ref.clone(), ptr) };
        let len: Val<u64> = unsafe { Val::new(fn_ref.clone(), len) };

        let mut ret = Self::undef_val(fn_ref);
        ret = unsafe { insert_struct_val_at_idx(ret, 0, ptr) };
        ret = unsafe { insert_struct_val_at_idx(ret, 1, len) };

        ret
    }
}

impl<'a, T: Ty> Indexable for &'a [T] {
    type IndexT = u64;
    type Target = &'a T;
    fn index(val: &Val<Self>, index: Val<Self::IndexT>) -> Val<Self::Target> {
        let ptr = Extractor::<Self, *const [T; 0]>::new(0).field(val.copy());
        let len = Self::len(val);
        let inbounds = index.copy().lt(len);
        jit_assert!(inbounds);
        unsafe { ptr.index(index).assume_ref() }
    }
    fn len(val: &Val<Self>) -> Val<Self::IndexT> {
        Extractor::<Self, u64>::new(1).field(val.copy())
    }
}

impl<'a, T: Ty> Indexable for &'a mut [T] {
    type IndexT = u64;
    type Target = &'a mut T;
    fn index(val: &Val<Self>, index: Val<Self::IndexT>) -> Val<Self::Target> {
        let raw_slice = unsafe { Val::new(val.fn_ref().clone(), val.raw()) };
        let ptr = Extractor::<&'a [T], *const [T; 0]>::new(0).field(raw_slice.copy());
        let len = Self::len(val);
        let inbounds = index.copy().lt(len);
        jit_assert!(inbounds);
        unsafe { ptr.index(index).as_mut().assume_mut() }
    }
    fn len(val: &Val<Self>) -> Val<Self::IndexT> {
        let raw_slice = unsafe { Val::new(val.fn_ref().clone(), val.raw()) };
        Extractor::<&'a [T], u64>::new(1).field(raw_slice.copy())
    }
}
