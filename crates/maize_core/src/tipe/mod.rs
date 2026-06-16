mod array;
mod cmp;
pub mod constant;
mod cvt;
pub mod indexing;
mod math;
mod ptr;
pub mod reflection;
mod slice;
mod uninit;
mod vec;

use std::mem::MaybeUninit;

use inkwell::{
    types::{BasicType, BasicTypeEnum},
    values::BasicValue,
};
pub use math::MathTy;
pub use ptr::A;
pub use vec::{V, VecTy};

use crate::{
    ContextRef, FloatType, FloatValue, IntType, IntValue,
    backend::{ErasedType, FnCtx, FnRef, UntypedValue},
    val::Val,
};

pub trait Ty {
    type LLType: BasicType<'static>;
    type LLVal: BasicValue<'static>;
    fn raw_ty(ctx: ContextRef) -> Self::LLType;
    fn ty(cg: &FnCtx) -> ErasedType {
        ErasedType(Self::raw_ty(cg.ctx()).as_basic_type_enum())
    }
    fn size() -> usize
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>()
    }
    fn align() -> usize
    where
        Self: Sized,
    {
        std::mem::align_of::<Self>()
    }
    fn type_val(val: UntypedValue) -> Self::LLVal;
    fn const_val(self, fn_ref: FnRef) -> Val<Self>
    where
        Self: Sized;
    fn undef_val(fn_ref: FnRef) -> Val<Self>
    where
        Self: Sized,
    {
        let undef = match Self::raw_ty(fn_ref.ctx()).as_basic_type_enum() {
            BasicTypeEnum::ArrayType(array_type) => array_type.get_undef().as_basic_value_enum(),
            BasicTypeEnum::FloatType(float_type) => float_type.get_undef().as_basic_value_enum(),
            BasicTypeEnum::IntType(int_type) => int_type.get_undef().as_basic_value_enum(),
            BasicTypeEnum::PointerType(pointer_type) => {
                pointer_type.get_undef().as_basic_value_enum()
            }
            BasicTypeEnum::StructType(struct_type) => struct_type.get_undef().as_basic_value_enum(),
            BasicTypeEnum::VectorType(vector_type) => vector_type.get_undef().as_basic_value_enum(),
            BasicTypeEnum::ScalableVectorType(scalable_vector_type) => {
                scalable_vector_type.get_undef().as_basic_value_enum()
            }
        };
        unsafe { Val::new(fn_ref, UntypedValue(undef)) }
    }
    fn type_metadata(_: &FnCtx, _: &mut UntypedValue) {}
    fn function_arg_types(ctx: ContextRef) -> impl ExactSizeIterator<Item = ErasedType> {
        [ErasedType(Self::raw_ty(ctx).as_basic_type_enum())].into_iter()
    }
    fn type_metadata_on_function(_: &FnCtx, _: &mut impl Iterator<Item = u32>) {}
    unsafe fn extract_arg(fn_ref: FnRef, iter: &mut impl Iterator<Item = UntypedValue>) -> Val<Self>
    where
        Self: Sized,
    {
        unsafe { Val::new(fn_ref, iter.next().expect("Should be in range")) }
    }
}

macro_rules! basic_impl_ty {
    ($(
    $tipes: ty => $type_fns: ident | $llty: ident | $llval: ident | $type_val_fn: ident | $fn_to_const: ident
    ),* $(,)?) => {
        $(
            impl Ty for $tipes {
                type LLType = $llty;
                type LLVal = $llval;
                fn raw_ty(ctx: ContextRef) -> Self::LLType {
                    ctx.$type_fns()
                }
                fn type_val(val: UntypedValue) -> Self::LLVal {
                    val.0.$type_val_fn()
                }
                fn const_val(self, fn_ref: FnRef) -> Val<Self> where Self: Copy {
                    let raw = $fn_to_const(Self::raw_ty(fn_ref.ctx()), self);
                    unsafe {Val::new(fn_ref, UntypedValue::new(raw.as_basic_value_enum()))}
                }
            }
        )*
    };
}

#[derive(Clone, Copy, PartialEq)]
pub struct F16(u16);
#[derive(Clone, Copy, PartialEq)]
pub struct BF16(u16);
#[derive(Clone, Copy, PartialEq)]
pub struct F8E4M3(u8);
#[derive(Clone, Copy, PartialEq)]
pub struct F8E5M2(u8);
#[derive(Clone, Copy, PartialEq)]
pub struct F8E8M0(u8);

