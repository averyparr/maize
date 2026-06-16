use inkwell::basic_block::BasicBlock;

use crate::{
    backend::{FnRef, UntypedValue},
    tipe::Ty,
    val::Val,
};

pub struct If(pub Val<bool>);
pub struct OrElse<T> {
    fn_ref: FnRef,
    then_block: BasicBlock<'static>,
    else_block: BasicBlock<'static>,
    dest_block: BasicBlock<'static>,
    then_result: T,
}

impl<T> Drop for OrElse<T> {
    fn drop(&mut self) {
        self.fn_ref.with_bb_as(self.else_block, || {
            // Safety: we build an unconditional branch
            let b = unsafe { self.fn_ref.curr_bb_builder() };
            b.build_unconditional_branch(self.dest_block)
                .expect("Should be able to build jmp");
        })
    }
}

impl If {
    pub fn then<T>(self, f: impl FnOnce() -> T) -> OrElse<T> {
        let fn_ref = self.0.fn_ref().clone();
        let init_block = fn_ref.get_current_bb();
        let mut then_block = fn_ref.append_bb("then");
        let else_block = fn_ref.append_bb("else");
        let dest_block = fn_ref.append_bb("dest");
        fn_ref.with_bb_as(init_block, || {
            // Safety: we build a conditional branch
            let b = unsafe { fn_ref.curr_bb_builder() };
            b.build_conditional_branch(self.0.typed(), then_block, else_block)
                .expect("Should be able to build jmp");
        });
        let then_result = fn_ref.with_bb_as(then_block, || {
            let ret = f();
            then_block = fn_ref.get_current_bb(); // in case it changed which block we're on
            // Safety: we build an unconditional branch
            let b = unsafe { fn_ref.curr_bb_builder() };
            b.build_unconditional_branch(dest_block)
                .expect("Should be able to build jmp");
            ret
        });
        OrElse {
            fn_ref,
            then_block,
            else_block,
            dest_block,
            then_result,
        }
    }
}

impl<T> OrElse<Val<T>> {
    pub fn or_else(mut self, f: impl FnOnce() -> Val<T>) -> Val<T>
    where
        T: Ty,
    {
        let fn_ref = self.fn_ref.clone();
        let else_result = fn_ref
            .with_bb_as(self.else_block, || {
                let ret = f();
                self.else_block = fn_ref.get_current_bb(); // in case we changed it
                ret
            })
            .raw()
            .0;
        let then_result = self.then_result.raw().0;
        let ret = fn_ref.with_bb_as(self.dest_block, || {
            let b = unsafe { fn_ref.curr_bb_builder() };
            let phi = b
                .build_phi(T::ty(&fn_ref).0, "phi")
                .expect("Should be able to build phi node");
            phi.add_incoming(&[
                (&then_result, self.then_block),
                (&else_result, self.else_block),
            ]);
            unsafe { Val::new(fn_ref.clone(), UntypedValue::new(phi.as_basic_value())) }
        });
        ret
    }
}

impl OrElse<()> {
    pub fn or_else(mut self, f: impl FnOnce()) {
        self.fn_ref.with_bb_as(self.else_block, || {
            f();
            self.else_block = self.fn_ref.get_current_bb();
        });
    }
}
