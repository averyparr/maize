use inkwell::values::VectorValue;

use crate::{
    traits::{HasCXVal, vectorizable::VectorizableTy},
    ty::{Ty, V},
};

pub trait IndexableTy: Ty {
    const LEN: usize;
    type ElemT: Ty;

    type ParametrizedLen<const M: usize>: IndexableTy<ElemT = Self::ElemT, Value = VectorValue<'static>>;

    fn split_as_iterator(
        val: impl HasCXVal,
    ) -> impl ExactSizeIterator<Item = <Self::ElemT as Ty>::Value>;
}

impl<T, const N: usize> IndexableTy for V<T, N>
where
    T: VectorizableTy,
{
    const LEN: usize = N;
    type ElemT = T;

    type ParametrizedLen<const M: usize> = V<T, M>;

    fn split_as_iterator(
        val: impl HasCXVal,
    ) -> impl ExactSizeIterator<Item = <Self::ElemT as Ty>::Value> {
        (0..N).map(move |i| {
            let cx = val.cx();
            let idx = cx.ctx().i32_type().const_int(i as _, false);
            let basic_val = unsafe {
                cx.with_builder(|b| {
                    b.build_extract_element(
                        val.bval().into_vector_value(),
                        idx,
                        "extract_from_vector",
                    )
                })
            }
            .expect("Should be able to extract element!");
            Self::ElemT::get_value(basic_val)
        })
    }
}
