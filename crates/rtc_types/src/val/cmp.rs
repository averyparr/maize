use std::borrow::Borrow;

use inkwell::{
    builder::Builder,
    values::{AnyValue, AnyValueEnum, BasicValueEnum, FloatMathValue, IntMathValue},
};

use crate::{
    codegen::typed_func::ConstValTy,
    ty::{Bool, ValTy, raw::*, vec::VectorizableTy},
    val::Val,
};

#[derive(Clone, Copy, Debug)]
pub enum Predicate {
    EQ,
    GE,
    GT,
    LE,
    LT,
    NE,
}

impl Predicate {
    pub fn float(self) -> inkwell::FloatPredicate {
        use inkwell::FloatPredicate as FP;
        match self {
            Self::EQ => FP::OEQ,
            Self::GE => FP::OGE,
            Self::GT => FP::OGT,
            Self::LE => FP::OLE,
            Self::LT => FP::OLT,
            Self::NE => FP::ONE,
        }
    }
    pub fn signed_int(self) -> inkwell::IntPredicate {
        use inkwell::IntPredicate as IP;
        match self {
            Self::EQ => IP::EQ,
            Self::GE => IP::SGE,
            Self::GT => IP::SGT,
            Self::LE => IP::SLE,
            Self::LT => IP::SLT,
            Self::NE => IP::NE,
        }
    }
    pub fn unsigned_int(self) -> inkwell::IntPredicate {
        use inkwell::IntPredicate as IP;
        match self {
            Self::EQ => IP::EQ,
            Self::GE => IP::UGE,
            Self::GT => IP::UGT,
            Self::LE => IP::ULE,
            Self::LT => IP::ULT,
            Self::NE => IP::NE,
        }
    }
}

fn float_compare(
    b: Builder<'static>,
    pred: Predicate,
    lhs: AnyValueEnum<'static>,
    rhs: AnyValueEnum<'static>,
) -> AnyValueEnum<'static> {
    fn inner<Fl: FloatMathValue<'static>>(
        b: Builder<'static>,
        pred: Predicate,
        lhs: Fl,
        rhs: Fl,
    ) -> AnyValueEnum<'static> {
        b.build_float_compare(pred.float(), lhs, rhs, "fcmp")
            .expect("fcmp should always succeed")
            .as_any_value_enum()
    }
    match lhs {
        AnyValueEnum::FloatValue(lhs) => inner(b, pred, lhs, rhs.into_float_value()),
        AnyValueEnum::VectorValue(lhs) => inner(b, pred, lhs, rhs.into_vector_value()),
        AnyValueEnum::ScalableVectorValue(lhs) => {
            inner(b, pred, lhs, rhs.into_scalable_vector_value())
        }
        _ => panic!(
            "Attempted to call `float_compare` on {:?} {:?} {:?}",
            lhs, pred, rhs
        ),
    }
}

fn signed_int_compare(
    b: Builder<'static>,
    pred: Predicate,
    lhs: AnyValueEnum<'static>,
    rhs: AnyValueEnum<'static>,
) -> AnyValueEnum<'static> {
    fn inner<Fl: IntMathValue<'static>>(
        b: Builder<'static>,
        pred: Predicate,
        lhs: Fl,
        rhs: Fl,
    ) -> AnyValueEnum<'static> {
        b.build_int_compare(pred.signed_int(), lhs, rhs, "icmp")
            .expect("icmp should always succeed")
            .as_any_value_enum()
    }
    match lhs {
        AnyValueEnum::IntValue(lhs) => inner(b, pred, lhs, rhs.into_int_value()),
        AnyValueEnum::VectorValue(lhs) => inner(b, pred, lhs, rhs.into_vector_value()),
        AnyValueEnum::ScalableVectorValue(lhs) => {
            inner(b, pred, lhs, rhs.into_scalable_vector_value())
        }
        _ => panic!(
            "Attempted to call `signed_int_compare` on {:?} {:?} {:?}",
            lhs, pred, rhs
        ),
    }
}

fn unsigned_int_compare(
    b: Builder<'static>,
    pred: Predicate,
    lhs: AnyValueEnum<'static>,
    rhs: AnyValueEnum<'static>,
) -> AnyValueEnum<'static> {
    fn inner<Fl: IntMathValue<'static>>(
        b: Builder<'static>,
        pred: Predicate,
        lhs: Fl,
        rhs: Fl,
    ) -> AnyValueEnum<'static> {
        b.build_int_compare(pred.unsigned_int(), lhs, rhs, "ucmp")
            .expect("ucmp should always succeed")
            .as_any_value_enum()
    }
    match lhs {
        AnyValueEnum::IntValue(lhs) => inner(b, pred, lhs, rhs.into_int_value()),
        AnyValueEnum::VectorValue(lhs) => inner(b, pred, lhs, rhs.into_vector_value()),
        AnyValueEnum::ScalableVectorValue(lhs) => {
            inner(b, pred, lhs, rhs.into_scalable_vector_value())
        }
        _ => panic!(
            "Attempted to call `unsigned_int_compare` on {:?} {:?} {:?}",
            lhs, pred, rhs
        ),
    }
}

