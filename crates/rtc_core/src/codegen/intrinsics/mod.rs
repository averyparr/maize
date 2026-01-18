use inkwell::{
    types::BasicType,
    values::{BasicValue, CallSiteValue},
};

use crate::{
    codegen::{CodegenModule, func_with_args::IntoFuncArgs},
    ty::Ty,
    val::Val,
};

pub mod cuda;

impl CodegenModule<'static> {
    /// # Safety:
    /// The intrinsic you're naming with `intrinsic_name` must be
    /// a valid intrinsic name, and the arguments you provide to it
    /// must be valid. There are _probably_ other safety
    /// preconditions, but I cannot think of them right now.
    pub unsafe fn call_voidlike_intrinsic<'lt, Args>(
        &self,
        args: Args::ArgValues<'lt>,
        intrinsic_name: &str,
    ) where
        Args: IntoFuncArgs,
    {
        let void_ty = self.cx().ctx().void_type();
        let arg_types: Vec<_> = Args::new_from_ctx(self.cx().ctx())
            .basic_ty_iter()
            .map(|t| t.into())
            .collect();
        let args: Vec<_> = Args::basic_val_iter(args).map(|a| a.into()).collect();

        let fn_ty = void_ty.fn_type(&arg_types, false);
        let fn_val = self.module().add_function(intrinsic_name, fn_ty, None);

        let _: CallSiteValue<'_> = unsafe {
            self.cx()
                .with_builder(|b| b.build_call(fn_val, &args, "voidlike_intrinsic"))
        }
        .expect("Could not generate voidlike intrinsic");
    }
    /// # Safety:
    /// The intrinsic you're naming with `intrinsic_name` must be
    /// a valid unary ntrinsic for types `T`.
    pub unsafe fn call_unary_intrinsic<T: Ty>(
        &self,
        val: T::Value,
        intrinsic_name: &str,
    ) -> T::Value {
        let ty = T::new(self.cx().ctx()).basic_ty();
        let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
        let fn_val = self.module().add_function(intrinsic_name, fn_ty, None);

        let call_site = unsafe {
            self.cx().with_builder(|b| {
                b.build_call(
                    fn_val,
                    &[val.as_basic_value_enum().into()],
                    "unary_intrinsic",
                )
            })
        }
        .expect("Could not generate unary intrinsic call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        T::get_value(ret_val)
    }

    /// # Safety:
    /// The intrinsic you're naming with `intrinsic_name` must be
    /// a valid unary intrinsic for types `T`.
    pub unsafe fn call_unary_function<T: Ty>(
        &self,
        val: Val<'_, T>,
        intrinsic_name: &str,
    ) -> Val<'_, T> {
        // Safety: See precondition above
        unsafe {
            Val::new(
                self,
                self.call_unary_intrinsic::<T>(val.to_underlying(), intrinsic_name),
            )
        }
    }

    /// # Safety:
    /// The intrinsic you're naming with `intrinsic_name` must be
    /// a valid binary intrinsic for types `T`.
    pub unsafe fn call_binary_intrinsic<T: Ty>(
        &self,
        a: T::Value,
        b: T::Value,
        intrinsic_name: &str,
    ) -> T::Value {
        let ty = T::new(self.cx().ctx()).basic_ty();
        let fn_ty = ty.fn_type(
            &[
                ty.as_basic_type_enum().into(),
                ty.as_basic_type_enum().into(),
            ],
            false,
        );
        let fn_val = self.module().add_function(intrinsic_name, fn_ty, None);

        let call_site = unsafe {
            self.cx().with_builder(|builder| {
                builder.build_call(
                    fn_val,
                    &[
                        a.as_basic_value_enum().into(),
                        b.as_basic_value_enum().into(),
                    ],
                    "binary_intrinsic",
                )
            })
        }
        .expect("Could not generate binary intrinsic call");

        let ret_val = call_site
            .try_as_basic_value()
            .expect_basic("Must be a basic value!");

        T::get_value(ret_val)
    }

    /// # Safety:
    /// The intrinsic you're naming with `intrinsic_name` must be
    /// a valid binary ntrinsic for types `T`.
    pub unsafe fn call_binary_function<T: Ty>(
        &self,
        a: Val<'_, T>,
        b: Val<'_, T>,
        intrinsic_name: &str,
    ) -> Val<'_, T> {
        // Safety: See precondition above
        unsafe {
            Val::new(
                self,
                self.call_binary_intrinsic::<T>(
                    a.to_underlying(),
                    b.to_underlying(),
                    intrinsic_name,
                ),
            )
        }
    }
}

/// Safety: You must promise that INTRINSIC_NAME
/// is valid for values of type T as a unary intrinsic.
/// You must also not implement anything other than
/// `INTRINSIC_NAME`
pub unsafe trait UnaryIntrinsic<T: Ty> {
    const INTRINSIC_NAME: &str;

    fn call_intrinsic(val: Val<'_, T>) -> Val<'_, T> {
        // Safety: User promised!
        unsafe { val.cm().call_unary_function(val, Self::INTRINSIC_NAME) }
    }
}

/// Safety: You must promise that INTRINSIC_NAME
/// is valid for values of type T as a binary intrinsic.
/// You must also not implement anything other than
/// `INTRINSIC_NAME`
pub unsafe trait BinaryIntrinsic<T: Ty> {
    const INTRINSIC_NAME: &str;

    fn call_intrinsic<'a>(lhs: Val<'a, T>, rhs: Val<'a, T>) -> Val<'a, T> {
        unsafe {
            lhs.cm()
                .call_binary_function(lhs, rhs, Self::INTRINSIC_NAME)
        }
    }
}
