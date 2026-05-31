mod elements;
mod math;

use inkwell::{builder::Builder, context::ContextRef, types::VectorType, values::VectorValue};

use crate::{
    backend::{FnRef, UntypedValue},
    tipe::{BF16, F8E4M3, F8E5M2, F8E8M0, F16, Ty},
    val::Val,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct V<T, const N: usize>([T; N]);

pub trait VecTy: Ty + Copy {
    fn vectorize(raw_ty: Self::LLType, size: u32) -> VectorType<'static>;
    fn splat(
        raw_val: Self::LLVal,
        size: u32,
        ctx: ContextRef<'static>,
        b: Builder<'static>,
    ) -> VectorValue<'static>;
}

macro_rules! impl_multi_vec {
    ($($tipes: ty),*) => {
        $(
            impl VecTy for $tipes {
                fn vectorize(raw_ty: Self::LLType, size: u32) -> VectorType<'static> {
                    raw_ty.vec_type(size)
                }
                fn splat(raw_val: Self::LLVal, size: u32, ctx: ContextRef<'static>, b: Builder<'static>) -> VectorValue<'static> {
                    let mut raw = Self::vectorize(raw_val.get_type(), size).const_zero();
                    for i in 0..size {
                        raw = b.build_insert_element(
                            raw,
                            raw_val,
                            ctx.i32_type().const_int(i as _, false),
                            "splat"
                        ).expect("insert element should succeed here");
                    }
                    raw
                }
            }
        )*
    };
}

impl_multi_vec!(
    i8, i16, i32, i64, u8, u16, u32, u64, F8E4M3, F8E5M2, F8E8M0, F16, BF16, f32, f64
);

impl<T, const N: usize> Ty for V<T, N>
where
    T: VecTy,
{
    type LLType = VectorType<'static>;
    type LLVal = VectorValue<'static>;

    fn raw_ty(ctx: ContextRef<'static>) -> Self::LLType {
        T::vectorize(T::raw_ty(ctx), N as u32)
    }

    fn type_val(val: UntypedValue) -> Self::LLVal {
        val.0.into_vector_value()
    }

    fn align() -> usize
    where
        Self: Sized,
    {
        let mut align = T::align();
        let mut rem = N;
        while rem.is_multiple_of(2) {
            rem /= 2;
            align *= 2;
        }
        align
    }

    fn const_val(self, fn_ref: FnRef) -> Val<Self>
    where
        Self: Copy,
    {
        let ty = Self::raw_ty(fn_ref.ctx());
        let mut raw = ty.const_zero();
        let b = unsafe { fn_ref.curr_bb_builder() };
        for (i, val) in self.0.into_iter().enumerate() {
            raw = b
                .build_insert_element(
                    raw,
                    T::const_val(val, fn_ref.clone()).typed(),
                    Ty::const_val(i as u64, fn_ref.clone()).typed(),
                    "insert",
                )
                .expect("insert element should succeed here");
        }

        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }
}
