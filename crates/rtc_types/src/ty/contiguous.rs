use inkwell::{
    builder::Builder,
    context::ContextRef,
    types::PointerType,
    values::{
        AggregateValue, AggregateValueEnum, AnyValue, AnyValueEnum, BasicValue, BasicValueEnum,
        PointerValue, VectorBaseValue,
    },
};

use crate::{
    ty::{M, MutTy, R, RefTy, Ty, V, ValTy, vec::VectorizableTy},
    val::Val,
};

pub enum ContiguousTypeVariety {
    Vector,
    Array,
}

pub unsafe trait ConstSizeContiguousTy<const SIZE: usize>: ValTy {
    type ElemT: ValTy;
    const VARIETY: ContiguousTypeVariety;

    fn insert_value(
        context: ContextRef<'static>,
        builder: Builder<'static>,
        val: Self::Value<'static>,
        elem: <Self::ElemT as ValTy>::Value<'static>,
        index: u32,
    ) -> Self::Value<'static> {
        let untyped_val = val.as_basic_value_enum();
        let raw = match Self::VARIETY {
            ContiguousTypeVariety::Vector => match untyped_val {
                BasicValueEnum::VectorValue(val) => builder
                    .build_insert_element(
                        val,
                        elem,
                        context.i32_type().const_int(index as _, false),
                        "scalable_vec_insert",
                    )
                    .expect("Insert vector element should have worked")
                    .as_basic_value_enum(),
                BasicValueEnum::ScalableVectorValue(val) => builder
                    .build_insert_element(
                        val,
                        elem,
                        context.i32_type().const_int(index as _, false),
                        "scalable_vec_insert",
                    )
                    .expect("Insert vector element should have worked")
                    .as_basic_value_enum(),
                _ => {
                    panic!("Asked for [scalable] vector value but passed type {untyped_val:?}");
                }
            },
            ContiguousTypeVariety::Array => match untyped_val {
                BasicValueEnum::ArrayValue(val) => builder
                    .build_insert_value(val, elem, index, "array_insert")
                    .expect("Array insert should have suceeded")
                    .as_basic_value_enum(),
                BasicValueEnum::StructValue(val) => builder
                    .build_insert_value(val, elem, index, "array_insert")
                    .expect("Array insert should have suceeded")
                    .as_basic_value_enum(),
                _ => panic!("Attempted to insert an element into {untyped_val:?}"),
            },
        };
        Self::type_val(raw.as_any_value_enum())
    }
    fn extract_value(
        context: ContextRef<'static>,
        builder: Builder<'static>,
        val: &Self::Value<'static>,
        index: u32,
    ) -> <Self::ElemT as ValTy>::Value<'static> {
        let untyped_val = val.as_basic_value_enum();
        let raw = match Self::VARIETY {
            ContiguousTypeVariety::Vector => match untyped_val {
                BasicValueEnum::VectorValue(val) => builder
                    .build_extract_element(
                        val,
                        context.i32_type().const_int(index as _, false),
                        "vec_extract",
                    )
                    .expect("Extract element should have worked")
                    .as_basic_value_enum(),
                BasicValueEnum::ScalableVectorValue(val) => builder
                    .build_extract_element(
                        val,
                        context.i32_type().const_int(index as _, false),
                        "scalable_vec_extract",
                    )
                    .expect("Extract element should have worked")
                    .as_basic_value_enum(),
                _ => panic!("Asked for [scalable] vector value but passed type {untyped_val:?}"),
            },
            ContiguousTypeVariety::Array => match untyped_val {
                BasicValueEnum::ArrayValue(val) => builder
                    .build_extract_value(val, index, "array_extract")
                    .expect("Array insert should have suceeded")
                    .as_basic_value_enum(),
                BasicValueEnum::StructValue(val) => builder
                    .build_extract_value(val, index, "struct_extract")
                    .expect("Array insert should have suceeded")
                    .as_basic_value_enum(),
                _ => panic!("Attempted to insert an element into {untyped_val:?}"),
            },
        };

        Self::ElemT::type_val(raw.as_any_value_enum())
    }
    fn gep(
        context: ContextRef<'static>,
        builder: Builder<'static>,
        ptr_to_self: PointerValue<'static>,
        index: u32,
    ) -> PointerValue<'static> {
        unsafe {
            builder
                .build_gep(
                    Self::ElemT::ty(context),
                    ptr_to_self,
                    &[context.i32_type().const_int(index as _, false)],
                    "contig_gep",
                )
                .expect("GEP should work")
        }
    }

    const SIZE: usize = SIZE;
    fn elements(val: Val<'_, Self>) -> [Val<'_, Self::ElemT>; SIZE] {
        let raw = val.get_ll_typed();

        ::core::array::from_fn(|index| {
            let element = unsafe {
                val.cx().with_builder(|b| {
                    Self::extract_value(
                        val.ctx(),
                        b,
                        &raw,
                        u32::try_from(index).expect("u32 overflowed usize"),
                    )
                })
            };
            unsafe { Val::new_from_value(val.cx(), element.as_basic_value_enum()) }
        })
    }
    fn from_elements(values: [Val<'_, Self::ElemT>; SIZE]) -> Val<'_, Self> {
        let cx = values[0].cx();
        let mut val = Self::undef(cx.ctx());
        for (i, scalar) in values.iter().enumerate() {
            val = unsafe {
                cx.with_builder(|b| {
                    Self::insert_value(
                        cx.ctx(),
                        b,
                        val,
                        scalar.get_ll_typed(),
                        u32::try_from(i).expect("u32 overflow"),
                    )
                })
            };
        }

        unsafe { Val::new_from_value(cx, val.as_basic_value_enum()) }
    }

    fn elements_ref<'a, 'b, Ref>(
        val: Val<'a, Ref::Ref<'b, Ref::PointeeTy>>,
    ) -> [Val<'a, Ref::Ref<'b, Self::ElemT>>; SIZE]
    where
        Ref: RefTy<PointeeTy = Self>,
    {
        let raw_ptr = val.get_ll_typed();
        ::core::array::from_fn(|index| {
            let ptr = unsafe {
                val.cx().with_builder(|b| {
                    Self::gep(
                        val.ctx(),
                        b,
                        raw_ptr,
                        u32::try_from(index).expect("u32 overflow"),
                    )
                })
            };
            unsafe { Val::new_from_value(val.cx(), ptr.as_basic_value_enum()) }
        })
    }

    fn elements_mut<'a, 'b, Mut>(
        val: Val<'a, Mut::Mut<'b, Mut::PointeeTy>>,
    ) -> [Val<'a, Mut::Mut<'b, Self::ElemT>>; SIZE]
    where
        Mut: MutTy<PointeeTy = Self>,
    {
        let raw_ptr = val.get_ll_typed();
        ::core::array::from_fn(|index| {
            let ptr = unsafe {
                val.cx().with_builder(|b| {
                    Self::gep(
                        val.ctx(),
                        b,
                        raw_ptr,
                        u32::try_from(index).expect("u32 overflow"),
                    )
                })
            };
            unsafe { Val::new_from_value(val.cx(), ptr.as_basic_value_enum()) }
        })
    }

    fn element_ref<'a, 'b, Ref>(
        val: Val<'a, Ref::Ref<'b, Ref::PointeeTy>>,
        index: u32,
    ) -> Val<'a, R<&'b Self::ElemT>>
    where
        Ref: RefTy<PointeeTy = Self> + 'b,
    {
        assert!((index as usize) < Self::SIZE);
        let raw_ptr = val.get_ll_typed();
        let ptr = unsafe {
            val.cx()
                .with_builder(|b| Self::gep(val.ctx(), b, raw_ptr, index))
        };
        unsafe { Val::new_from_value(val.cx(), ptr.as_basic_value_enum()) }
    }

    fn element_mut<'a, 'b, Mut>(
        val: Val<'a, Mut::Mut<'b, Mut::PointeeTy>>,
        index: u32,
    ) -> Val<'a, Mut::Mut<'b, Self::ElemT>>
    where
        Mut: MutTy<PointeeTy = Self> + 'b,
    {
        assert!((index as usize) < Self::SIZE);
        let raw_ptr = val.get_ll_typed();
        let ptr = unsafe {
            val.cx()
                .with_builder(|b| Self::gep(val.ctx(), b, raw_ptr, index))
        };
        unsafe { Val::new_from_value(val.cx(), ptr.as_basic_value_enum()) }
    }

    fn splat<'a>(val: Val<'a, Self::ElemT>) -> Val<'a, Self>
    where
        Self::ElemT: Copy,
    {
        Self::from_elements(::core::array::from_fn(|_| val.copy()))
    }
}

unsafe impl<T, const N: usize> ConstSizeContiguousTy<N> for V<T, N>
where
    T: VectorizableTy,
{
    type ElemT = T;

    const VARIETY: ContiguousTypeVariety = ContiguousTypeVariety::Vector;
}

unsafe impl<T, const N: usize> ConstSizeContiguousTy<N> for [T; N]
where
    T: ValTy,
{
    type ElemT = T;

    const VARIETY: ContiguousTypeVariety = ContiguousTypeVariety::Array;
}
