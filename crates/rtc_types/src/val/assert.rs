#[macro_export]
macro_rules! kernel_assert {
    ($cond: expr, $msg: literal) => {{
        let cond: $crate::val::Val<'_, Bool> = $cond.copy();
        fn f() {}
        // ::[...]::above_fn_name::f
        let mut name = ::core::any::type_name_of_val(&f);
        name = &name[..name.len() - 3];
        cond.cx()
            .intrinsics()
            .assert(cond.copy(), $msg, file!(), line!(), name)
    }};
    ($cond: expr) => {
        kernel_assert!($cond, "Assertion failed: ")
    };
}
