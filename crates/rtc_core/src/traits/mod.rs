use inkwell::values::BasicValueEnum;

use crate::{codegen::FnCodegen, val::Val};

pub mod constants;
pub mod holder;
pub mod indexes;
pub mod ptr;
pub mod stores;
pub mod vectorizable;

pub trait HasCXVal {
    fn cx(&self) -> &FnCodegen<'static>;
    fn bval(&self) -> BasicValueEnum<'static>;
}

impl<'lt, T> HasCXVal for Val<'lt, T> {
    fn cx(&self) -> &FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}

impl<'lt, T> HasCXVal for &Val<'lt, T> {
    fn cx(&self) -> &FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}

impl<'lt, T> HasCXVal for &mut Val<'lt, T> {
    fn cx(&self) -> &FnCodegen<'static> {
        self.cm().cx()
    }
    fn bval(&self) -> BasicValueEnum<'static> {
        self.val()
    }
}
