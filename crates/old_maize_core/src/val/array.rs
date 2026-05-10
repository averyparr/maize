use crate::{
    ty::{Addrspace, ContiguousUniformTy, FixedSizeContiguousUniformTy, M, R, SizedTy, U32, ValTy},
    val::Val,
};

impl<'a, T, const N: usize> Val<'a, [T; N]>
where
    T: SizedTy,
{
    pub fn array_elements(self) -> [Val<'a, T>; N] {
        <[T; N]>::elements(self)
    }

    pub fn copy_elements(&self) -> [Val<'a, T>; N]
    where
        T: Copy,
    {
        <[T; N]>::copy_elements(self)
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

impl<'a, 'b, Space: Addrspace, T, const N: usize> Val<'a, R<&'b [T; N], Space>>
where
    T: ValTy + 'a,
{
    pub fn index_ref<'c>(&'c self, index: usize) -> Val<'a, R<&'c T, Space>> {
        <[T; N]>::element_ref(self.reborrow(), index.try_into().expect("u32 overflow"))
    }
    pub fn runtime_index_ref<'c>(&'c self, index: Val<'a, U32>) -> Val<'a, R<&'c T, Space>> {
        <[T; N]>::runtime_element_ref(self.reborrow(), index)
    }
    pub fn elements_ref<'c>(&'c self) -> [Val<'a, R<&'c T, Space>>; N] {
        <[T; N]>::elements_ref(self.reborrow())
    }
}

impl<'a, 'b, Space: Addrspace, T, const N: usize> Val<'a, M<&'b mut [T; N], Space>>
where
    T: ValTy + 'a,
{
    pub fn index_ref<'c>(&'c self, index: usize) -> Val<'a, R<&'c T, Space>> {
        <[T; N]>::element_ref(self.reborrow(), index.try_into().expect("u32 overflow"))
    }
    pub fn index_mut<'c>(&'c mut self, index: usize) -> Val<'a, M<&'c mut T, Space>>
    where
        'b: 'c,
    {
        <[T; N]>::element_mut(self.reborrow_mut(), index.try_into().expect("u32 overflow"))
    }
    pub fn runtime_index_ref<'c>(&'c self, index: Val<'a, U32>) -> Val<'a, R<&'c T, Space>>
    where
        'b: 'c,
    {
        <[T; N]>::runtime_element_ref(self.reborrow(), index)
    }
    pub fn runtime_index_mut<'c>(&'c mut self, index: Val<'a, U32>) -> Val<'a, M<&'c mut T, Space>>
    where
        'b: 'c,
    {
        <[T; N]>::runtime_element_mut(self.reborrow_mut(), index)
    }
    pub fn elements_mut<'c>(&'c mut self) -> [Val<'a, M<&'c mut T, Space>>; N] {
        <[T; N]>::elements_mut(self.reborrow_mut())
    }
    pub fn elements_ref<'c>(&'c self) -> [Val<'a, R<&'c T, Space>>; N] {
        <[T; N]>::elements_ref(self.reborrow())
    }
}
