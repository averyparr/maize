#[macro_export]
macro_rules! jit_assert {
    ($cond: expr) => {{
        fn __assert(cond: $crate::val::Val<bool>) {
            fn f() {}
            let function = std::any::type_name_of_val(&f)
                .strip_suffix("::f")
                .unwrap()
                .split("::")
                .last()
                .expect("jit_assert called from outside a function?");
            let mut message = String::from(concat!("Assertion failed: ", stringify!($cond)));
            let use_backtrace = ::std::env::var("RUST_BACKTRACE")
                .map(|v| v == "1")
                .unwrap_or(false);
            if use_backtrace {
                message.push_str("\nBacktrace:\n");
                message.push_str(&std::backtrace::Backtrace::force_capture().to_string());
                println!("{}", std::backtrace::Backtrace::force_capture().to_string());
            }
            cond.fn_ref().clone().with_intrinsic(|lib| {
                lib.assert(cond.clone(), &message, file!(), line!(), function)
            })
        }
        __assert($cond);
    }};
}
