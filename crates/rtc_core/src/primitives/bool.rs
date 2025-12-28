use crate::{ty::Bool, val::Val};

impl<'lt> Val<'lt, Bool> {
    fn call_if<IfFn: FnOnce()>(&self, if_fn: IfFn) {
        self.cx().with_branch(if_fn, || {});
    }
    fn if_else<IfFn: FnOnce(), ElseFn: FnOnce()>(&self, if_fn: IfFn, else_fn: ElseFn) {
        self.cx().with_branch(if_fn, else_fn);
    }
}
