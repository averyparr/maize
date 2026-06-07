use inkwell::{context::ContextRef, types::StructType, values::StructValue};

use crate::{tipe::Ty, val::Val};

mod cp_async_bulk;
mod scatter_gather;
mod tensor_tile;
mod tensormap;

pub struct TensorMap;
impl Ty for TensorMap {
    type LLType = StructType<'static>;
    type LLVal = StructValue<'static>;

    fn raw_ty(ctx: ContextRef<'static>) -> Self::LLType {
        let data = ctx.i64_type().array_type(16);
        ctx.struct_type(&[data.into()], false)
    }
    fn size() -> usize
    where
        Self: Sized,
    {
        return 128; // bytes
    }
    fn align() -> usize
    where
        Self: Sized,
    {
        return 128; // I'm not sure about this!
    }

    fn type_val(val: crate::backend::UntypedValue) -> Self::LLVal {
        val.0.into_struct_value()
    }

    fn const_val(self, fn_ref: crate::backend::FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        panic!(concat!(
            "tensor-maps are considered an opaque type; ",
            "use cuTensorMap* functions to create them at runtime, ",
            "and only interact via pointers",
        ));
    }
}
