use inkwell::{context::ContextRef, types::BasicType};

use crate::backend::{ErasedType, FnRef, UntypedValue};

pub trait FnArgs {
    type ArgValues;
    fn raw_type_sequence(ctx: ContextRef<'static>) -> Vec<ErasedType>;
    fn extract_and_type_args(fn_ref: FnRef) -> Self::ArgValues;
    fn arg_arr(args: Self::ArgValues) -> impl IntoIterator<Item = UntypedValue>;
}

impl FnArgs for () {
    type ArgValues = ();
    fn raw_type_sequence(_: ContextRef<'static>) -> Vec<ErasedType> {
        vec![]
    }
    fn extract_and_type_args(fn_ref: FnRef) -> Self::ArgValues {
        let mut args = fn_ref.args();
        assert_eq!(args.next(), None);
    }
    fn arg_arr(_: Self::ArgValues) -> impl IntoIterator<Item = UntypedValue> {
        []
    }
}

macro_rules! derive_fn_args {
    ($($tipes: ident),*) => {
        impl<$($tipes: $crate::tipe::Ty),*> $crate::func::args::FnArgs for ($($tipes,)*) {
            type ArgValues = ($($crate::val::Val<$tipes>,)*);
            fn raw_type_sequence(ctx: ::inkwell::context::ContextRef<'static>) -> ::std::vec::Vec<$crate::backend::ErasedType> {
                vec![$($crate::backend::ErasedType($tipes::raw_ty(ctx).as_basic_type_enum())),*]
            }
            fn extract_and_type_args(fn_ref: FnRef) -> Self::ArgValues {
                let mut param = 0;
                $(
                    $tipes::type_metadata_on_function(&fn_ref, param);
                    param += 1;
                )*
                let _ = param;
                let mut args = fn_ref.args();
                #[allow(non_snake_case)]
                if $(let Some($tipes) = args.next() && )* let None = args.next() {
                    (
                        $(
                            // Safety: Raw construction from function args
                            unsafe {
                                $crate::val::Val::new(fn_ref.clone(), $tipes)
                            },
                        )*
                    )
                } else {
                    panic!("Incorrect argument sequence!");
                }
            }
            fn arg_arr(args: Self::ArgValues) -> impl IntoIterator<Item = UntypedValue> {
                #[allow(non_snake_case)]
                let ($($tipes,)*) = args;
                [$($tipes.raw(),)*]
            }
        }
    };
}

derive_fn_args!(A);
derive_fn_args!(A, B);
derive_fn_args!(A, B, C);
derive_fn_args!(A, B, C, D);
derive_fn_args!(A, B, C, D, E);
derive_fn_args!(A, B, C, D, E, F);
derive_fn_args!(A, B, C, D, E, F, G);
derive_fn_args!(A, B, C, D, E, F, G, H);
derive_fn_args!(A, B, C, D, E, F, G, H, I);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J);
