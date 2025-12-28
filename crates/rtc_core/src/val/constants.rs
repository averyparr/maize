use inkwell::{context::ContextRef, values::BasicValue};

use crate::{
    codegen::FnCodegen,
    ty::{Ty, primitive::*},
    val::Val,
};

pub struct C<T>(pub T);

impl<T> From<T> for C<T> {
    fn from(value: T) -> Self {
        C(value)
    }
}

pub trait AcceptsConstants: Sized {
    type Assoc;
    const ZERO: Self::Assoc;
    const ONE: Self::Assoc;

    fn new_const<'lt>(c: Self::Assoc, ctx: &'lt FnCodegen<'static>) -> Val<'lt, Self>;
}

macro_rules! derive_constant {
    ($name: ident, $tipe: ty, $ty_fn: ident, $const_ty_fn: ident$(, $rest_args: tt)?) => {
        impl AcceptsConstants for $name {
            type Assoc = $tipe;
            fn new_const<'lt>(c: Self::Assoc, cx: &'lt FnCodegen<'static>) -> Val<'lt, Self> {
                let val = cx
                    .ctx()
                    .$ty_fn()
                    .$const_ty_fn((c as _)$(, $rest_args)?)
                    .as_basic_value_enum();
                Val::new(cx, val)
            }
            const ONE: Self::Assoc = 1 as _;
            const ZERO: Self::Assoc = 0 as _;
        }
    };
}

// Can't do this yet because rustc stable doesn't support f16
// derive_constant!(F16, f16, f16_type, const_float);
derive_constant!(F32, f32, f32_type, const_float);
derive_constant!(F64, f64, f64_type, const_float);
// Can't do this yet because rustc stable doesn't support f128
// derive_constant!(F128, f128, f128_type, const_float);

derive_constant!(I8, i8, i8_type, const_int, true);
derive_constant!(I16, i16, i16_type, const_int, true);
derive_constant!(I32, i32, i32_type, const_int, true);
derive_constant!(I64, i64, i64_type, const_int, true);
derive_constant!(I128, i128, i128_type, const_int, true);

derive_constant!(U8, u8, i8_type, const_int, false);
derive_constant!(U16, u16, i16_type, const_int, false);
derive_constant!(U32, u32, i32_type, const_int, false);
derive_constant!(U64, u64, i64_type, const_int, false);
derive_constant!(U128, u128, i128_type, const_int, false);
