use inkwell::{
    intrinsics::Intrinsic,
    types::{AnyType, BasicType, FloatType, IntType, VectorType},
    values::{AnyValue, BasicValue},
};

use crate::{ty::ValTy, val::Val};

enum AbsVariant {
    Int,
    Float,
    Vector,
}

trait AbsAble {
    const ABS_TYPE: AbsVariant;
}

impl AbsAble for IntType<'_> {
    const ABS_TYPE: AbsVariant = AbsVariant::Int;
}

impl AbsAble for FloatType<'_> {
    const ABS_TYPE: AbsVariant = AbsVariant::Float;
}

impl AbsAble for VectorType<'_> {
    const ABS_TYPE: AbsVariant = AbsVariant::Vector;
}

trait AbsAbleTy: for<'a> ValTy<Type<'a>: AbsAble> {
    fn call_abs(val: Val<'_, Self>) -> Val<'_, Self> {
        let abs_intrinsic_name = match Self::Type::ABS_TYPE {
            AbsVariant::Int => "llvm.abs",
            AbsVariant::Float => "llvm.fabs",
            AbsVariant::Vector => {
                let element_ty = Self::ty(val.ctx())
                    .as_any_type_enum()
                    .into_vector_type()
                    .get_element_type();
                if element_ty.is_int_type() {
                    "llvm.abs"
                } else if element_ty.is_float_type() {
                    "llvm.fabs"
                } else {
                    panic!("Unable to match abs against {element_ty:?}");
                }
            }
        };

        let intrinsic =
            Intrinsic::find(abs_intrinsic_name).expect("This should be a valid LLVM intrinsic");
        let function = intrinsic
            .get_declaration(
                val.cx().module(),
                &[Self::ty(val.ctx()).as_basic_type_enum().into()],
            )
            .expect("There should be a function value with this signature");

        let raw_ret = Self::type_val(
            unsafe {
                val.cx().with_builder(|b| {
                    b.build_call(
                        function,
                        &[val.get_ll_typed().as_basic_value_enum().into()],
                        "call_abs",
                    )
                })
            }
            .expect("Call build should succeed")
            .as_any_value_enum(),
        );

        if let Some(ins) = raw_ret.as_instruction_value() {
            val.cx().apply_ins_opt(ins);
        }

        unsafe { Val::new_from_value(val.cx(), raw_ret.as_basic_value_enum()) }
    }
}

impl<T> AbsAbleTy for T where for<'a> T: ValTy<Type<'a>: AbsAble> {}

#[expect(private_bounds)]
impl<T> Val<'_, T>
where
    T: AbsAbleTy,
{
    pub fn abs(self) -> Self {
        T::call_abs(self)
    }
}
