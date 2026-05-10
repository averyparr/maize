use inkwell::{context::ContextRef, values::AnyValueEnum};

use super::raw::int::*;
use super::{AnyTy, ValTy};

macro_rules! int_ty_impl {
    ($($tipes: ident => $instance_fn: ident),*$(,)?) => {$(
        impl AnyTy for $tipes {
            type AnyType<'ctx> = ::inkwell::types::IntType<'ctx>;
            fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
                ctx.$instance_fn()
            }
        }
        impl ValTy for $tipes {
            type Value<'ctx> = ::inkwell::values::IntValue<'ctx>;
            fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
                ctx.$instance_fn().get_undef()
            }
            fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
                ctx.$instance_fn().const_zero()
            }
            fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
                if let AnyValueEnum::IntValue(val) = val {
                    Some(val)
                } else {
                    None
                }
            }
        }
    )*};
}

int_ty_impl!(
    I8 => i8_type,
    I16 => i16_type,
    I32 => i32_type,
    I64 => i64_type,
    I128 => i128_type,
    U8 => i8_type,
    U16 => i16_type,
    U32 => i32_type,
    U64 => i64_type,
    U128 => i128_type,
);