pub trait FromF32 {
    fn to_f32(self) -> f32;
    fn from_f32(val: f32) -> Self;
}

#[inline]
fn round_shift(value: u32, shift: u32) -> u32 {
    if shift == 0 {
        return value;
    }
    if shift >= 32 {
        return 0;
    }
    let dropped = value & ((1u32 << shift) - 1);
    let half = 1u32 << (shift - 1);
    let kept = value >> shift;
    if dropped > half || (dropped == half && (kept & 1) == 1) {
        kept + 1
    } else {
        kept
    }
}

// ---------------- F16 (IEEE half: 1-5-10, bias 15) ----------------

impl FromF32 for F16 {
    fn to_f32(self) -> f32 {
        let bits = self.0;
        let sign = (bits >> 15) & 0x1;
        let exp = (bits >> 10) & 0x1f;
        let mant = bits & 0x3ff;
        let sign_f = (sign as u32) << 31;

        let out: u32 = match exp {
            0 => {
                if mant == 0 {
                    sign_f
                } else {
                    let mut m = mant as u32;
                    let mut e: i32 = -1;
                    while m & 0x400 == 0 {
                        m <<= 1;
                        e += 1;
                    }
                    m &= 0x3ff;
                    let exp_f = ((127 - 15 - e) as u32) << 23;
                    sign_f | exp_f | (m << 13)
                }
            }
            0x1f => sign_f | (0xff << 23) | ((mant as u32) << 13),
            _ => {
                let exp_f = ((exp as i32 - 15 + 127) as u32) << 23;
                sign_f | exp_f | ((mant as u32) << 13)
            }
        };
        f32::from_bits(out)
    }

    fn from_f32(val: f32) -> Self {
        let x = val.to_bits();
        let sign = ((x >> 31) & 0x1) as u16;
        let exp = ((x >> 23) & 0xff) as i32;
        let mant = x & 0x7fffff;
        let sign_h = sign << 15;

        // NaN / Inf
        if exp == 0xff {
            if mant != 0 {
                // NaN: keep it a NaN, preserve top mantissa bits
                let payload = (mant >> 13) as u16 & 0x3ff;
                return F16(sign_h | (0x1f << 10) | (payload | (payload == 0) as u16));
            }
            return F16(sign_h | (0x1f << 10)); // Inf
        }

        let unbiased = exp - 127;

        // Overflow to Inf
        if unbiased > 15 {
            return F16(sign_h | (0x1f << 10));
        }

        // Normal range for f16: unbiased in [-14, 15]
        if unbiased >= -14 {
            let exp_h = ((unbiased + 15) as u16) << 10;
            // round-to-nearest-even on the 13 dropped bits
            let m = round_shift(mant, 13) as u16;
            // rounding may carry into exponent; reassemble via add handles it
            return F16(sign_h | (exp_h | m).wrapping_add(0)); // m may be 0x400 -> carries into exp
        }

        // Subnormal or underflow to zero
        // value = 1.mant x 2^unbiased, shift to align to f16 subnormal (min exp -14)
        let shift = (-14 - unbiased) as u32; // >= 1
        if shift > 24 {
            return F16(sign_h); // underflow to signed zero
        }
        let full_mant = mant | 0x800000; // restore implicit 1
        let total_shift = 13 + shift;
        let m = round_shift(full_mant, total_shift) as u16;
        F16(sign_h | m)
    }
}

// ---------------- BF16 (1-8-7, bias 127 = top 16 bits of f32) ----------------

impl FromF32 for BF16 {
    fn to_f32(self) -> f32 {
        f32::from_bits((self.0 as u32) << 16)
    }

    fn from_f32(val: f32) -> Self {
        let x = val.to_bits();
        // NaN: ensure result stays NaN
        if val.is_nan() {
            return BF16(((x >> 16) as u16) | 0x0040);
        }
        // round-to-nearest-even
        let rounding_bias = 0x7fff + ((x >> 16) & 1);
        BF16(((x + rounding_bias) >> 16) as u16)
    }
}

// ---------------- F8E4M3 (1-4-3, bias 7, no Inf; NaN = 0x7F/0xFF) ----------------

