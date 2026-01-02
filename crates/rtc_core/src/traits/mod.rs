use inkwell::values::BasicValueEnum;

use crate::{codegen::FnCodegen, val::Val};

pub mod constants;
pub mod holder;
pub mod indexes;
pub mod ptr;
pub mod stores;
pub mod vectorizable;

pub trait HasCXVal {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &FnCodegen<'static>;
    fn bval(&self) -> BasicValueEnum<'static>;
}

impl<'lt, T> HasCXVal for Val<'lt, T> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}

impl<'lt, T> HasCXVal for &Val<'lt, T> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}

impl<'lt, T> HasCXVal for &mut Val<'lt, T> {
    #[expect(
        private_interfaces,
        reason = "We intend to only make this available through our API"
    )]
    fn cx(&self) -> &FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}
