use std::{marker::PhantomData, ops::Add};

use inkwell::values::BasicValue;

use crate::{ty::AddableTy, val::Val};

impl<'ctx, T> Add<Val<'ctx, T>> for Val<'ctx, T>
where
    T: AddableTy<'ctx>,
{
    type Output = Val<'ctx, T>;
    fn add(self, rhs: Val<'ctx, T>) -> Self::Output {
        let lhs_val = T::get_value(self.val);
        let rhs_val = T::get_value(rhs.val);
        let inner_val = self.cx.with_builder(|b| T::emit_add(b, lhs_val, rhs_val));
        Val {
            cx: self.cx,
            val: inner_val.as_basic_value_enum(),
            phantom: PhantomData,
        }
    }
}
