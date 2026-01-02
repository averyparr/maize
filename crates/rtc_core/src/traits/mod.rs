use inkwell::values::BasicValueEnum;

use crate::{codegen::FnCodegen, val::Val};

pub mod constants;
pub mod holder;
pub mod indexes;
pub mod ptr;
pub mod stores;
pub mod vec;
pub mod vectorizable;

pub trait HasCXVal<'a> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &'a FnCodegen<'static>;
    fn bval(&self) -> BasicValueEnum<'static>;
}

impl<'lt, T: ?Sized> HasCXVal<'lt> for Val<'lt, T> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&'_ self) -> &'lt FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}

impl<'lt, T> HasCXVal<'lt> for &Val<'lt, T> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &'lt FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}

impl<'lt, T> HasCXVal<'lt> for &mut Val<'lt, T> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &'lt FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}