impl FromF32 for F8E4M3 {
    fn to_f32(self) -> f32 {
        let bits = self.0;
        let sign = (bits >> 7) & 0x1;
        let exp = (bits >> 3) & 0xf;
        let mant = bits & 0x7;
        let sign_f = (sign as u32) << 31;

        // NaN: S.1111.111
        if exp == 0xf && mant == 0x7 {
            return f32::from_bits(sign_f | 0x7fc00000);
        }

        let out: u32 = match exp {
            0 => {
                if mant == 0 {
                    sign_f
                } else {
                    let mut m = mant as u32;
                    let mut e: i32 = -1;
                    while m & 0x8 == 0 {
                        m <<= 1;
                        e += 1;
                    }
                    m &= 0x7;
                    let exp_f = ((127 - 7 - e) as u32) << 23;
                    sign_f | exp_f | (m << 20)
                }
            }
            _ => {
                let exp_f = ((exp as i32 - 7 + 127) as u32) << 23;
                sign_f | exp_f | ((mant as u32) << 20)
            }
        };
        f32::from_bits(out)
    }

    fn from_f32(val: f32) -> Self {
        let x = val.to_bits();
        let sign = ((x >> 31) & 0x1) as u8;
        let sign_b = sign << 7;

        if val.is_nan() {
            return F8E4M3(sign_b | 0x7f);
        }

        let exp = ((x >> 23) & 0xff) as i32;
        let mant = x & 0x7fffff;
        let unbiased = exp - 127;

        // Max normal for E4M3 is 448 (S.1111.110). Clamp overflow/Inf to max (no Inf in format).
        // unbiased > 8, or (==8 and would round above 448) -> saturate
        if val.is_infinite() || unbiased > 8 {
            return F8E4M3(sign_b | 0x7e);
        }

        if unbiased >= -6 {
            // normal
            let m = round_shift(mant, 20);
            let mut e = unbiased + 7;
            let mut mm = m;
            if mm == 0x8 {
                // mantissa carry
                mm = 0;
                e += 1;
            }
            if e > 0xf || (e == 0xf && mm > 0x6) {
                return F8E4M3(sign_b | 0x7e); // saturate to 448
            }
            return F8E4M3(sign_b | ((e as u8) << 3) | (mm as u8));
        }

        // subnormal
        let shift = (-6 - unbiased) as u32;
        if shift > 4 {
            return F8E4M3(sign_b); // underflow
        }
        let full_mant = mant | 0x800000;
        let m = round_shift(full_mant, 20 + shift) as u8;
        F8E4M3(sign_b | m)
    }
}

// ---------------- F8E5M2 (1-5-2, bias 15, IEEE-style Inf/NaN) ----------------

impl FromF32 for F8E5M2 {
    fn to_f32(self) -> f32 {
        let bits = self.0;
        let sign = (bits >> 7) & 0x1;
        let exp = (bits >> 2) & 0x1f;
        let mant = bits & 0x3;
        let sign_f = (sign as u32) << 31;

        let out: u32 = match exp {
            0 => {
                if mant == 0 {
                    sign_f
                } else {
                    let mut m = mant as u32;
                    let mut e: i32 = -1;
                    while m & 0x4 == 0 {
                        m <<= 1;
                        e += 1;
                    }
                    m &= 0x3;
                    let exp_f = ((127 - 15 - e) as u32) << 23;
                    sign_f | exp_f | (m << 21)
                }
            }
            0x1f => sign_f | (0xff << 23) | ((mant as u32) << 21),
            _ => {
                let exp_f = ((exp as i32 - 15 + 127) as u32) << 23;
                sign_f | exp_f | ((mant as u32) << 21)
            }
        };
        f32::from_bits(out)
    }

    fn from_f32(val: f32) -> Self {
        let x = val.to_bits();
        let sign = ((x >> 31) & 0x1) as u8;
        let sign_b = sign << 7;

        if val.is_nan() {
            return F8E5M2(sign_b | 0x7f); // exp all ones, nonzero mant
        }
        if val.is_infinite() {
            return F8E5M2(sign_b | 0x7c);
        }

        let exp = ((x >> 23) & 0xff) as i32;
        let mant = x & 0x7fffff;
        let unbiased = exp - 127;

        // Overflow to Inf (max normal exp unbiased == 15)
        if unbiased > 15 {
            return F8E5M2(sign_b | 0x7c);
        }

        if unbiased >= -14 {
            let m = round_shift(mant, 21);
            let mut e = unbiased + 15;
            let mut mm = m;
            if mm == 0x4 {
                mm = 0;
                e += 1;
            }
            if e >= 0x1f {
                return F8E5M2(sign_b | 0x7c); // overflow to Inf
            }
            return F8E5M2(sign_b | ((e as u8) << 2) | (mm as u8));
        }

        // subnormal
        let shift = (-14 - unbiased) as u32;
        if shift > 3 {
            return F8E5M2(sign_b);
        }
        let full_mant = mant | 0x800000;
        let m = round_shift(full_mant, 21 + shift) as u8;
        F8E5M2(sign_b | m)
    }
}

