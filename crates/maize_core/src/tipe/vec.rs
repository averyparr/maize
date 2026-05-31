use inkwell::{
    builder::{Builder, BuilderError},
    context::ContextRef,
    types::{BasicType, VectorType},
    values::VectorValue,
};

use crate::{
    backend::{FnRef, UntypedValue},
    tipe::{
        BF16, F8E4M3, F8E5M2, F8E8M0, F16, Ty,
        math::{DivType, MathTy},
    },
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

macro_rules! impl_vec_math {
    ($name: literal: $op_name: ident => $float_op: ident | $signed_op: ident | $unsigned_op: ident) => {
        fn $op_name(
            b: Builder<'static>,
            fn_ref: FnRef,
            lhs: UntypedValue,
            rhs: UntypedValue,
        ) -> Result<Val<Self>, BuilderError> {
            let lhs = lhs.0.into_vector_value();
            let rhs = rhs.0.into_vector_value();
            let elem_ty = lhs.get_type().get_element_type();
            assert_eq!(elem_ty, rhs.get_type().get_element_type());
            assert_eq!(elem_ty, T::raw_ty(fn_ref.ctx()).as_basic_type_enum());

            let res = match T::DIV_TYPE {
                DivType::Signed => b.$signed_op(lhs, rhs, concat!("ivec_", $name)),
                DivType::Unsigned => b.$unsigned_op(lhs, rhs, concat!("uvec_", $name)),
                DivType::Float => b.$float_op(lhs, rhs, concat!("fvec_", $name)),
            };
            res.map(|v| unsafe { Val::new(fn_ref, UntypedValue(v.into())) })
        }
    };
}

impl<T: VecTy, const N: usize> MathTy for V<T, N>
where
    T: MathTy,
{
    impl_vec_math!("add": try_emit_add => build_float_add | build_int_add | build_int_add);
    impl_vec_math!("sub": try_emit_sub => build_float_sub | build_int_sub | build_int_sub);
    impl_vec_math!("mul": try_emit_mul => build_float_mul | build_int_mul | build_int_mul);
    impl_vec_math!("div": try_emit_div => build_float_div | build_int_signed_div | build_int_unsigned_div);

    const DIV_TYPE: DivType = T::DIV_TYPE;
}

impl<T: VecTy> Val<T> {
    pub fn splat<const N: usize>(&self) -> Val<V<T, N>> {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let raw_vec = T::splat(self.typed(), N as _, self.fn_ref().ctx(), b);
        unsafe { Val::new(self.fn_ref().clone(), UntypedValue(raw_vec.into())) }
    }
}

impl<T: VecTy, const N: usize> Val<V<T, N>> {
    pub fn elements(self) -> [Val<T>; N] {
        std::array::from_fn(|i| {
            let fn_ref = self.fn_ref();
            let val = self.typed();
            let b = unsafe { fn_ref.curr_bb_builder() };
            let index = fn_ref.ctx().i32_type().const_int(i as _, false);
            let elem = b
                .build_extract_element(val, index, "extractelem")
                .expect("Build extract element should work");
            unsafe { Val::new(fn_ref.clone(), UntypedValue(elem)) }
        })
    }

    pub fn from_elements(elements: [Val<T>; N]) -> Self {
        let fn_ref = elements[0].fn_ref().clone();
        let mut raw = V::<T, N>::raw_ty(fn_ref.ctx()).const_zero();
        let b = unsafe { fn_ref.curr_bb_builder() };

        for (i, elem) in elements.iter().enumerate() {
            let index: Val<u32> = fn_ref.constant(i.try_into().expect("usize -> u32 overflow"));
            raw = b
                .build_insert_element(raw, elem.typed(), index.typed(), "from_elements")
                .expect("Insert element should succeed");
        }

        unsafe { Self::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub fn extract_vec<const E: usize>(self, offset: usize) -> Val<V<T, E>> {
        let fn_ref = self.fn_ref().clone();
        let intrins = fn_ref
            .get_intrinsic::<V<T, E>, (V<T, N>, i64)>("llvm.vector.extract", true)
            .expect("Should have a llvm vector extract");
        let offset = fn_ref.constant(offset as _);
        fn_ref.call_extern(intrins, (self, offset), None)
    }

    pub fn insert_vec<const E: usize>(self, other: Val<V<T, E>>, offset: usize) -> Self {
        let fn_ref = self.fn_ref().clone();
        let intrins = fn_ref
            .get_intrinsic::<V<T, N>, (V<T, N>, V<T, E>, i64)>("llvm.vector.insert", false)
            .expect("Should have a llvm vector extract");
        let offset = fn_ref.constant(offset as _);
        fn_ref.call_extern(intrins, (self, other, offset), None)
    }

    pub fn extract_element(self, offset: usize) -> Val<T> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let index: Val<u32> = fn_ref.constant(offset as u32);
        let raw = b
            .build_extract_element(self.typed(), index.typed(), "extractelem")
            .expect("Extract element should suceed");
        unsafe { Val::new(fn_ref, UntypedValue(raw)) }
    }

    pub fn insert_element(self, element: Val<T>, offset: usize) -> Self {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let index: Val<u32> = fn_ref.constant(offset as u32);
        let raw = b
            .build_insert_element(self.typed(), element.typed(), index.typed(), "extractelem")
            .expect("Extract element should suceed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub fn windows<const W: usize>(
        self,
    ) -> (
        impl Iterator<Item = Val<V<T, W>>>,
        impl Iterator<Item = Val<T>>,
    )
    where
        T: Copy,
    {
        let num_windows = N / W;
        let rem = N % W;
        let winself = self.copy();
        let windows = (0..num_windows)
            .map(|w| w * W)
            .map(move |offset| winself.copy().extract_vec(offset));
        let rest_offset = num_windows * W;
        let rest = (rest_offset..rest_offset + rem).map(move |o| {
            let b = unsafe { self.fn_ref().curr_bb_builder() };
            let index = self.fn_ref().constant(o as u32).typed();
            let raw_elem = b
                .build_extract_element(self.typed(), index, "rest_extract")
                .expect("extract element should succeed");
            unsafe { Val::new(self.fn_ref().clone(), UntypedValue(raw_elem)) }
        });
        (windows, rest)
    }
}
