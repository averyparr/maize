use crate::{
    traits::{HasCXVal, holder::Holds},
    val::Val,
};

impl<'lt, Holder> Val<'lt, Holder>
where
    Holder: Holds,
{
    pub fn get(self) -> Val<'lt, Holder::T> {
        let held_value = Holder::extract_value(self.cx(), self.val());
        // Safety: We just extracted the held value, so
        // it should be of the correct type.
        unsafe { Val::new(self.cm(), held_value) }
    }
}
