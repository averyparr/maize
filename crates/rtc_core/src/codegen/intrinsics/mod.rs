// use inkwell::{types::BasicType, values::BasicValue};

// use crate::{codegen::CodegenModule, ty::Ty, val::Val};

// pub mod cuda;

// impl CodegenModule<'static> {
//     /// # Safety:
//     /// The intrinsic you're naming with `intrinsic_name` must be
//     /// a valid unary ntrinsic for types `T`.
//     pub unsafe fn call_unary_intrinsic<T: Ty>(
//         &self,
//         val: T::Value,
//         intrinsic_name: &str,
//     ) -> T::Value {
//         let ty = T::new(self.cx().ctx()).basic_ty();
//         let fn_ty = ty.fn_type(&[ty.as_basic_type_enum().into()], false);
//         let fn_val = self.module().add_function(intrinsic_name, fn_ty, None);

//         let call_site = unsafe {
//             self.cx().with_builder(|b| {
//                 b.build_call(
//                     fn_val,
//                     &[val.as_basic_value_enum().into()],
//                     "unary_intrinsic",
//                 )
//             })
//         }
//         .expect("Could not generate unary intrinsic call");

//         let ret_val = call_site
//             .try_as_basic_value()
//             .expect_basic("Must be a basic value!");

//         T::get_value(ret_val)
//     }

//     /// # Safety:
//     /// The intrinsic you're naming with `intrinsic_name` must be
//     /// a valid unary ntrinsic for types `T`.
//     pub unsafe fn call_unary_function<T: Ty>(
//         &self,
//         val: Val<'_, T>,
//         intrinsic_name: &str,
//     ) -> Val<'_, T> {
//         // Safety: See precondition above
//         let ret = unsafe { self.call_unary_intrinsic::<T>(val.to_underlying(), intrinsic_name) };
//         Val::new(self, ret.as_basic_value_enum())
//     }

//     /// # Safety:
//     /// The intrinsic you're naming with `intrinsic_name` must be
//     /// a valid binary intrinsic for types `T`.
//     pub unsafe fn call_binary_intrinsic<T: Ty>(
//         &self,
//         a: T::Value,
//         b: T::Value,
//         intrinsic_name: &str,
//     ) -> T::Value {
//         let ty = T::new(self.cx().ctx()).basic_ty();
//         let fn_ty = ty.fn_type(
//             &[
//                 ty.as_basic_type_enum().into(),
//                 ty.as_basic_type_enum().into(),
//             ],
//             false,
//         );
//         let fn_val = self.module().add_function(intrinsic_name, fn_ty, None);

//         let call_site = unsafe {
//             self.cx().with_builder(|builder| {
//                 builder.build_call(
//                     fn_val,
//                     &[
//                         a.as_basic_value_enum().into(),
//                         b.as_basic_value_enum().into(),
//                     ],
//                     "binary_intrinsic",
//                 )
//             })
//         }
//         .expect("Could not generate binary intrinsic call");

//         let ret_val = call_site
//             .try_as_basic_value()
//             .expect_basic("Must be a basic value!");

//         T::get_value(ret_val)
//     }

//     /// # Safety:
//     /// The intrinsic you're naming with `intrinsic_name` must be
//     /// a valid binary ntrinsic for types `T`.
//     pub unsafe fn call_binary_function<T: Ty>(
//         &self,
//         a: Val<'_, T>,
//         b: Val<'_, T>,
//         intrinsic_name: &str,
//     ) -> Val<'_, T> {
//         // Safety: See precondition above
//         let ret = unsafe {
//             self.call_binary_intrinsic::<T>(a.to_underlying(), b.to_underlying(), intrinsic_name)
//         };
//         Val::new(self, ret.as_basic_value_enum())
//     }
// }
