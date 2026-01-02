use inkwell::values::BasicValueEnum;

use crate::{codegen::FnCodegen, ty::Ty, val::S};

pub trait Holds {
    type T: Ty;
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn extract_value(
        cx: &FnCodegen<'static>,
        val: BasicValueEnum<'static>,
    ) -> <Self::T as Ty>::Value;
}

impl<T> Holds for T
where
    T: Ty,
{
    type T = Self;
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn extract_value(
        _: &FnCodegen<'static>,
        val: BasicValueEnum<'static>,
    ) -> <Self::T as Ty>::Value {
        Self::get_value(val)
    }
}

impl<T> Holds for S<T>
where
    T: Ty,
{
    type T = T;
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn extract_value(
        cx: &FnCodegen<'static>,
        val: BasicValueEnum<'static>,
    ) -> <Self::T as Ty>::Value {
        assert!(val.is_pointer_value(), "Must have a pointer value!");
        let ptr = val.into_pointer_value();
        let pointee_ty = T::new(cx.ctx()).basic_ty();
        let value_enum = unsafe { cx.with_builder(|b| b.build_load(pointee_ty, ptr, "hold_load")) }
            .expect("Unable to generate load!");
        T::get_value(value_enum)
    }
}
