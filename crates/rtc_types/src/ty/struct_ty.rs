use crate::{
    ty::{Addrspace, SizedTy, raw::*},
    val::Val,
};

#[macro_export]
macro_rules! _get_first {
    ($first: ident $(, $rest: ident)*) => {
        $first
    };
}

#[macro_export]
macro_rules! struct_reflect {
    (impl_traits: $name: ident$(<$($generics: ident$(: $bounds: tt)?),*>)? => ($mod: ident, $realized: ident, $accessor: ident, $accessor_ref: ident, $accessor_mut: ident) | $($field_type: ty),*) => {
        impl$(<$($generics: $crate::ty::ValTy $( + $bounds )?),*>)? $crate::ty::AnyTy for $name$(<$($generics),*>)? {
            type AnyType<'ctx> = $crate::inkwell::types::StructType<'ctx>;
            fn any_ty<'ctx>(ctx: $crate::inkwell::context::ContextRef<'ctx>) -> Self::AnyType<'ctx> {
                ctx.struct_type(&[$(
                    $crate::inkwell::types::BasicType::as_basic_type_enum(&<$field_type as $crate::ty::Ty>::ty(ctx)).into()
                ),*], false)
            }
        }
        impl$(<$($generics: $crate::ty::ValTy $( + $bounds )?),*>)? $crate::ty::ValTy for $name$(<$($generics),*>)? {
            type Value<'ctx> = $crate::inkwell::values::StructValue<'ctx>;

            fn undef<'ctx>(ctx: $crate::inkwell::context::ContextRef<'ctx>) -> Self::Value<'ctx> {
                <Self as $crate::ty::Ty>::ty(ctx).get_undef()
            }

            fn zeros<'ctx>(ctx: $crate::inkwell::context::ContextRef<'ctx>) -> Self::Value<'ctx> {
                <Self as $crate::ty::Ty>::ty(ctx).const_zero()
            }

            fn try_type_val<'ctx>(val: $crate::inkwell::values::AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
                if let $crate::inkwell::values::AnyValueEnum::StructValue(val) = val {
                    Some(val)
                } else {
                    None
                }
            }
        }
        impl$(<$($generics: $crate::ty::StructReflectTy $( + $bounds )?),*>)? $crate::ty::AlignedTy for $name$(<$($generics),*>)? {
            const ALIGN: u32 = ::std::mem::align_of::<<Self as $crate::ty::StructReflectTy>::RealStruct>() as _;
        }

        impl$(<$($generics: $crate::ty::StructReflectTy $( + $bounds )?),*>)? $crate::ty::SizedTy for $name$(<$($generics),*>)? {
            const SIZE: u32 = ::std::mem::size_of::<<Self as $crate::ty::StructReflectTy>::RealStruct>() as _;
        }
        impl$(<$($generics: $crate::ty::StructReflectTy $( + $bounds )?),*>)? $crate::ty::StructReflectTy for $name$(<$($generics),*>)? {
            type RealStruct = $mod::$realized$(<$($generics),*>)?;
        }
        impl$(<$($generics: $crate::ty::StructReflectTy $( + $bounds )?),*>)? $crate::ty::AccessibleStructTy for $name$(<$($generics),*>)? {
            type Accessor<'a> = $mod::$accessor::<'a $($(, $generics)*)?>;
            type AccessorRef<'a, 'b, Space: $crate::ty::Addrspace> = $mod::$accessor_ref<'a, 'b, Space $($(, $generics)*)?>
            where
                'a: 'b,
                Self: 'b;
            type AccessorMut<'a, 'b, Space: $crate::ty::Addrspace> = $mod::$accessor_mut<'a, 'b, Space $($(, $generics)*)?>
            where
                'a: 'b,
                Self: 'b;
            fn into_accessor(val: $crate::val::Val<'_, Self>) -> Self::Accessor<'_> {
                $mod::$accessor::new(val)
            }
            fn accessor<'a, 'b, Space: $crate::ty::Addrspace>(
                val: $crate::val::Val<'a, $crate::ty::R<&'b Self, Space>>,
            ) -> Self::AccessorRef<'a, 'b, Space>
            where
                'a: 'b
            {
                $mod::$accessor_ref::new(val)
            }
            fn accessor_mut<'a, 'b, Space: $crate::ty::Addrspace>(
                val: $crate::val::Val<'a, $crate::ty::M<&'b mut Self, Space>>,
            ) -> Self::AccessorMut<'a, 'b, Space>
            where
                'a: 'b
            {
                $mod::$accessor_mut::new(val)
            }
        }
    };
    (
        $(#[$($m:meta),*$(,)?])?
        $svis: vis struct $name: ident$(<$($generics: ident $(: $bounds: tt)?),*$(,)?>)?(
            $($vis: vis $field_type: ty),* $(,)?
        ) => $namespace: ident
    ) => {
        compile_error!("We only support structs like {} because they have names");
    };
    (
        $(#[$($m:meta),*$(,)?])*
        $svis: vis struct $name: ident$(<$($generics: ident $(: $bounds: tt)?),*$(,)?>)?{
            $($vis: vis $field_name: ident: $field_type: ty),*$(,)?
        } => $namespace: ident
    ) => {
        $(#[$($m),*])*
        $svis struct $name$(<$($generics $(: $bounds)?),*>)? {
            $(
                $vis $field_name: $field_type
            ),*
        }

        mod $namespace {
            use super::*;
            #[repr(C)]
            $(#[$($m),*])*
            pub struct Realized$(<$($generics: $crate::ty::StructReflectTy $(+ $bounds)?),*>)? {
                $(
                    $vis $field_name: <$field_type as $crate::ty::StructReflectTy>::RealStruct
                ),*
            }
            pub struct Accessor<'a, $($($generics $(: $bounds)?),*)?> {
                $(
                    $vis $field_name: $crate::val::Val<'a, $field_type>
                ),*
            }
            pub struct AccessorRef<'a, 'b, Space $($(, $generics: $crate::ty::ValTy + 'b $(+ $bounds)?)*)?>{
                $(
                    $vis $field_name: $crate::val::Val<'a, $crate::ty::R<&'b $field_type, Space>>
                ),*
            }
            pub struct AccessorMut<'a, 'b, Space $($(, $generics: $crate::ty::ValTy + 'b $(+ $bounds)?)*)?>{
                $(
                    $vis $field_name: $crate::val::Val<'a, $crate::ty::M<&'b mut $field_type, Space>>
                ),*
            }

            impl$(<$($generics: $crate::ty::ValTy),*>)? $name$(<$($generics),*>)? {
                pub fn from_fields<'a>($($field_name: $crate::val::Val<'a, $field_type>),*) -> $crate::val::Val<'a, Self> {
                    let num_iters = [$((1,&$field_name).0),*].into_iter().sum();
                    let mut iter = 0..num_iters;
                    let cx = $crate::_get_first!($($field_name),*).cx();
                    let mut ret = unsafe { $crate::val::Val::new_undef(cx) };
                    $(
                        ret = cx.insert_struct_field(
                            ret,
                            $field_name,
                            iter.next().expect("Range should work...")
                        );
                    )*

                    ret
                }
            }

            impl<'a, $($($generics: $crate::ty::ValTy),*)?> Accessor<'a, $($($generics),*)?> {
                pub(super) fn new(val: $crate::val::Val<'a, $name$(<$($generics),*>)?>) -> Self {
                    let raw_ty = $crate::val::__structreflect::_lltyped(&val).get_type();
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    unsafe {
                        Self {
                            $(
                                $field_name: $crate::val::__structreflect::_new(
                                    cx,
                                    $crate::inkwell::values::BasicValue::as_basic_value_enum(
                                        &cx.get_struct_field::<$field_type, _>(
                                            &val,
                                            iter.next().expect("range failure!")
                                        )
                                    )
                                ),
                            )*
                        }
                    }
                }
            }
            impl<'a, 'b, Space: $crate::ty::Addrspace $($(,$generics: $crate::ty::StructReflectTy)*)?> AccessorRef<'a, 'b, Space $($(, $generics)*)?>
            {
                pub(super) fn new(val: $crate::val::Val<'a, $crate::ty::R<&'b $name$(<$($generics),*>)?, Space>>) -> Self
                {
                    type Full$(<$($generics),*>)? = $name$(<$($generics),*>)?;
                    let raw_ty = <Full$(<$($generics),*>)? as $crate::ty::Ty>::ty($crate::val::__structreflect::_ctx(&val));
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    unsafe {
                        Self {
                            $(
                                $field_name: $crate::val::__structreflect::_new(
                                    cx,
                                    $crate::inkwell::values::BasicValue::as_basic_value_enum(
                                        &cx.get_struct_ptr::<$field_type, _, _>(
                                            val.as_ptr(),
                                            iter.next().expect("range failure!")
                                        )
                                    )
                                ),
                            )*
                        }
                    }
                }
            }
            impl<'a, 'b, Space: $crate::ty::Addrspace $($(,$generics: $crate::ty::StructReflectTy)*)?> AccessorMut<'a, 'b, Space $($(, $generics)*)?>
            {
                pub(super) fn new(val: $crate::val::Val<'a, $crate::ty::M<&'b mut $name$(<$($generics),*>)?, Space>>) -> Self
                {
                    type Full$(<$($generics),*>)? = $name$(<$($generics),*>)?;
                    let raw_ty = <Full$(<$($generics),*>)? as $crate::ty::Ty>::ty($crate::val::__structreflect::_ctx(&val));
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    unsafe {
                        Self {
                            $(
                                $field_name: $crate::val::__structreflect::_new(
                                    cx,
                                    $crate::inkwell::values::BasicValue::as_basic_value_enum(
                                        &cx.get_struct_ptr::<$field_type, _, _>(
                                            val.as_ptr(),
                                            iter.next().expect("range failure!")
                                        )
                                    )
                                ),
                            )*
                        }
                    }
                }
            }
        }
        struct_reflect!(impl_traits: $name$(<$($generics),*>)? => ($namespace, Realized, Accessor, AccessorRef, AccessorMut) | $($field_type),*);
    };
}

pub trait StructReflectTy: SizedTy {
    type RealStruct;
}

macro_rules! impl_reflect_for_primitive {
    ($($trace_ty: ty => $real_ty: ty;)*) => {$(
        impl StructReflectTy for $trace_ty {
            type RealStruct = $real_ty;
        }
    )*};
}

impl_reflect_for_primitive!(
    I8 => i8;
    I16 => i16;
    I32 => i32;
    I64 => i64;
    U8 => u8;
    U16 => u16;
    U32 => u32;
    U64 => u64;
    // Outliers
    F16 => u16;
    BF16 => u16;
    // Regular floats
    F32 => f32;
    F64 => f64;
);

#[expect(unused)]
pub struct V2F16(u32);

impl StructReflectTy for V<F16, 2> {
    type RealStruct = V2F16;
}

impl<T, Space: Addrspace> StructReflectTy for P<*const T, Space>
where
    T: StructReflectTy,
{
    type RealStruct = *const T::RealStruct;
}
impl<T, Space: Addrspace> StructReflectTy for P<*mut T, Space>
where
    T: StructReflectTy,
{
    type RealStruct = *mut T::RealStruct;
}
impl<'a, T, Space: Addrspace> StructReflectTy for R<&'a T, Space>
where
    T: StructReflectTy,
{
    type RealStruct = &'a T::RealStruct;
}
impl<'a, T, Space: Addrspace> StructReflectTy for M<&'a mut T, Space>
where
    T: StructReflectTy,
{
    type RealStruct = &'a T::RealStruct;
}

pub trait AccessibleStructTy: StructReflectTy {
    type Accessor<'a>;
    type AccessorRef<'a, 'b, Space: Addrspace>
    where
        'a: 'b,
        Self: 'b;
    type AccessorMut<'a, 'b, Space: Addrspace>
    where
        'a: 'b,
        Self: 'b;
    fn into_accessor(val: Val<'_, Self>) -> Self::Accessor<'_>;
    fn accessor<'a, 'b, Space: Addrspace>(
        val: Val<'a, R<&'b Self, Space>>,
    ) -> Self::AccessorRef<'a, 'b, Space>
    where
        'a: 'b;
    fn accessor_mut<'a, 'b, Space: Addrspace>(
        val: Val<'a, M<&'b mut Self, Space>>,
    ) -> Self::AccessorMut<'a, 'b, Space>
    where
        'a: 'b;
}

impl<'a, StructT: AccessibleStructTy> Val<'a, StructT> {
    pub fn into_accessor(self) -> StructT::Accessor<'a> {
        StructT::into_accessor(self)
    }
}

impl<'a, 'b, StructT: AccessibleStructTy, Space: Addrspace> Val<'a, R<&'b StructT, Space>> {
    pub fn accessor<'c>(&'c self) -> StructT::AccessorRef<'a, 'c, Space>
    where
        'b: 'c,
    {
        StructT::accessor(self.reborrow())
    }
}

impl<'a, 'b, StructT: AccessibleStructTy, Space: Addrspace> Val<'a, M<&'b mut StructT, Space>> {
    pub fn accessor_mut<'c>(&'c mut self) -> StructT::AccessorMut<'a, 'c, Space> {
        StructT::accessor_mut(self.reborrow_mut())
    }
}
