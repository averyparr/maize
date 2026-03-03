use crate::{
    ty::{MutTy, RefTy, SizedTy, raw::*},
    val::Val,
};

#[macro_export]
macro_rules! struct_reflect {
    (impl_traits: $name: ident$(<$($generics: ident),*>)? => ($mod: ident, $realized: ident, $accessor: ident, $accessor_ref: ident, $accessor_mut: ident) | $($field_type: ty),*) => {
        impl$(<$($generics: StructReflect),*>)? $crate::ty::AnyTy for $name$(<$($generics),*>)? {
            type AnyType<'ctx> = ::inkwell::types::StructType<'ctx>;
            fn any_ty<'ctx>(ctx: ::inkwell::context::ContextRef<'ctx>) -> Self::AnyType<'ctx> {
                ctx.struct_type(&[$(
                    inkwell::types::BasicType::as_basic_type_enum(&<$field_type as $crate::ty::Ty>::ty(ctx)).into()
                ),*], false)
            }
        }
        impl$(<$($generics: StructReflect),*>)? $crate::ty::ValTy for $name$(<$($generics),*>)? {
            type Value<'ctx> = ::inkwell::values::StructValue<'ctx>;

            fn undef<'ctx>(ctx: ::inkwell::context::ContextRef<'ctx>) -> Self::Value<'ctx> {
                <Self as $crate::ty::Ty>::ty(ctx).get_undef()
            }

            fn zeros<'ctx>(ctx: ::inkwell::context::ContextRef<'ctx>) -> Self::Value<'ctx> {
                <Self as $crate::ty::Ty>::ty(ctx).const_zero()
            }

            fn try_type_val<'ctx>(val: ::inkwell::values::AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
                if let ::inkwell::values::AnyValueEnum::StructValue(val) = val {
                    Some(val)
                } else {
                    None
                }
            }
        }
        impl$(<$($generics: StructReflect),*>)? $crate::ty::AlignedTy for $name$(<$($generics),*>)? {
            const ALIGN: u32 = ::std::mem::align_of::<<Self as $crate::ty::StructReflectTy>::RealStruct>() as _;
        }

        impl$(<$($generics: StructReflect),*>)? $crate::ty::SizedTy for $name$(<$($generics),*>)? {
            const SIZE: u32 = ::std::mem::size_of::<<Self as $crate::ty::StructReflectTy>::RealStruct>() as _;
        }
        impl$(<$($generics: StructReflect),*>)? $crate::ty::StructReflectTy for $name$(<$($generics),*>)? {
            type RealStruct = $mod::$realized$(<$($generics),*>)?;
        }
        impl$(<$($generics: StructReflect),*>)? $crate::ty::AccessibleStructTy for $name$(<$($generics),*>)? {
            type Accessor<'a> = $mod::$accessor::<'a, $($generics),*>;
            type AccessorRef<'a, 'b, Ref: $crate::ty::RefTy + 'b> = $mod::$accessor_ref<'a, 'b, Ref>
            where
                'a: 'b;
            type AccessorMut<'a, 'b, Mut: $crate::ty::MutTy + 'b> = $mod::$accessor_mut<'a, 'b, Mut>
            where
                'a: 'b;
            fn into_accessor(val: $crate::val::Val<'_, Self>) -> Self::Accessor<'_> {
                $mod::$accessor::new(val)
            }
            fn accessor<'a, 'b, Ref: $crate::ty::RefTy<PointeeTy = Self>>(
                val: $crate::val::Val<'a, Ref::Ref<'b, Self>>,
            ) -> Self::AccessorRef<'a, 'b, Ref>
            where
                'a: 'b{
                $mod::$accessor_ref::new(val)
            }
            fn accessor_mut<'a, 'b, Mut: $crate::ty::MutTy<PointeeTy = Self>>(
                val: $crate::val::Val<'a, Mut::Mut<'b, Self>>,
            ) -> Self::AccessorMut<'a, 'b, Mut>
            where
                'a: 'b{
                $mod::$accessor_mut::new(val)
            }

        }
    };
    (
        $(#[$($m:meta),*$(,)?])?
        $svis: vis struct $name: ident$(<$($generics: ident),*$(,)?>)?(
            $($vis: vis $field_type: ty),* $(,)?
        ) => $namespace: ident
    ) => {
        $(#[$($m),*])?
        $svis struct $name$(<$($generics: StructReflect),*>)?($($vis $field_type),*);
        pub mod $namespace {
            use super::*;
            #[repr(C)]
            $(#[$($m),*])?
            pub struct Realized$(<$($generics: StructReflect),*>)?($($vis <$field_type as $crate::ty::StructReflectTy>::RealStruct),*);
            pub struct Accessor<'a, $($generics),*>($(
                $vis $crate::val::Val<'a, $field_type>
            ),*);
            pub struct AccessorRef<'a, 'b, Ref: 'b +  $crate::ty::RefTy>($(
                $vis $crate::val::Val<'a, Ref::Ref<'b, $field_type>>
            ),*);
            pub struct AccessorMut<'a, 'b, Mut: 'b +  $crate::ty::MutTy>($(
                $vis $crate::val::Val<'a, Mut::Mut<'b, $field_type>>
            ),*);
            impl<'a, $($($generics: StructReflect),*)?> Accessor<'a, $($generics: StructReflect),*> {
                pub(super) fn new(val: $crate::val::Val<'a, $name$(<$($generics: StructReflect),*>)?>) -> Self {
                    type Full = $name$(<$($generics),*>)?;
                    let raw_ty = <Full as $crate::ty::Ty>::ty(val.ctx());
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    unsafe {
                        Self(
                            $(
                                cx.extract_struct_ptr_val::<$field_type, _>(&val, iter.next().expect("range failure!")),
                            )*
                        )
                    }
                }
            }
            impl<'a, 'b, Ref: 'b + $crate::ty::RefTy> AccessorRef<'a, 'b, Ref> {
                pub(super) fn new(val: $crate::val::Val<'a, Ref::Ref<'b, $name$(<$($generics: StructReflect),*>)?>>) -> Self {
                    type Full = $name$(<$($generics: StructReflect),*>)?;
                    let raw_ty = <Full as $crate::ty::Ty>::ty(val.ctx());
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    let ptr_to_struct = val.get_ll_typed();
                    let raw_struct = unsafe {$crate::val::Val::new(cx, ptr_to_struct)};
                    unsafe {
                        Self(
                            $(
                                $crate::val::Val::new_from_value(cx, ::inkwell::values::BasicValue::as_basic_value_enum(&cx.extract_struct_ptr_val::<$field_type, Full>(&raw_struct, iter.next().expect("range failure!")).raw_ptr())),
                            )*
                        )
                    }
                }
            }
            impl<'a, 'b, Mut: 'b + $crate::ty::MutTy> AccessorMut<'a, 'b, Mut> {
                pub(super) fn new(val: $crate::val::Val<'a, Mut::Mut<'b, $name$(<$($generics: StructReflect),*>)?>>) -> Self {
                    type Full = $name$(<$($generics: StructReflect),*>)?;
                    let raw_ty = <Full as $crate::ty::Ty>::ty(val.ctx());
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    let ptr_to_struct = val.get_ll_typed();
                    let raw_struct = unsafe {$crate::val::Val::new(cx, ptr_to_struct)};
                    unsafe {
                        Self(
                            $(
                                $crate::val::Val::new_from_value(cx, ::inkwell::values::BasicValue::as_basic_value_enum(&cx.extract_struct_ptr_val::<$field_type, Full>(&raw_struct, iter.next().expect("range failure!")).raw_ptr())),
                            )*
                        )
                    }
                }
            }
        }
        struct_reflect!(impl_traits: $name$(<$($generics),*>)? => ($namespace, Realized, Accessor, AccessorRef, AccessorMut) | $($field_type),*);
    };
    (
        $(#[$($m:meta),*$(,)?])?
        $svis: vis struct $name: ident$(<$($generics: ident),*$(,)?>)?{
            $($vis: vis $field_name: ident: $field_type: ty),*$(,)?
        } => $namespace: ident
    ) => {
        $(#[$($m),*])?
        $svis struct $name$(<$($generics: StructReflect),*>)? {
            $(
                $vis $field_name: $field_type
            ),*
        }

        mod $namespace {
            use super::*;
            #[repr(C)]
            $(#[$($m),*])?
            pub struct Realized$(<$($generics: StructReflect),*>)? {
                $(
                    $vis $field_name: <$field_type as $crate::ty::StructReflectTy>::RealStruct
                ),*
            }
            pub struct Accessor<'a, $($generics),*> {
                $(
                    $vis $field_name: $crate::val::Val<'a, $field_type>
                ),*
            }
            pub struct AccessorRef<'a, 'b, Ref: 'b +  $crate::ty::RefTy>{
                $(
                    $vis $field_name: $crate::val::Val<'a, Ref::Ref<'b, $field_type>>
                ),*
            }
            pub struct AccessorMut<'a, 'b, Mut: 'b +  $crate::ty::MutTy>{
                $(
                    $vis $field_name: $crate::val::Val<'a, Mut::Mut<'b, $field_type>>
                ),*
            }

            impl<'a, $($generics),*> Accessor<'a, $($generics),*> {
                pub(super) fn new(val: $crate::val::Val<'a, $name$(<$($generics),*>)?>) -> Self {
                    type Full = $name$(<$($generics),*>)?;
                    let raw_ty = <Full as $crate::ty::Ty>::ty(val.ctx());
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    unsafe {
                        Self {
                            $(
                                $field_name: cx.extract_struct_ptr_val::<$field_type, _>(&val, iter.next().expect("range failure!"))
                            ),*
                        }
                    }
                }
            }

            impl<'a, 'b, Ref: 'b + $crate::ty::RefTy> AccessorRef<'a, 'b, Ref> {
                pub(super) fn new(val: $crate::val::Val<'a, Ref::Ref<'b, $name$(<$($generics: StructReflect),*>)?>>) -> Self {
                    type Full = $name$(<$($generics: StructReflect),*>)?;
                    let raw_ty = <Full as $crate::ty::Ty>::ty(val.ctx());
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    let ptr_to_struct = val.get_ll_typed();
                    let raw_struct = unsafe {$crate::val::Val::new(cx, ptr_to_struct)};
                    unsafe {
                        Self{
                            $(
                                $field_name: $crate::val::Val::new_from_value(cx, ::inkwell::values::BasicValue::as_basic_value_enum(&cx.extract_struct_ptr_val::<$field_type, Full>(&raw_struct, iter.next().expect("range failure!")).raw_ptr())),
                            )*
                        }
                    }
                }
            }
            impl<'a, 'b, Mut: 'b + $crate::ty::MutTy> AccessorMut<'a, 'b, Mut> {
                pub(super) fn new(val: $crate::val::Val<'a, Mut::Mut<'b, $name$(<$($generics: StructReflect),*>)?>>) -> Self {
                    type Full = $name$(<$($generics: StructReflect),*>)?;
                    let raw_ty = <Full as $crate::ty::Ty>::ty(val.ctx());
                    let num_fields = raw_ty.count_fields();
                    let mut iter = 0..num_fields;
                    let cx = val.cx();
                    let ptr_to_struct = val.get_ll_typed();
                    let raw_struct = unsafe {$crate::val::Val::new(cx, ptr_to_struct)};
                    unsafe {
                        Self{
                            $(
                                $field_name: $crate::val::Val::new_from_value(cx, ::inkwell::values::BasicValue::as_basic_value_enum(&cx.extract_struct_ptr_val::<$field_type, Full>(&raw_struct, iter.next().expect("range failure!")).raw_ptr())),
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

pub trait AccessibleStructTy: StructReflectTy {
    type Accessor<'a>;
    type AccessorRef<'a, 'b, Ref: RefTy + 'b>
    where
        'a: 'b,
        Self: 'b;
    type AccessorMut<'a, 'b, Mut: MutTy + 'b>
    where
        'a: 'b,
        Self: 'b;
    fn into_accessor(val: Val<'_, Self>) -> Self::Accessor<'_>;
    fn accessor<'a, 'b, Ref: RefTy<PointeeTy = Self>>(
        val: Val<'a, Ref::Ref<'b, Self>>,
    ) -> Self::AccessorRef<'a, 'b, Ref>
    where
        'a: 'b;
    fn accessor_mut<'a, 'b, Mut: MutTy<PointeeTy = Self>>(
        val: Val<'a, Mut::Mut<'b, Self>>,
    ) -> Self::AccessorMut<'a, 'b, Mut>
    where
        'a: 'b;
}

impl<'a, StructT: AccessibleStructTy> Val<'a, StructT> {
    pub fn into_accessor(self) -> StructT::Accessor<'a> {
        StructT::into_accessor(self)
    }
}

impl<'a, StructT: AccessibleStructTy, Ref> Val<'a, Ref>
where
    Ref: RefTy<PointeeTy = StructT>,
{
    pub fn accessor<'b>(&'b self) -> StructT::AccessorRef<'a, 'b, Ref> {
        StructT::accessor(self.reborrow())
    }
}

impl<'a, StructT: AccessibleStructTy, Mut> Val<'a, Mut>
where
    Mut: MutTy<PointeeTy = StructT>,
{
    pub fn accessor_mut<'b>(&'b mut self) -> StructT::AccessorMut<'a, 'b, Mut> {
        StructT::accessor_mut(self.reborrow_mut())
    }
}
