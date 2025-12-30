use std::ops::Range;

use inkwell::{builder::Builder, values::VectorValue};

use crate::{
    primitives::{MutOps, RefOps},
    ty::{ArithmeticTy, FromCtx, Ty, VF, VecTy},
    val::{AcceptsConstants, Val},
};

impl<'lt, T, VecT> Val<'lt, VecT>
where
    T: Ty,
    VecT: VecTy<Value = VectorValue<'static>, ElemT = T> + 'lt,
{
    pub fn at_const(&self, idx: usize) -> Val<'lt, T> {
        let idx_val = self
            .cm()
            .cx()
            .ctx()
            .i64_type()
            .const_int(u64::try_from(idx).expect("usize -> u64 panic"), false);
        let vec_val = VecT::get_value(self.get_val());
        let idx_val = unsafe {
            self.cm()
                .cx()
                .with_builder(|b| b.build_extract_element(vec_val, idx_val, "idx_const"))
        }
        .expect("Unable to build extract to idx");
        Val::new(self.cm(), idx_val)
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = Val<'lt, VecT::ElemT>> {
        (0..VecT::N).map(|i| self.at_const(i))
    }

    pub fn sum(&self) -> Val<'lt, T>
    where
        T: ArithmeticTy + AcceptsConstants + 'lt,
    {
        let mut ret = T::new_const(T::ZERO, self.cm()).with_storage();
        let mut ret_ptr = ret.get_mut();
        for val in self.iter() {
            let new_val = ret_ptr.load() + val;
            ret_ptr.store(new_val);
        }
        ret_ptr.load()
    }
}
