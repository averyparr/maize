use inkwell::context::{Context, ContextRef};

pub fn create_context() -> ContextRef<'static> {
    thread_local! {
        static CONTEXT: Context = Context::create()
    }
    // SAFETEY: We are referencing a now-initialized Context object
    unsafe { ContextRef::new(CONTEXT.with(|c| c.raw())) }
}
