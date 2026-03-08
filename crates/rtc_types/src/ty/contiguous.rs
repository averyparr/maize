use inkwell::values::BasicValue;

use crate::{
    ty::{Addrspace, M, R, V, ValTy, vec::VectorizableTy},
    val::Val,
};

pub enum HowToExtractElements {
    Vector,
    ScalableVector,
    Array,
    /// Unusual, but can happen -- e.g. {i32, i32, i32, i32}
    Struct,
}

pub unsafe trait UniformTy: ValTy {
    type ElemT: ValTy;
    const EXTRACTION_METHOD: HowToExtractElements;
}

pub unsafe trait ContiguousUniformTy<const SIZE: usize>: UniformTy {
    const SIZE: usize = SIZE;
    fn elements(val: Val<'_, Self>) -> [Val<'_, Self::ElemT>; SIZE]
    where
        Self: Sized,
    {
        ::core::array::from_fn(|index| {
            let index = u32::try_from(index).expect("u32 overflowed usize");
            let element = val.cx().extract_elem::<Self::ElemT, Self>(&val, index);
            unsafe { Val::new(val.cx(), element.as_basic_value_enum()) }
        })
    }
    fn copy_elements<'a>(val: &Val<'a, Self>) -> [Val<'a, Self::ElemT>; SIZE]
    where
        Self::ElemT: Copy,
        Self: Sized,
    {
        ::core::array::from_fn(|index| {
            let index = u32::try_from(index).expect("u32 overflowed usize");
            let element = val.cx().extract_elem::<Self::ElemT, Self>(&val, index);
            unsafe { Val::new(val.cx(), element.as_basic_value_enum()) }
        })
    }
    fn from_elements(values: [Val<'_, Self::ElemT>; SIZE]) -> Val<'_, Self>
    where
        Self: Sized,
    {
        let cx = values[0].cx();
        let mut val = unsafe { Val::new_undef(cx) };
        for (index, scalar) in values.into_iter().enumerate() {
            let index = u32::try_from(index).expect("u32 overflowed usize");
            val = cx.insert_elem::<Self::ElemT, Self>(val, scalar, index);
        }
        val
    }

    fn elements_ref<'a, 'b, Space: Addrspace>(
        ptr: Val<'a, R<&'b Self, Space>>,
    ) -> [Val<'a, R<&'b Self::ElemT, Space>>; SIZE] {
        let cx = ptr.cx();
        ::core::array::from_fn(|index| {
            let index = u32::try_from(index).expect("u32 overflow");
            let ptr = cx.get_elem_ptr::<Self::ElemT, _, _>(&ptr.as_ptr(), index);
            unsafe { Val::new(cx, ptr.as_basic_value_enum()) }
        })
    }

    fn elements_mut<'a, 'b, Space: Addrspace>(
        ptr: Val<'a, M<&'b mut Self, Space>>,
    ) -> [Val<'a, M<&'b mut Self::ElemT, Space>>; SIZE] {
        let cx = ptr.cx();
        ::core::array::from_fn(|index| {
            let index = u32::try_from(index).expect("u32 overflow");
            let ptr = cx.get_elem_ptr::<Self::ElemT, _, _>(&ptr.as_ptr(), index);
            unsafe { Val::new(cx, ptr.as_basic_value_enum()) }
        })
    }

    fn element<'a>(val: Val<'a, Self>, index: usize) -> Val<'a, Self::ElemT>
    where
        Self: Sized,
    {
        let index = u32::try_from(index).expect("u32 overflowed usize");
        let element = val.cx().extract_elem::<Self::ElemT, Self>(&val, index);
        unsafe { Val::new(val.cx(), element.as_basic_value_enum()) }
    }
    fn element_ref<'a, 'b, Space: Addrspace>(
        ptr: Val<'a, R<&'b Self, Space>>,
        index: u32,
    ) -> Val<'a, R<&'b Self::ElemT>> {
        assert!((index as usize) < Self::SIZE);
        let cx = ptr.cx();
        let ptr = cx.get_elem_ptr::<Self::ElemT, _, _>(&ptr.as_ptr(), index);
        unsafe { Val::new(cx, ptr.as_basic_value_enum()) }
    }

    fn element_mut<'a, 'b, Space: Addrspace>(
        ptr: Val<'a, M<&'b mut Self, Space>>,
        index: u32,
    ) -> Val<'a, M<&'b mut Self::ElemT, Space>> {
        assert!((index as usize) < Self::SIZE);
        let cx = ptr.cx();
        let ptr = cx.get_elem_ptr::<Self::ElemT, _, _>(&ptr.as_ptr(), index);
        unsafe { Val::new(cx, ptr.as_basic_value_enum()) }
    }

    fn splat<'a>(val: Val<'a, Self::ElemT>) -> Val<'a, Self>
    where
        Self::ElemT: Copy,
        Self: Sized,
    {
        Self::from_elements(::core::array::from_fn(|_| val))
    }
}

unsafe impl<T, const N: usize> UniformTy for V<T, N>
where
    T: VectorizableTy,
{
    type ElemT = T;
    const EXTRACTION_METHOD: HowToExtractElements = HowToExtractElements::Vector;
}
unsafe impl<T, const N: usize> UniformTy for [T; N]
where
    T: ValTy,
{
    type ElemT = T;
    const EXTRACTION_METHOD: HowToExtractElements = HowToExtractElements::Array;
}

unsafe impl<T, const N: usize> ContiguousUniformTy<N> for V<T, N> where T: VectorizableTy {}

unsafe impl<T, const N: usize> ContiguousUniformTy<N> for [T; N] where T: ValTy {}
