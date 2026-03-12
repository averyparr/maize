use inkwell::{builder::Builder, values::BasicValue};

use crate::{
    ty::{Ty, V, ValTy, raw::*, vec::VectorizableTy},
    val::Val,
};

pub trait ConvertibleTy<To: ValTy>: ValTy {
    fn cvt(val: Val<'_, Self>) -> Val<'_, To>;
}

trait ScalarConvertibleTy: ValTy {
    fn cast_to(
        val: Self::Value<'static>,
        b: Builder<'static>,
        dst_ty: Self::Type<'static>,
    ) -> Self::Value<'static>;
}

macro_rules! impl_scalar_convertible {
    (float: $($tipes: ty),*) => {
        $(
            impl ScalarConvertibleTy for $tipes {
                fn cast_to(
                    val: Self::Value<'static>,
                    b: Builder<'static>,
                    dst_ty: Self::Type<'static>,
                ) -> Self::Value<'static> {
                    b.build_float_cast(val, dst_ty, "fcast")
                        .expect("Float cast should succeed")
                }
            }
        )*
    };
    (sint: $($tipes: ty),*) => {
        $(
            impl ScalarConvertibleTy for $tipes {
                fn cast_to(
                    val: Self::Value<'static>,
                    b: Builder<'static>,
                    dst_ty: Self::Type<'static>,
                ) -> Self::Value<'static> {
                    b.build_int_cast_sign_flag(val, dst_ty, false, "sicast")
                        .expect("Int cast should succeed")
                }
            }
        )*
    };
    (uint: $($tipes: ty),*) => {
        $(
            impl ScalarConvertibleTy for $tipes {
                fn cast_to(
                    val: Self::Value<'static>,
                    b: Builder<'static>,
                    dst_ty: Self::Type<'static>,
                ) -> Self::Value<'static> {
                    b.build_int_cast_sign_flag(val, dst_ty, false, "uicast")
                        .expect("Unsigned int cast should succeed")
                }
            }
        )*
    };
}

impl_scalar_convertible!(float: F16, BF16, F32, F64);
impl_scalar_convertible!(sint: I8, I16, I32, I64, I128);
impl_scalar_convertible!(uint: U8, U16, U32, U64, U128);

impl<From, To> ConvertibleTy<To> for From
where
    for<'a> To: ScalarConvertibleTy<Type<'a> = From::Type<'a>, Value<'a> = From::Value<'a>>,
    for<'a> From: ScalarConvertibleTy,
{
    fn cvt(val: Val<'_, Self>) -> Val<'_, To> {
        let raw = val.ll_typed();
        let dst_ty = To::ty(val.ctx());
        let raw_cast = unsafe { val.cx().with_builder(|b| Self::cast_to(raw, b, dst_ty)) };

        unsafe { Val::new(val.cx(), raw_cast.as_basic_value_enum()) }
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