pub unsafe trait ComparableTy: ValTy {
    /// This is typcially bools, but vectors
    /// compare elementwise and return a V<Bool, _>
    /// so we must allow this type of specialization
    type ComparisonT: ValTy;
    fn compare<'a>(
        predicate: Predicate,
        lhs: &Val<'a, Self>,
        rhs: &Val<'a, Self>,
    ) -> Val<'a, Self::ComparisonT> {
        let raw_res = unsafe {
            lhs.cx()
                .with_builder(|b| Self::build_comparison_raw(b, predicate, lhs.raw(), rhs.raw()))
        };

        unsafe { Val::new(lhs.cx(), raw_res) }
    }

    fn build_comparison_raw(
        b: Builder<'static>,
        pred: Predicate,
        lhs: BasicValueEnum<'static>,
        rhs: BasicValueEnum<'static>,
    ) -> BasicValueEnum<'static>;
}

macro_rules! compare_for_scalar {
    ($trace_ty: ty => $raw_compare_specialization: ident) => {
        unsafe impl ComparableTy for $trace_ty {
            type ComparisonT = Bool;
            fn build_comparison_raw(
                b: Builder<'static>,
                pred: Predicate,
                lhs: BasicValueEnum<'static>,
                rhs: BasicValueEnum<'static>,
            ) -> BasicValueEnum<'static> {
                $raw_compare_specialization(b, pred, lhs.into(), rhs.into())
                    .try_into()
                    .expect("This returned an any-value which was not a basic-value")
            }
        }
    };
}

compare_for_scalar!(F32 => float_compare);
compare_for_scalar!(I32 => signed_int_compare);
compare_for_scalar!(U32 => unsigned_int_compare);

unsafe impl<T, const N: usize> ComparableTy for V<T, N>
where
    T: ComparableTy + VectorizableTy,
{
    type ComparisonT = V<Bool, N>;
    fn build_comparison_raw(
        b: Builder<'static>,
        pred: Predicate,
        lhs: BasicValueEnum<'static>,
        rhs: BasicValueEnum<'static>,
    ) -> BasicValueEnum<'static> {
        T::build_comparison_raw(b, pred, lhs, rhs)
    }
}

impl<'a, T> Val<'a, T>
where
    T: ComparableTy,
{
    pub fn eq(&self, rhs: impl Borrow<Self>) -> Val<'a, T::ComparisonT> {
        T::compare(Predicate::EQ, self, rhs.borrow())
    }
    pub fn ne(&self, rhs: impl Borrow<Self>) -> Val<'a, T::ComparisonT> {
        T::compare(Predicate::NE, self, rhs.borrow())
    }
    pub fn le(&self, rhs: impl Borrow<Self>) -> Val<'a, T::ComparisonT> {
        T::compare(Predicate::LE, self, rhs.borrow())
    }
    pub fn lt(&self, rhs: impl Borrow<Self>) -> Val<'a, T::ComparisonT> {
        T::compare(Predicate::LT, self, rhs.borrow())
    }
    pub fn ge(&self, rhs: impl Borrow<Self>) -> Val<'a, T::ComparisonT> {
        T::compare(Predicate::GE, self, rhs.borrow())
    }
    pub fn gt(&self, rhs: impl Borrow<Self>) -> Val<'a, T::ComparisonT> {
        T::compare(Predicate::GT, self, rhs.borrow())
    }
}

impl<'a, C> Val<'a, C>
where
    C: ConstValTy + ComparableTy,
{
    pub fn eq_const(&self, rhs: C::Assoc) -> Val<'a, C::ComparisonT> {
        C::compare(Predicate::EQ, self, &C::to_const(rhs, self.cx()))
    }
    pub fn ne_const(&self, rhs: C::Assoc) -> Val<'a, C::ComparisonT> {
        C::compare(Predicate::NE, self, &C::to_const(rhs, self.cx()))
    }
    pub fn le_const(&self, rhs: C::Assoc) -> Val<'a, C::ComparisonT> {
        C::compare(Predicate::LE, self, &C::to_const(rhs, self.cx()))
    }
    pub fn lt_const(&self, rhs: C::Assoc) -> Val<'a, C::ComparisonT> {
        C::compare(Predicate::LT, self, &C::to_const(rhs, self.cx()))
    }
    pub fn ge_const(&self, rhs: C::Assoc) -> Val<'a, C::ComparisonT> {
        C::compare(Predicate::GE, self, &C::to_const(rhs, self.cx()))
    }
    pub fn gt_const(&self, rhs: C::Assoc) -> Val<'a, C::ComparisonT> {
        C::compare(Predicate::GT, self, &C::to_const(rhs, self.cx()))
    }
}
