use inkwell::{
    builder::Builder,
    types::{AnyType, AnyTypeEnum},
    values::{BasicValue, BasicValueEnum, FloatValue, IntValue},
};

use crate::{
    ty::{Ty, V, ValTy, vec::VectorizableTy},
    val::Val,
};

pub trait ConvertibleTy<To: ValTy>: ValTy {
    fn cvt(val: Val<'_, Self>) -> Val<'_, To>;
}

trait ScalarConvertible {
    fn cast_to(self, b: Builder<'static>, dst_ty: AnyTypeEnum<'static>) -> BasicValueEnum<'static>;
}

impl ScalarConvertible for FloatValue<'static> {
    fn cast_to(self, b: Builder<'static>, dst_ty: AnyTypeEnum<'static>) -> BasicValueEnum<'static> {
        b.build_float_cast(self, dst_ty.into_float_type(), "fcvt")
            .expect("Float cast should always succeed")
            .as_basic_value_enum()
    }
}
impl ScalarConvertible for IntValue<'static> {
    fn cast_to(self, b: Builder<'static>, dst_ty: AnyTypeEnum<'static>) -> BasicValueEnum<'static> {
        b.build_int_cast(self, dst_ty.into_int_type(), "icvt")
            .expect("Int cast should always succeed")
            .as_basic_value_enum()
    }
}

impl<From, To> ConvertibleTy<To> for From
where
    for<'a> To: ValTy<Type<'a> = From::Type<'a>, Value<'a> = From::Value<'a>>,
    for<'a> From: ValTy<Value<'static>: ScalarConvertible>,
{
    fn cvt(val: Val<'_, Self>) -> Val<'_, To> {
        let raw = val.ll_typed();
        let dst_ty = To::ty(val.ctx());
        let raw_cast = unsafe {
            val.cx()
                .with_builder(|b| raw.cast_to(b, dst_ty.as_any_type_enum()))
        };
        unsafe { Val::new(val.cx(), raw_cast) }
    }
}

impl<From: VectorizableTy, To: VectorizableTy, const N: usize> ConvertibleTy<V<To, N>>
    for V<From, N>
{
    fn cvt(val: Val<'_, Self>) -> Val<'_, V<To, N>> {
        let raw = val.ll_typed();
        let src_ty = raw.get_type();
        let dst_ty = V::<To, N>::ty(val.ctx());
        let element_type = src_ty.get_element_type();
        let raw_cast = if element_type.is_int_type() {
            unsafe {
                val.cx()
                    .with_builder(|b| b.build_int_cast(raw, dst_ty, "icast"))
            }
            .expect("int cast should succeed")
        } else if element_type.is_float_type() {
            unsafe {
                val.cx()
                    .with_builder(|b| b.build_float_cast(raw, dst_ty, "fcast"))
            }
            .expect("float cast should succeed")
        } else {
            panic!("Attempted to cast between {src_ty:?} -> {dst_ty:?}");
        };

        unsafe { Val::new(val.cx(), raw_cast.as_basic_value_enum()) }
    }
}

impl<'a, From> Val<'a, From>
where
    From: ValTy,
{
    pub fn cast<To>(self) -> Val<'a, To>
    where
        From: ConvertibleTy<To>,
        To: ValTy,
    {
        From::cvt(self)
    }
}

impl<'a, From, const N: usize> Val<'a, V<From, N>>
where
    From: VectorizableTy,
{
    pub fn vec_cast<To>(self) -> Val<'a, V<To, N>>
    where
        V<From, N>: ConvertibleTy<V<To, N>>,
        To: VectorizableTy,
    {
        V::<From, N>::cvt(self)
    }
}
