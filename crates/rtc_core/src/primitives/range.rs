use inkwell::types::IntMathType;

use crate::{ty::Ty, val::Val};

struct Range<'lt, T>(Val<'lt, T>, Val<'lt, T>);

impl<'lt, T> Range<'lt, T>
where
    T: Ty,
    T::Type: IntMathType<'static>,
{
}
