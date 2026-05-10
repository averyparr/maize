use inkwell::values::BasicValueEnum;

use crate::{
    ty::{AnyTy, ValTy, raw::*, vec::VectorizableTy},
    val::Val,
};

pub trait PrintableTy: ValTy {
    fn format_str() -> impl AsRef<str>;
    fn to_flat_va_args(val: Val<'_, Self>) -> impl Iterator<Item = BasicValueEnum<'static>>;
}

macro_rules! printable_primitive {
    ($($tipe: ty => $fmt_str: literal),*$(,)?) => {
        $(
            impl PrintableTy for $tipe {
                fn format_str() -> impl AsRef<str> {
                    $fmt_str
                }
                fn to_flat_va_args(val: Val<'_, Self>) -> impl Iterator<Item = BasicValueEnum<'static>> {
                    [val.raw()].into_iter()
                }
            }
        )*
    };
}

printable_primitive!(
    BF16 => "bf16_%#.4g",
    F16 => "f16_%#.5g",
    F32 => "%#.9g",
    F64 =>"%#.17g",
    I8 => "%hhd",
    I16 => "%hd",
    I32 => "%d",
    I64 => "%lld",
    U8 => "%hhu",
    U16 => "%hu",
    U32 => "%u",
    U64 => "%llu",
);

impl<T: AnyTy> PrintableTy for P<*const T> {
    fn format_str() -> impl AsRef<str> {
        "%p"
    }
    fn to_flat_va_args(val: Val<'_, Self>) -> impl Iterator<Item = BasicValueEnum<'static>> {
        [val.raw()].into_iter()
    }
}
impl<T: AnyTy> PrintableTy for P<*mut T> {
    fn format_str() -> impl AsRef<str> {
        "%p"
    }
    fn to_flat_va_args(val: Val<'_, Self>) -> impl Iterator<Item = BasicValueEnum<'static>> {
        [val.raw()].into_iter()
    }
}

impl<T: PrintableTy + VectorizableTy, const N: usize> PrintableTy for V<T, N> {
    fn format_str() -> impl AsRef<str> {
        let elem_format = T::format_str();
        let elem_format = elem_format.as_ref();
        let mut full = String::with_capacity(3 + (elem_format.len() + 1) * N);
        full.push_str("V[");
        for _ in 0..N {
            full.push_str(elem_format);
            full.push(',');
        }
        if full.ends_with(',') {
            full.pop();
        }
        full.push(']');
        full
    }

    fn to_flat_va_args(val: Val<'_, Self>) -> impl Iterator<Item = BasicValueEnum<'static>> {
        val.elements()
            .into_iter()
            .map(|v| T::to_flat_va_args(v))
            .flatten()
    }
}

impl<T: PrintableTy, const N: usize> PrintableTy for [T; N]
where
    T: VectorizableTy,
{
    fn format_str() -> impl AsRef<str> {
        let elem_format = T::format_str();
        let elem_format = elem_format.as_ref();
        let mut full = String::with_capacity(3 + (elem_format.len() + 1) * N);
        full.push_str("[");
        for _ in 0..N {
            full.push_str(elem_format);
            full.push(',');
        }
        if full.ends_with(',') {
            full.pop();
        }
        full.push(']');
        full
    }

    fn to_flat_va_args(val: Val<'_, Self>) -> impl Iterator<Item = BasicValueEnum<'static>> {
        val.array_elements()
            .into_iter()
            .map(|v| T::to_flat_va_args(v))
            .flatten()
    }
}

pub fn to_va_args<'a, P: PrintableTy>(
    val: Val<'a, P>,
) -> impl Iterator<Item = BasicValueEnum<'static>> {
    P::to_flat_va_args(val)
}
pub fn format_str_of_val<'a, P: PrintableTy>(_: &Val<'a, P>) -> impl AsRef<str> {
    P::format_str()
}

pub const fn check_string_for_escapes(val: &str) {
    if count_percent_vals(val) != 0 {
        panic!(
            "Due to vprintf limitations, formatting strings cannot contain '%' symbols as they lead to undefined stack reads"
        );
    }
}

const fn count_percent_vals(val: &str) -> usize {
    let byte_slice = val.as_bytes();
    let percent = b'%';
    let mut idx = 0;
    let mut count = 0;
    while idx < byte_slice.len() {
        if byte_slice[idx] == percent {
            count += 1;
        }
        idx += 1;
    }
    count
}

pub const fn count_format_args(val: &str) -> usize {
    let byte_slice = val.as_bytes();
    let open = b'{';
    let close = b'}';
    let mut idx = 0;
    let mut count = 0;
    while idx < byte_slice.len() {
        if byte_slice[idx] == open {
            if idx + 1 == byte_slice.len() {
                panic!("String ended with '{{'");
            }
            if byte_slice[idx + 1] != close {
                panic!(
                    r"Due to formatting limitations, all arguments must be formatted with '{{}}'"
                );
            }
            count += 1;
        }
        idx += 1;
    }
    count
}

#[macro_export]
macro_rules! _count_args {
    () => { 0 };
    ($first:expr $(, $rest:expr)*) => { 1 + $crate::_count_args!($($rest),*) }
}
#[macro_export]
macro_rules! kernel_print {
    ($raw_str: literal) => {{
        const RAW_STR: &str = $raw_str;
        const _: () = $crate::ty::printable::check_string_for_escapes(RAW_STR);
        const FMT_ARG_COUNT: usize = $crate::ty::printable::count_format_args(RAW_STR);
        const ARG_COUNT: usize = 0;
        const _: () = {
            assert!(ARG_COUNT == FMT_ARG_COUNT, "Please ensure the number of formatting {{}} is the same as the number of variadic args you passed");
        };
        compile_error!(
            "Because we must function calls, please provide a value on which to call `.cx()`. If you don't want formatting, use `kernel_print!($message => $cx);`"
        );
    }};
    ($raw_str: literal => $cx: expr) => {{
        let cx: &$crate::codegen::typed_func::FnCodegen = $cx;
        const RAW_STR: &str = $raw_str;
        const _: () = $crate::ty::printable::check_string_for_escapes(RAW_STR);
        const FMT_ARG_COUNT: usize = $crate::ty::printable::count_format_args(RAW_STR);
        const ARG_COUNT: usize = 0;
        const _: () = {
            assert!(ARG_COUNT == FMT_ARG_COUNT, "Please ensure the number of formatting {{}} is the same as the number of variadic args you passed");
        };
        let fmt_str = cx.insert_str(RAW_STR, None, "fmt_str");
        let vprint_fn_val = cx.declare_function::<
            $crate::ty::Void,
            (
                $crate::ty::P<*const $crate::ty::U8>,
                $crate::ty::P<*mut $crate::ty::Void>,
            )
        >("vprintf");
        cx.call_void_fn(vprint_fn_val, (fmt_str, Val::zeros(cx)))

    }};
    ($raw_str: literal, $first_arg: expr$(,$args: expr)*$(,)?) => {{
        const RAW_STR: &str = $raw_str;
        const _: () = $crate::ty::printable::check_string_for_escapes(RAW_STR);
        const FMT_ARG_COUNT: usize = $crate::ty::printable::count_format_args(RAW_STR);
        const ARG_COUNT: usize = $crate::_count_args!($first_arg $(,$args)*);
        const _: () = {
            assert!(ARG_COUNT == FMT_ARG_COUNT, "Please ensure the number of formatting {{}} is the same as the number of variadic args you passed");
        };

        let mut full_fmt_str = String::new();
        let mut all_values = Vec::new();
        let mut nonformat_frags = RAW_STR.split("{}");
        let mut next_nonformat_frag = ||{
            nonformat_frags.next().expect("Should be N+1 fragments for N args")
        };

        full_fmt_str.push_str(next_nonformat_frag());
        full_fmt_str.push_str($crate::ty::printable::format_str_of_val(::core::borrow::Borrow::borrow(&$first_arg)).as_ref());
        all_values.extend($crate::ty::printable::to_va_args($first_arg));

        $(
            full_fmt_str.push_str(next_nonformat_frag());
            full_fmt_str.push_str($crate::ty::printable::format_str_of_val(::core::borrow::Borrow::borrow(&$args)).as_ref());
            all_values.extend($crate::ty::printable::to_va_args($args));
        )*
        full_fmt_str.push_str(next_nonformat_frag());

        let cx = $first_arg.cx();
        let fmt_str = cx.insert_str(&full_fmt_str, None, "fmt_str");
        let va_ptr = cx.store_vals_in_struct_alloca(&all_values, true);
        let vprint_fn_val = cx.declare_function::<
            $crate::ty::Void,
            (
                $crate::ty::P<*const $crate::ty::U8>,
                $crate::ty::P<*mut $crate::ty::Void>,
            )
        >("vprintf");
        cx.call_void_fn(vprint_fn_val, (fmt_str, va_ptr));
    }};
}
