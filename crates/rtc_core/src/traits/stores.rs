use inkwell::values::PointerValue;

use crate::{
    traits::{HasCXVal, holder::Holds, indexes::IndexableTy},
    ty::{FromCtx, Ty},
    val::S,
};

pub trait Stores: Holds {
    fn get_ptr_to_value(val: &impl HasCXVal) -> PointerValue<'static>;

    fn get_ptr_at_idx(val: &impl HasCXVal, idx: usize) -> PointerValue<'static>
    where
        Self::T: IndexableTy,
    {
        let cx = val.cx();
        assert!(idx < Self::T::LEN);
        let ptr = Self::get_ptr_to_value(val);
        let pointee_ty = <Self::T as IndexableTy>::ElemT::new(cx.ctx()).basic_ty();
        let idx_val = cx.ctx().i32_type().const_int(idx as _, false);
        // SAFETY:
        // We have verified that the index is in-bounds for our indexable
        // held type.
        unsafe {
            cx.with_builder(|b| b.build_in_bounds_gep(pointee_ty, ptr, &[idx_val], "ptr_ref"))
        }
        .expect("Unable to build in-bounds GEP")
    }
}

impl<T> Stores for S<T>
where
    T: Ty,
{
    fn get_ptr_to_value(val: &impl HasCXVal) -> PointerValue<'static> {
        val.bval().into_pointer_value()
    }
}
