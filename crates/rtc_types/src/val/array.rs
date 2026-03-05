use crate::{
    ty::{ContiguousUniformTy, MutTy, R, RefTy, SizedTy, ValTy},
    val::Val,
};

impl<'a, T, const N: usize> Val<'a, [T; N]>
where
    T: SizedTy,
{
    pub fn array_elements(self) -> [Val<'a, T>; N] {
        <[T; N]>::elements(self)
    }

    pub fn array_from_elements(arr: [Val<'a, T>; N]) -> Self {
        <[T; N]>::from_elements(arr)
    }
}

impl<'a, T> Val<'a, T> {
    pub fn array_splat<const N: usize>(self) -> Val<'a, [T; N]>
    where
        T: Copy + ValTy,
    {
        <[T; N]>::splat(self)
    }
}

impl<'a, 'b, Ref, T, const N: usize> Val<'a, Ref>
where
    Ref: RefTy<PointeeTy = [T; N]>,
    T: ValTy + 'a,
{
    pub fn index_ref<'c>(&'c self, index: usize) -> Val<'a, R<&'c T>> {
        <[T; N]>::element_ref::<Ref>(Ref::reborrow(self), index.try_into().expect("u32 overflow"))
    }
    pub fn elements_ref<'c>(&'c self) -> [Val<'a, Ref::Ref<'c, T>>; N] {
        <[T; N]>::elements_ref::<Ref>(self.reborrow())
    }
}

impl<'a, 'b, Mut, T, const N: usize> Val<'a, Mut>
where
    Mut: MutTy<PointeeTy = [T; N]>,
    T: ValTy + 'a,
{
    pub fn index_mut<'c>(&'c mut self, index: usize) -> Val<'a, Mut::Mut<'c, T>>
    where
        'a: 'c,
    {
        <[T; N]>::element_mut::<Mut>(self.reborrow_mut(), index.try_into().expect("u32 overflow"))
    }
    pub fn elements_mut<'c>(&'c mut self) -> [Val<'a, Mut::Mut<'c, T>>; N] {
        <[T; N]>::elements_mut::<Mut>(self.reborrow_mut())
    }
}
