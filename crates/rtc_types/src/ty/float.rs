use inkwell::{context::ContextRef, values::AnyValueEnum};

use super::raw::float::*;
use super::{AnyTy, Ty, ValTy};

macro_rules! float_ty_impl {
    ($($tipes: ident => $instance_fn: ident),*$(,)?) => {$(
        impl AnyTy for $tipes {
            type AnyType<'ctx> = ::inkwell::types::FloatType<'ctx>;
            fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
                ctx.$instance_fn()
            }
        }
        impl ValTy for $tipes {
            type Value<'ctx> = ::inkwell::values::FloatValue<'ctx>;
            fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
                ctx.$instance_fn().get_undef()
            }
            fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
                ctx.$instance_fn().const_zero()
            }
            fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
                if let AnyValueEnum::FloatValue(val) = val {
                    Some(val)
                } else {
                    None
                }
            }
        }
    )*};
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub(crate) struct RawF16(u16);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub(crate) struct RawF128(u128);
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub(crate) struct RawBF16(u16);

float_ty_impl!(
    F16 => f16_type,
    F32 => f32_type,
    F64 => f64_type,
    F128 => f128_type,
    BF16 => bf16_type,
);

impl AnyTy for E4M3 {
    type AnyType<'ctx> = inkwell::types::IntType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        ctx.i8_type()
    }
}
impl AnyTy for E5M2 {
    type AnyType<'ctx> = inkwell::types::IntType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        ctx.i8_type()
    }
}
impl ValTy for E4M3 {
    type Value<'ctx> = inkwell::values::IntValue<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).get_undef()
    }

    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        // This is true of the FP8 types
        Self::ty(ctx).const_int(0, false)
    }

    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        if let AnyValueEnum::IntValue(val) = val {
            Some(val)
        } else {
            None
        }
    }
}
impl ValTy for E5M2 {
    type Value<'ctx> = inkwell::values::IntValue<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).get_undef()
    }

    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        // This is true of the FP8 types
        Self::ty(ctx).const_int(0, false)
    }

    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        if let AnyValueEnum::IntValue(val) = val {
            Some(val)
        } else {
            None
        }
    }
}
