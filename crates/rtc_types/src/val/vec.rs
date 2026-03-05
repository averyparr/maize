use crate::{
    ty::{ContiguousUniformTy, V, vec::VectorizableTy},
    val::Val,
};

impl<'a, T, const N: usize> Val<'a, V<T, N>>
where
    T: VectorizableTy,
{
    pub fn elements(self) -> [Val<'a, T>; N] {
        V::elements(self)
    }
    pub fn copy_elements(&self) -> [Val<'a, T>; N]
    where
        T: Copy,
    {
        V::copy_elements(self)
    }

    pub fn from_elements(arr: [Val<'a, T>; N]) -> Self {
        V::from_elements(arr)
    }

    pub fn to_array(self) -> Val<'a, [T; N]> {
        Val::array_from_elements(self.elements())
    }

    pub fn from_array(val: Val<'a, [T; N]>) -> Self {
        Val::from_elements(val.array_elements())
    }
    pub fn element_at(self, index: usize) -> Val<'a, T> {
        V::element(self, index)
    }
}

impl<'a, T> Val<'a, T>
where
    T: VectorizableTy,
{
    pub fn splat<const N: usize>(self) -> Val<'a, V<T, N>>
    where
        T: Copy,
    {
        V::splat(self)
    }
}
