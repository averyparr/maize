mod holder;
mod indexes;
mod ops;
mod ptr;
mod stores;

use std::marker::PhantomData;

use inkwell::values::{BasicValue, BasicValueEnum, InstructionValue};

use crate::{codegen::CodegenModule, ty::Ty};

/// Describes a generic 'Value' of a certain
/// type, with that type being either T or
/// held by some thin-wrapper around T (e.g. S<T>, C<T>)
#[derive(Clone, Copy)]
pub struct Val<'lt, T> {
    cm: &'lt CodegenModule<'static>,
    val: BasicValueEnum<'static>,
    phantom: PhantomData<T>,
}

/// Indicates that a value has backing storage
/// which makes it possible to e.g. take mutable
/// references to it.
/// Importantly, does *not* implement Ty itself
pub struct S<T>(PhantomData<T>);

impl<'lt, T> Val<'lt, T> {
    pub(crate) fn cm(&self) -> &'lt CodegenModule<'static> {
        &self.cm
    }
    pub(crate) fn val(&self) -> BasicValueEnum<'static> {
        self.val
    }
    pub(crate) unsafe fn new(cm: &'lt CodegenModule<'static>, val: T::Value) -> Self
    where
        T: Ty,
    {
        Self {
            cm,
            val: val.as_basic_value_enum(),
            phantom: PhantomData,
        }
    }

    pub fn with_storage(self) -> Val<'lt, S<T>> {
        let cg = self.cm.cx();
        let ptr = cg.build_alloca(self.val);
        let _: InstructionValue = unsafe {
            cg.with_builder(|b| b.build_store(ptr, self.val))
                .expect("Unable to build store")
        };
        Val {
            cm: self.cm,
            val: ptr.as_basic_value_enum(),
            phantom: PhantomData,
        }
    }

    pub fn to_underlying(&self) -> T::Value
    where
        T: Ty,
    {
        T::get_value(self.val())
    }
}
