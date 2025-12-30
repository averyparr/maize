mod constants;
mod holder;
mod val;
mod vec;
mod with_storage;

pub use constants::{AcceptsConstants, C};
pub use holder::Holds;

use std::marker::PhantomData;

use inkwell::values::BasicValueEnum;

use crate::codegen::CodegenModule;

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
