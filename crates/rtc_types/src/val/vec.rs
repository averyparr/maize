use inkwell::values::BasicValue;

use crate::{
    ty::{ContiguousUniformTy, V, vec::VectorizableTy},
    val::Val,
};

impl<'a, T, const N: usize> Val<'a, V<T, N>>
where
    T: VectorizableTy,
{
    pub fn elements(self) -> [Val<'a, T>; N] {
        let raw = self.ll_typed();
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
                        self.cx().constant_from(index as u64).ll_typed(),
                        "get_elt_i",
                    )
                })
                .expect("extract_element failed");
            Val::new(self.cx(), element.into())
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
                    b.build_insert_element(vec_val, scalar.ll_typed(), idx.ll_typed(), "insert")
                })
                .expect("Insert element should succeed")
            };
        }

        unsafe { Val::new(cx, vec_val.as_basic_value_enum()) }
    }

    pub fn to_array(self) -> Val<'a, [T; N]> {
        Val::array_from_elements(self.elements())
    }

    pub fn from_array(val: Val<'a, [T; N]>) -> Self {
        Val::from_elements(val.array_elements())
    }
}

impl<'a, T> Val<'a, T>
where
    T: VectorizableTy,
{
    pub fn splat<const N: usize>(self) -> Val<'a, V<T, N>>
    where
        T: Copy,
    {
        V::splat(self)
    }
}
