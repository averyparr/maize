use inkwell::values::{AnyValue, BasicValue};

use crate::{
    ty::{M, R, RefTy, V, ValTy, vec::VectorizableTy},
    val::Val,
};

impl<'a, T, const N: usize> Val<'a, V<T, N>>
where
    T: VectorizableTy,
{
    pub fn elements(self) -> [Val<'a, T>; N] {
        let raw = self.get_ll_typed();
        // Safety:
        //  - We extract elements from the vector using indices
        //          which are below the size of the vector, so
        //          the call to `.build_extract_element` are valid.
        //  - Similarly, each element is of type T, so the final
        //          cast is valid.
        ::core::array::from_fn(|index| unsafe {
            let element = self
                .cx()
                .with_builder(|b| {
                    b.build_extract_element(
                        raw,
                        self.cx().constant_from(index as u64).get_ll_typed(),
                        "get_elt_i",
                    )
                })
                .expect("extract_element failed");
            Val::new_from_value(self.cx(), element.into())
        })
    }

    pub fn from_elements(arr: [Val<'a, T>; N]) -> Self {
        let cx = arr[0].cx();
        let vec_ty = T::vec_ty(cx.ctx(), N);
        let mut vec_val = vec_ty.get_undef();

        for (i, scalar) in arr.iter().enumerate() {
            let idx = cx.constant_from(i as u64);
            vec_val = unsafe {
                cx.with_builder(|b| {
                    b.build_insert_element(
                        vec_val,
                        scalar.get_ll_typed(),
                        idx.get_ll_typed(),
                        "insert",
                    )
                })
                .expect("Insert element should succeed")
            };
        }

        unsafe { Val::new_from_value(cx, vec_val.as_basic_value_enum()) }
    }

    pub fn splat(val: Val<'a, T>) -> Self
    where
        T: Copy,
    {
        Self::from_elements(::core::array::from_fn(|_| val.copy()))
    }

    pub fn to_array(self) -> Val<'a, [T; N]> {
        Val::array_from_elements(self.elements())
    }
}

impl<'a, 'b, T, const N: usize> Val<'a, R<&'b V<T, N>>>
where
    T: VectorizableTy,
{
    pub fn elements_ref(self) -> [Val<'a, R<&'b T>>; N] {
        let raw = self.get_ll_typed();
        ::core::array::from_fn(|index| unsafe {
            let idx = self.cx().constant_from(index as u64).get_ll_typed();
            let element_ty = T::ty(self.ctx());
            let element = self
                .cx()
                .with_builder(|b| b.build_in_bounds_gep(element_ty, raw, &[idx], "gep_vec_ref"))
                .expect("GEP should succeed");
            Val::new(self.cx(), element.into())
        })
    }
}

impl<'a, 'b, T, const N: usize> Val<'a, M<&'b mut V<T, N>>>
where
    T: VectorizableTy,
{
    pub fn elements_ref<'c>(&'c self) -> [Val<'a, R<&'c T>>; N] {
        M::reborrow(&self).elements_ref()
    }
    pub fn elements_mut<'c>(&'c mut self) -> [Val<'a, M<&'c mut T>>; N] {
        let raw = self.get_ll_typed();
        ::core::array::from_fn(|index| unsafe {
            let idx = self.cx().constant_from(index as u64).get_ll_typed();
            let element_ty = T::ty(self.ctx());
            let element = self
                .cx()
                .with_builder(|b| b.build_in_bounds_gep(element_ty, raw, &[idx], "gep_vec_ref"))
                .expect("GEP should succeed");
            Val::new_from_value(self.cx(), element.as_basic_value_enum())
        })
    }
}
