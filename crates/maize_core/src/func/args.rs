use inkwell::types::BasicType;

use crate::{
    ContextRef,
    backend::{ErasedType, FnRef, UntypedValue},
};

pub trait FnArgs {
    type ArgValues;
    fn raw_type_sequence(ctx: ContextRef) -> Vec<ErasedType>;
    fn extract_and_type_args(fn_ref: FnRef) -> Self::ArgValues;
    fn arg_arr(args: Self::ArgValues) -> impl IntoIterator<Item = UntypedValue>;
}

impl FnArgs for () {
    type ArgValues = ();
    fn raw_type_sequence(_: ContextRef) -> Vec<ErasedType> {
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
            fn raw_type_sequence(ctx: ContextRef) -> ::std::vec::Vec<$crate::backend::ErasedType> {
                Vec::from_iter(::std::iter::empty()$(.chain($tipes::function_arg_types(ctx)))*)
            }
            fn extract_and_type_args(fn_ref: FnRef) -> Self::ArgValues {
                let mut params = 0..;
                $(
                    $tipes::type_metadata_on_function(&fn_ref, &mut params);
                )*
                let nparams = params.next().unwrap();
                let mut args = Vec::from_iter(fn_ref.args()).into_iter();

                let ret = unsafe { (
                    $($tipes::extract_arg(fn_ref.clone(), &mut args),)*
                ) };

                let None = args.next() else {
                    panic!("Incorrect argument sequence!");
                };

                ret
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
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
derive_fn_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
derive_fn_args!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
derive_fn_args!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
derive_fn_args!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
derive_fn_args!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
derive_fn_args!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
derive_fn_args!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
