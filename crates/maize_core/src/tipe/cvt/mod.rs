mod bool;
mod float;
mod int;

use crate::val::Val;

pub trait ConvertTo<Other>: Sized {
    fn cvt(val: Val<Self>) -> Val<Other>;
}

impl<T> Val<T> {
    pub fn cvt<Other>(self) -> Val<Other>
    where
        T: ConvertTo<Other>,
    {
        ConvertTo::cvt(self)
    }
}
