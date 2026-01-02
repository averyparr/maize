use crate::{
    traits::{holder::Holds, indexes::IndexableTy},
    val::Val,
};

impl<'lt, Holder> Val<'lt, Holder>
where
    Holder: Holds,
{
    pub fn get(self) -> Val<'lt, Holder::T> {
        Holder::extract_value(self)
    }
}
