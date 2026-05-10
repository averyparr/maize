use crate::{ty::raw::*, val::Val};

macro_rules! try_const {
    (try_const_int, $inkwell_name: ident, $trace_ty: ty, $real_ty: ty) => {
        impl Val<'_, $trace_ty> {
            pub fn try_const_int(self) -> Option<$real_ty> {
                self.ll_typed().$inkwell_name().map(|v| {
                    v.try_into()
                        .expect(stringify!("Invalid value stored in ", $real_ty, "value"))
                })
            }
        }
    };
    (try_const_float, $inkwell_name: ident, $trace_ty: ty, $real_ty: ty) => {
        impl Val<'_, $trace_ty> {
            pub fn try_const_float(self) -> Option<$real_ty> {
                self.ll_typed()
                    .$inkwell_name()
                    .map(|(val, lost_precision)| {
                        assert!(
                            !lost_precision,
                            "Constants should not lose precision this way"
                        );
                        val as $real_ty
                    })
            }
        }
    };
}

try_const!(try_const_int, get_sign_extended_constant, I8, i8);
try_const!(try_const_int, get_sign_extended_constant, I16, i16);
try_const!(try_const_int, get_sign_extended_constant, I32, i32);
try_const!(try_const_int, get_sign_extended_constant, I64, i64);

try_const!(try_const_int, get_zero_extended_constant, U8, u8);
try_const!(try_const_int, get_zero_extended_constant, U16, u16);
try_const!(try_const_int, get_zero_extended_constant, U32, u32);
try_const!(try_const_int, get_zero_extended_constant, U64, u64);

try_const!(try_const_float, get_constant, F64, f64);
try_const!(try_const_float, get_constant, F32, f32);
