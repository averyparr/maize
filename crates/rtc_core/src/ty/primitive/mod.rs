mod bf16x2;
mod f16x2;
mod scalar;

pub use bf16x2::BF16x2;
pub use f16x2::F16x2;
use inkwell::values::VectorValue;
pub use scalar::*;

use crate::{
    traits::vectorizable::VectorizableTy,
    ty::{Ty, V},
    val::Val,
};

pub unsafe trait HasFundamentalVectorTy<const LEN: usize>: VectorizableTy {
    type VecTy: Ty<Value = VectorValue<'static>>;
    fn __as_fundamental_vector_type(val: Val<'_, V<Self, LEN>>) -> Val<'_, Self::VecTy> {
        // Safety: User promised these are the same type
        unsafe { Val::new(val.cm(), val.to_underlying()) }
    }
    fn __from_fundamental_vector_type(val: Val<'_, Self::VecTy>) -> Val<'_, V<Self, LEN>> {
        // Safety: User promised that these are the same type
        unsafe { Val::new(val.cm(), val.to_underlying()) }
    }
}
