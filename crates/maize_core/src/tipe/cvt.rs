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
