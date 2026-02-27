use inkwell::values::{AggregateValue, BasicValue};

use crate::{
    ty::{SizedTy, Ty},
    val::Val,
};

impl<'a, T, const N: usize> Val<'a, [T; N]>
where
    T: SizedTy,
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
                .with_builder(|b| b.build_extract_value(raw, index as u32, "get_value_i"))
                .expect("extract_element failed");
            Val::new_from_value(self.cx(), element.into())
        })
    }

    pub fn array_from_elements(arr: [Val<'a, T>; N]) -> Self {
        let cx = arr[0].cx();
        let vec_ty = <[T; N]>::ty(cx.ctx());
        let mut arr_val = vec_ty.get_undef().as_aggregate_value_enum();

        for (i, scalar) in arr.iter().enumerate() {
            arr_val = unsafe {
                cx.with_builder(|b| {
                    b.build_insert_value(
                        arr_val.as_aggregate_value_enum(),
                        scalar.get_ll_typed(),
                        i as u32,
                        "insert",
                    )
                })
                .expect("Insert element should succeed")
            };
        }

        unsafe { Val::new_from_value(cx, arr_val.as_basic_value_enum()) }
    }

    pub fn array_splat(val: Val<'a, T>) -> Self
    where
        T: Copy,
    {
        Self::array_from_elements(::core::array::from_fn(|_| val.copy()))
    }
}
