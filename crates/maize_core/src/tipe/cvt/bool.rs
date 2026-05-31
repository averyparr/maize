use crate::{
    backend::UntypedValue,
    tipe::{Ty, cvt::ConvertTo},
    val::Val,
};

macro_rules! convert_bools {
    ($($tipes: ty),*) => {
        $(
            impl ConvertTo<$tipes> for bool {
                fn cvt(val: Val<Self>) -> Val<$tipes> {
                    let fn_ref = val.fn_ref().clone();
                    let b = unsafe { fn_ref.curr_bb_builder() };
                    let raw = b
                        .build_int_z_extend(val.typed(), <$tipes>::raw_ty(fn_ref.ctx()), concat!("bool_to_", stringify!($tipes)))
                        .expect("bool to int should succeed");
                    unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
                }
            }

        )*
    };
}

convert_bools!(i8, i16, i32, i64, u8, u16, u32, u64);
