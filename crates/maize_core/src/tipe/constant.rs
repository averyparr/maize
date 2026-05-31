use crate::tipe::Ty;
use inkwell::values::BasicValue;

use crate::{
    backend::{FnRef, UntypedValue},
    val::Val,
};

pub trait IntoConst: Sized {
    fn as_const_val(val: Self, fn_ref: FnRef) -> Val<Self>;
}

macro_rules! const_int_impl {
    ($($tipes: ty),*) => {
        $(
            impl IntoConst for $tipes {
                fn as_const_val(val: Self, fn_ref: FnRef) -> Val<Self> {
                    let raw = Self::raw_ty(fn_ref.ctx()).const_int(val as _, false);
                    let raw = UntypedValue(raw.as_basic_value_enum());
                    // Safety: We have a constant
                    unsafe { Val::new(fn_ref, raw) }
                }
            }
        )*
    };
}

const_int_impl!(bool, i8, u8, i16, u16, i32, u32, i64, u64);

macro_rules! const_float_impl {
    ($($tipes: ty),*) => {
        $(
            impl IntoConst for $tipes {
                fn as_const_val(val: Self, fn_ref: FnRef) -> Val<Self> {
                    let raw = Self::raw_ty(fn_ref.ctx()).const_float(val as _);
                    let raw = UntypedValue(raw.as_basic_value_enum());
                    // Safety: We have a constant
                    unsafe { Val::new(fn_ref, raw) }
                }
            }
        )*
    };
}

const_float_impl!(f32, f64);
