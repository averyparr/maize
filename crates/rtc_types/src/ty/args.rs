#![allow(non_snake_case)]

use inkwell::{
    attributes::AttributeLoc,
    context::ContextRef,
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum},
    values::{BasicMetadataValueEnum, BasicValue},
};

use crate::codegen::FnCodegen;
use crate::{ty::SizedTy, val::Val};

pub trait IntoFuncArgs {
    fn arg_aligns() -> impl AsRef<[u32]>;
    fn produce_args<'ctx>(ctx: ContextRef<'ctx>) -> impl AsRef<[BasicTypeEnum<'ctx>]>;
    fn produce_metadata_args<'ctx>(
        ctx: ContextRef<'ctx>,
    ) -> impl AsRef<[BasicMetadataTypeEnum<'ctx>]>;
    type ArgValues<'ctx>;
    fn try_extract_args<'a>(cx: &'a FnCodegen) -> Option<Self::ArgValues<'a>>;
    fn args_to_raw(args: Self::ArgValues<'_>) -> impl AsRef<[BasicMetadataValueEnum<'static>]>;
}

macro_rules! impl_into_func_args {
    ($($names: ident => $idx: literal;)*) => {
        impl<$($names),*> IntoFuncArgs for ($($names,)*)
        where
            $($names: SizedTy,)*
        {
            fn arg_aligns() -> impl AsRef<[u32]> {
                [$($names::ALIGN,)*]
            }
            fn produce_args<'ctx>(ctx: ContextRef<'ctx>) -> impl AsRef<[BasicTypeEnum<'ctx>]> {
                [$(
                    $names::ty(ctx).as_basic_type_enum(),
                )*]
            }
            fn produce_metadata_args<'ctx>(ctx: ContextRef<'ctx>) -> impl AsRef<[BasicMetadataTypeEnum<'ctx>]> {
                [$(
                    $names::ty(ctx).as_basic_type_enum().into(),
                )*]
            }
            type ArgValues<'ctx> = ($(Val<'ctx, $names>,)*);
            fn try_extract_args<'a>(cx: &'a FnCodegen) -> Option<Self::ArgValues<'a>> {
                let ctx = cx.ctx();
                let func = cx.func();
                let types_of_params = Self::produce_args(ctx);
                for (param, proposed_type) in func.get_param_iter().zip(types_of_params.as_ref().iter()) {
                    if BasicMetadataTypeEnum::from(param.get_type()) != (*proposed_type).into() {
                        return None;
                    }
                }
                let aligns = Self::arg_aligns();
                let aligns = aligns.as_ref();
                $(
                    for attr in $names::fn_arg_attrs(ctx).into_iter() {
                        func.add_attribute(AttributeLoc::Param($idx), attr);
                    }
                )*
                if func.count_params() as usize != aligns.len() {
                    return None;
                }
                // Safety: We have checked the LLVM types of all arguments to the function,
                // so we can be confident that the type-unsafe cast here is nonetheless correct
                unsafe {
                    Some(($(
                        Val::<'a, $names>::new_from_value(
                            cx,
                            func.get_nth_param($idx)
                                .expect("Param number mismatch")
                        ),
                    )*))
                }
            }
            fn args_to_raw(args: Self::ArgValues<'_>) -> impl AsRef<[BasicMetadataValueEnum<'static>]> {
                let ($($names,)*) = args;
                [$(
                    $names.get_ll_typed().as_basic_value_enum().into(),
                )*]
            }
        }
    };
}

impl_into_func_args!(
    A => 0;
);
impl_into_func_args!(
    A => 0;
    B => 1;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
);
impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
    U => 20;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
    U => 20;
    V => 21;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
    U => 20;
    V => 21;
    W => 22;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
    U => 20;
    V => 21;
    W => 22;
    X => 23;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
    U => 20;
    V => 21;
    W => 22;
    X => 23;
    Y => 24;
);

impl_into_func_args!(
    A => 0;
    B => 1;
    C => 2;
    D => 3;
    E => 4;
    F => 5;
    G => 6;
    H => 7;
    I => 8;
    J => 9;
    K => 10;
    L => 11;
    M => 12;
    N => 13;
    O => 14;
    P => 15;
    Q => 16;
    R => 17;
    S => 18;
    T => 19;
    U => 20;
    V => 21;
    W => 22;
    X => 23;
    Y => 24;
    Z => 25;
);
