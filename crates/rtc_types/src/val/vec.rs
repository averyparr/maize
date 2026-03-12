use crate::{
    ty::{ContiguousUniformTy, FixedSizeContiguousUniformTy, V, vec::VectorizableTy},
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

    // Ugh, this would be so much neater with generic const exprs
    pub fn chunks_exact<const M: usize>(self) -> Vec<Val<'a, V<T, M>>> {
        assert!(N % M == 0, "M must divide N");
        let mut elements = self.elements().into_iter();
        let mut ret = Vec::with_capacity(N / M);
        for _ in 0..N / M {
            let to_add =
                ::core::array::from_fn(|_| elements.next().expect("Size should have matched"));

            ret.push(Val::from_elements(to_add));
        }
        ret
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
