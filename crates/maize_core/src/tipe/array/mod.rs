mod indexing;

use inkwell::{
    types::{ArrayType, BasicType},
    values::ArrayValue,
};

use crate::{
    ContextRef,
    backend::{FnRef, UntypedValue},
    tipe::Ty,
    val::Val,
};

impl<T, const N: usize> Ty for [T; N]
where
    T: Ty,
{
    type LLType = ArrayType<'static>;
    type LLVal = ArrayValue<'static>;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        T::raw_ty(ctx).array_type(N as _)
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        val.0.into_array_value()
    }

    fn const_val(self, fn_ref: FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        let const_inners = self.map(|v| T::const_val(v, fn_ref.clone()).raw().0);
        let mut out = Self::raw_ty(fn_ref.ctx()).get_undef();
        let b = unsafe { fn_ref.curr_bb_builder() };
        for (index, value) in const_inners.into_iter().enumerate() {
            out = b
                .build_insert_value(out, value, index as _, "insert_arr")
                .expect("insert_value should succeed")
                .into_array_value();
        }
        unsafe { Val::new(fn_ref, UntypedValue(out.into())) }
    }
}

impl<T: Ty, const N: usize> Val<[T; N]> {
    pub fn elements(self) -> [Val<T>; N] {
        let raw = self.typed();
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        ::std::array::from_fn(|index| {
            let value = b
                .build_extract_value(raw, index as _, "element_decompose")
                .expect("Extract element should succeed");
            unsafe { Val::new(self.fn_ref().clone(), UntypedValue(value)) }
        })
    }
}
