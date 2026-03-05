use inkwell::{
    intrinsics::Intrinsic,
    types::{FloatType, IntType},
    values::{AnyValueEnum, BasicValue, FloatValue},
};

use crate::{
    ty::{MathTy, MathVariant, V, ValTy, vec::VectorizableTy},
    val::Val,
};

trait SumProdReducable {
    fn add_identity<'a>(self) -> AnyValueEnum<'a>
    where
        Self: 'a;
    fn mul_identity<'a>(self) -> AnyValueEnum<'a>
    where
        Self: 'a;
}

impl SumProdReducable for IntType<'_> {
    fn add_identity<'a>(self) -> AnyValueEnum<'a>
    where
        Self: 'a,
    {
        self.const_int(0, false).into()
    }
    fn mul_identity<'a>(self) -> AnyValueEnum<'a>
    where
        Self: 'a,
    {
        self.const_int(1, false).into()
    }
}
impl SumProdReducable for FloatType<'_> {
    fn add_identity<'a>(self) -> AnyValueEnum<'a>
    where
        Self: 'a,
    {
        self.const_float(-0.0).into()
    }
    fn mul_identity<'a>(self) -> AnyValueEnum<'a>
    where
        Self: 'a,
    {
        self.const_float(1.0).into()
    }
}

fn call_vector_reduction_intrinsic<'a, T: VectorizableTy, const N: usize>(
    intrinsic_name: &str,
    val: Val<'a, V<T, N>>,
) -> Val<'a, T> {
    let intrinsic = Intrinsic::find(intrinsic_name).expect("Should have an intrinsic of this name");
    let vector_ty = val.ll_typed().get_type();
    let function = intrinsic
        .get_declaration(val.cx().module(), &[vector_ty.into()])
        .expect("There should be a function with this type");
    let raw_ret = unsafe {
        val.cx()
            .with_builder(|b| b.build_call(function, &[val.ll_typed().into()], "vec_red"))
    }
    .expect("Build call should always succeed");

    if let Some(ins) = raw_ret
        .try_as_basic_value()
        .basic()
        .expect("Should always be a basic value")
        .as_instruction_value()
    {
        val.cx().apply_ins_opt(ins);
    }

    unsafe { Val::new(val.cx(), raw_ret.try_as_basic_value().unwrap_basic()) }
}

fn call_float_vector_reduction_with_init<'a, T: VectorizableTy, const N: usize>(
    intrinsic_name: &str,
    init: FloatValue<'static>,
    val: Val<'a, V<T, N>>,
) -> Val<'a, T> {
    let intrinsic = Intrinsic::find(intrinsic_name).expect("Should have an intrinsic of this name");
    let vector_ty = val.ll_typed().get_type();
    // NOTE: This _looks_ incorrect as the signature is typically (elt, [elt x N]),
    // but in pratice this is how you actually look it up.
    let function = intrinsic
        .get_declaration(val.cx().module(), &[vector_ty.into()])
        .expect("There should be a function of signature `elt (elt, [elt x N])`");
    let raw_ret = unsafe {
        val.cx().with_builder(|b| {
            b.build_call(
                function,
                &[init.into(), val.ll_typed().into()],
                "float_reduction",
            )
        })
    }
    .expect("This call should always succeed");

    if let Some(ins) = raw_ret
        .try_as_basic_value()
        .basic()
        .expect("Should always be a basic value")
        .as_instruction_value()
    {
        val.cx().apply_ins_opt(ins);
    }

    unsafe { Val::new(val.cx(), raw_ret.try_as_basic_value().unwrap_basic()) }
}

trait SumProdReducableTy:
    MathTy + VectorizableTy + for<'a> ValTy<Type<'a>: SumProdReducable> + Sized
{
    fn call_sum<const N: usize>(val: Val<'_, V<Self, N>>) -> Val<'_, Self> {
        let ty = val.ll_typed();
        let element_ty = ty.get_type().get_element_type();
        match Self::MATH_VARIANT {
            MathVariant::Float => call_float_vector_reduction_with_init(
                "llvm.vector.reduce.fadd",
                element_ty
                    .into_float_type()
                    .add_identity()
                    .into_float_value(),
                val,
            ),
            MathVariant::SignedInt | MathVariant::UnsignedInt => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.add", val)
            }
        }
    }
    fn call_prod<const N: usize>(val: Val<'_, V<Self, N>>) -> Val<'_, Self> {
        let ty = val.ll_typed();
        let element_ty = ty.get_type().get_element_type();
        match Self::MATH_VARIANT {
            MathVariant::Float => call_float_vector_reduction_with_init(
                "llvm.vector.reduce.fmul",
                element_ty
                    .into_float_type()
                    .mul_identity()
                    .into_float_value(),
                val,
            ),
            MathVariant::SignedInt | MathVariant::UnsignedInt => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.mul", val)
            }
        }
    }
    fn call_max<const N: usize>(val: Val<'_, V<Self, N>>) -> Val<'_, Self> {
        match Self::MATH_VARIANT {
            crate::ty::MathVariant::Float => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.fmax", val)
            }
            crate::ty::MathVariant::SignedInt => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.smax", val)
            }
            crate::ty::MathVariant::UnsignedInt => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.umax", val)
            }
        }
    }
    fn call_min<const N: usize>(val: Val<'_, V<Self, N>>) -> Val<'_, Self> {
        match Self::MATH_VARIANT {
            crate::ty::MathVariant::Float => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.fmin", val)
            }
            crate::ty::MathVariant::SignedInt => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.smin", val)
            }
            crate::ty::MathVariant::UnsignedInt => {
                call_vector_reduction_intrinsic("llvm.vector.reduce.umin", val)
            }
        }
    }
}

impl<T> SumProdReducableTy for T
where
    T: VectorizableTy + MathTy,
    for<'a> T: ValTy<Type<'a>: SumProdReducable>,
{
}

#[expect(private_bounds)]
impl<'a, T, const N: usize> Val<'a, V<T, N>>
where
    T: SumProdReducableTy,
{
    pub fn sum(self) -> Val<'a, T> {
        T::call_sum(self)
    }
    pub fn prod(self) -> Val<'a, T> {
        T::call_prod(self)
    }
    pub fn max(self) -> Val<'a, T> {
        T::call_max(self)
    }
    pub fn min(self) -> Val<'a, T> {
        T::call_min(self)
    }
}