fn const_into_i64(ty: IntType, val: impl Into<i64>) -> IntValue {
    ty.const_int(val.into() as u64, false)
}

fn const_u64(ty: IntType, val: u64) -> IntValue {
    ty.const_int(val, false)
}

fn const_i128(_: IntType, _: i128) -> IntValue {
    todo!("Unable to represent const i128s for now");
}

fn const_u128(_: IntType, _: u128) -> IntValue {
    todo!("Unable to represent const u128s for now");
}

fn const_e4m3(ty: IntType, val: F8E4M3) -> IntValue {
    ty.const_int(val.0 as _, false)
}

fn const_e5m2(ty: IntType, val: F8E5M2) -> IntValue {
    ty.const_int(val.0 as _, false)
}

fn const_e8m0(ty: IntType, val: F8E8M0) -> IntValue {
    ty.const_int(val.0 as _, false)
}

fn const_f16(ty: FloatType, val: F16) -> FloatValue {
    let val = {
        let bits = val.0;
        let sign = (bits >> 15) & 0x1;
        let exp = (bits >> 10) & 0x1f;
        let mant = bits & 0x3ff;

        let sign_f = (sign as u32) << 31;

        let out: u32 = match exp {
            0 => {
                if mant == 0 {
                    // signed zero
                    sign_f
                } else {
                    // subnormal: normalize it
                    let mut m = mant as u32;
                    let mut e: i32 = -1;
                    while m & 0x400 == 0 {
                        m <<= 1;
                        e += 1;
                    }
                    m &= 0x3ff; // remove the implicit leading bit
                    let exp_f = ((127 - 15 - e) as u32) << 23;
                    let mant_f = m << 13;
                    sign_f | exp_f | mant_f
                }
            }
            0x1f => {
                // inf or NaN
                sign_f | (0xff << 23) | ((mant as u32) << 13)
            }
            _ => {
                // normal
                let exp_f = ((exp as i32 - 15 + 127) as u32) << 23;
                let mant_f = (mant as u32) << 13;
                sign_f | exp_f | mant_f
            }
        };

        f32::from_bits(out)
    };
    ty.const_float(val as _)
}

fn const_bf16(ty: FloatType, val: BF16) -> FloatValue {
    let bits = val.0;
    let val = f32::from_bits((bits as u32) << 16);
    ty.const_float(val as _)
}

fn const_into_f64(ty: FloatType, val: impl Into<f64>) -> FloatValue {
    ty.const_float(val.into())
}

basic_impl_ty!(
    i8 => i8_type | IntType | IntValue | into_int_value | const_into_i64,
    i16 => i16_type | IntType | IntValue | into_int_value | const_into_i64,
    i32 => i32_type | IntType | IntValue | into_int_value | const_into_i64,
    i64 => i64_type | IntType | IntValue | into_int_value | const_into_i64,
    i128 => i128_type | IntType | IntValue | into_int_value | const_i128,

    u8 => i8_type | IntType | IntValue | into_int_value | const_into_i64,
    u16 => i16_type | IntType | IntValue | into_int_value | const_into_i64,
    u32 => i32_type | IntType | IntValue | into_int_value | const_into_i64,
    u64 => i64_type | IntType | IntValue | into_int_value | const_u64,
    u128 => i128_type | IntType | IntValue | into_int_value | const_u128,

    F8E4M3 => i8_type | IntType | IntValue | into_int_value | const_e4m3,
    F8E5M2 => i8_type | IntType | IntValue | into_int_value | const_e5m2,
    F8E8M0 => i8_type | IntType | IntValue | into_int_value | const_e8m0,
    F16 => f16_type | FloatType | FloatValue | into_float_value | const_f16,
    BF16 => bf16_type | FloatType | FloatValue | into_float_value | const_bf16,
    f32 => f32_type | FloatType | FloatValue | into_float_value | const_into_f64,
    f64 => f64_type | FloatType | FloatValue | into_float_value | const_into_f64,
);

impl Ty for bool {
    type LLType = IntType;
    type LLVal = IntValue;

    fn raw_ty(ctx: ContextRef) -> Self::LLType {
        ctx.bool_type()
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        val.0.into_int_value()
    }
    fn const_val(self, fn_ref: FnRef) -> Val<Self>
    where
        Self: Copy,
    {
        let raw = fn_ref.ctx().bool_type().const_int(self as _, false);
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}
