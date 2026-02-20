use inkwell::{
    attributes::{Attribute, AttributeLoc},
    context::ContextRef,
    types::{BasicMetadataTypeEnum, BasicType},
    values::{AnyValue, FunctionValue},
};

use crate::codegen::FnCodegen;
use crate::{ty::SizedTy, val::Val};

pub trait IntoFuncArgs {
    fn arg_aligns() -> impl AsRef<[usize]>;
    fn produce_args<'ctx>(ctx: ContextRef<'ctx>) -> impl AsRef<[BasicMetadataTypeEnum<'ctx>]>;
    type ArgValues<'ctx>;
    fn try_extract_args<'a>(cx: &'a FnCodegen) -> Option<Self::ArgValues<'a>>;
}

macro_rules! impl_into_func_args {
    ($($names: ident => $idx: literal;)*) => {
        impl<$($names),*> IntoFuncArgs for ($($names,)*)
        where
            $($names: SizedTy,)*
        {
            fn arg_aligns() -> impl AsRef<[usize]> {
                [$($names::ALIGN,)*]
            }
            fn produce_args<'ctx>(ctx: ContextRef<'ctx>) -> impl AsRef<[BasicMetadataTypeEnum<'ctx>]> {
                [$(
                    $names::ty(ctx).as_basic_type_enum().into(),
                )*]
            }
            type ArgValues<'ctx> = ($(Val<'ctx, $names>,)*);
            fn try_extract_args<'a>(cx: &'a FnCodegen) -> Option<Self::ArgValues<'a>> {
                let align_kind_id = Attribute::get_named_enum_kind_id("align");
                let ctx = cx.ctx();
                let func = cx.func();
                let types_of_params = Self::produce_args(ctx);
                for (param, proposed_type) in func.get_param_iter().zip(types_of_params.as_ref().iter()) {
                    if BasicMetadataTypeEnum::from(param.get_type()) != *proposed_type {
                        return None;
                    }
                }
                let aligns = Self::arg_aligns();
                let aligns = aligns.as_ref();
                for (idx, &align) in aligns.iter().enumerate() {
                    let attribute = ctx.create_enum_attribute(align_kind_id, align as u64);
                    func.add_attribute(AttributeLoc::Param(idx as u32), attribute);
                }
                if func.count_params() as usize != aligns.len() {
                    return None;
                }
                // Safety: We have checked the LLVM types of all arguments to the function,
                // so we can be confident that the type-unsafe cast here is nonetheless correct
                unsafe {
                    Some(($(
                        Val::<'a, $names>::new(
                            cx,
                            func.get_nth_param($idx)
                                .expect("Param number mismatch")
                                .as_any_value_enum(),
                        ),
                    )*))
                }
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
