use std::cmp::Ordering;

use crate::{
    backend::UntypedValue,
    tipe::{BF16, F16, Ty, V},
    val::Val,
};

pub trait ConvertTo<Other>: Sized {
    fn cvt(val: Val<Self>) -> Val<Other>;
}

impl<T> Val<T> {
    pub fn cvt<Other>(self) -> Val<Other>
    where
        T: ConvertTo<Other>,
    {
        ConvertTo::cvt(self)
    }
}

macro_rules! float_cvt_chain {
    ($($from: ty => $to: ty),* $(,)?) => {
        $(
            impl ConvertTo<$to> for $from {
                fn cvt(val: Val<Self>) -> Val<$to> {
                    let fn_ref = val.fn_ref();
                    let b = unsafe { fn_ref.curr_bb_builder() };
                    let raw = b
                        .build_float_cast(
                            val.typed(),
                            <$to>::raw_ty(fn_ref.ctx()),
                            concat!(stringify!($from), "cvt", stringify!($to))
                        )
                        .expect("Float convert should have succeeded");
                    unsafe { Val::new(fn_ref.clone(), UntypedValue(raw.into())) }
                }
            }

            impl<const N: usize> ConvertTo<V<$to, N>> for V<$from, N> {
                fn cvt(val: Val<Self>) -> Val<V<$to, N>> {
                    let fn_ref = val.fn_ref();
                    let b = unsafe { fn_ref.curr_bb_builder() };
                    let raw = b
                        .build_float_cast(
                            val.typed(),
                            V::<$to, N>::raw_ty(fn_ref.ctx()),
                            concat!(stringify!($from), "cvt", stringify!($to))
                        )
                        .expect("Float convert should have succeeded");
                    unsafe { Val::new(fn_ref.clone(), UntypedValue(raw.into())) }
                }
            }
        )*
    };
}

macro_rules! int_cvt_chain {
    ($ext_name: ident: $($from: ty => $to: ty),* $(,)?;) => {
        $(
            impl ConvertTo<$to> for $from {
                fn cvt(val: Val<Self>) -> Val<$to> {
                    let fn_ref = val.fn_ref();
                    let b = unsafe { fn_ref.curr_bb_builder() };
                    let int_type = <$to>::raw_ty(fn_ref.ctx());
                    let name = concat!(stringify!($from), "_to_", stringify!($to));
                    let raw_ret = match Self::size().cmp(&<$to>::size()) {
                        Ordering::Less => b
                            .$ext_name(val.typed(), int_type, name)
                            .expect("Int extend should work"),
                        Ordering::Equal => val.typed(),
                        Ordering::Greater => b
                            .build_int_truncate(val.typed(), int_type, name)
                            .expect("Int truncate should work"),
                    };
                    unsafe { Val::new(fn_ref.clone(), UntypedValue(raw_ret.into())) }
                }
            }
        )*
    };
}

int_cvt_chain!(
    build_int_s_extend:
    i8 => i16,
    i8 => i32,
    i8 => i64,
    i8 => u8,
    i8 => u16,
    i8 => u32,
    i8 => u64,

    i16 => i8,
    i16 => i32,
    i16 => i64,
    i16 => u8,
    i16 => u16,
    i16 => u32,
    i16 => u64,

    i32 => i8,
    i32 => i16,
    i32 => i64,
    i32 => u8,
    i32 => u16,
    i32 => u32,
    i32 => u64,

    i64 => i8,
    i64 => i16,
    i64 => i32,
    i64 => u8,
    i64 => u16,
    i64 => u32,
    i64 => u64,
    ;
);

int_cvt_chain!(
    build_int_z_extend:
    u8 => i16,
    u8 => i32,
    u8 => i64,
    u8 => u8,
    u8 => u16,
    u8 => u32,
    u8 => u64,

    u16 => i8,
    u16 => i32,
    u16 => i64,
    u16 => u8,
    u16 => u16,
    u16 => u32,
    u16 => u64,

    u32 => i8,
    u32 => i16,
    u32 => i64,
    u32 => u8,
    u32 => u16,
    u32 => u32,
    u32 => u64,

    u64 => i8,
    u64 => i16,
    u64 => i32,
    u64 => u8,
    u64 => u16,
    u64 => u32,
    u64 => u64,
    ;
);

float_cvt_chain!(
    f64 => f32,
    f64 => F16,
    f64 => BF16,
    f32 => f64,
    f32 => F16,
    f32 => BF16,
    F16 => f64,
    F16 => f32,
    F16 => BF16,
    BF16 => f64,
    BF16 => f32,
    BF16 => F16,
);
