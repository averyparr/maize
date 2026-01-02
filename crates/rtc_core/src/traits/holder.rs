use crate::{
    traits::HasCXVal,
    ty::Ty,
    val::{S, Val},
};

pub trait Holds {
    type T: Ty;
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn extract_value(val: Val<'_, Self>) -> Val<'_, Self::T>;
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
    fn extract_value(val: Val<'_, Self>) -> Val<'_, Self::T> {
        val
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
    fn extract_value(val: Val<'_, Self>) -> Val<'_, Self::T> {
        let cm = val.cm();
        let cx = val.cx();
        let val = val.val();
        assert!(val.is_pointer_value(), "Must have a pointer value!");
        let ptr = val.into_pointer_value();
        let pointee_ty = T::new(cx.ctx()).basic_ty();
        let value_enum = unsafe { cx.with_builder(|b| b.build_load(pointee_ty, ptr, "hold_load")) }
            .expect("Unable to generate load!");
        let val = T::get_value(value_enum);
        unsafe { Val::new(cm, val) }
    }
}
