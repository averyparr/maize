use inkwell::types::{FloatType, IntType, PointerType, VectorType};

use crate::{ty::SizedTy, val::Val};

trait Bitcastable {}
impl Bitcastable for IntType<'_> {}
impl Bitcastable for FloatType<'_> {}
impl Bitcastable for VectorType<'_> {}
impl Bitcastable for PointerType<'_> {}

pub trait BitcastableTy: SizedTy {
    unsafe fn bitcast_to<To: BitcastableTy>(val: Val<'_, Self>) -> Val<'_, To> {
        let raw_val = unsafe {
            val.cx()
                .with_builder(|b| b.build_bit_cast(val.get_raw(), To::ty(val.ctx()), "bitcast"))
        }
        .expect("Bitcast should succeed here");
        unsafe { Val::new_from_value(val.cx(), raw_val) }
    }
}

impl<T> BitcastableTy for T where for<'a> T: SizedTy<Type<'a>: Bitcastable> {}

impl<'a, T> Val<'a, T>
where
    T: BitcastableTy,
{
    pub unsafe fn bitcast<To: BitcastableTy>(self) -> Val<'a, To> {
        let () = const {
            assert!(
                T::SIZE == To::SIZE,
                "Attempted to bitcast between values of different sizes"
            )
        };
        unsafe { T::bitcast_to(self) }
    }
}
