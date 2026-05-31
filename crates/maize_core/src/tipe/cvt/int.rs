use super::ConvertTo;

use crate::{backend::UntypedValue, tipe::Ty, val::Val};
use std::cmp::Ordering;

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
