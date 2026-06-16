use crate::val::Val;

pub struct While<F: Fn() -> Val<bool>>(pub F);

impl<F: Fn() -> Val<bool>> While<F> {
    pub fn run(self, f: impl FnOnce()) {
        let mut init_val = self.0().with_storage();
        let val_ptr = init_val.get_mut().as_mut_ptr();
        let fn_ref = init_val.fn_ref();
        let init_block = fn_ref.get_current_bb();
        let head_block = fn_ref.append_bb("head");
        let bulk_block = fn_ref.append_bb("bulk");
        let tail_block = fn_ref.append_bb("tail");

        fn_ref.with_bb_as(init_block, || {
            // Safety: unconditional branch
            let b = unsafe { fn_ref.curr_bb_builder() };
            b.build_unconditional_branch(head_block)
                .expect("jmp should build");
        });

        fn_ref.with_bb_as(head_block, || {
            // Safety: just conditional jumping
            let b = unsafe { fn_ref.curr_bb_builder() };
            b.build_conditional_branch(
                unsafe { val_ptr.copy().read() }.typed(),
                bulk_block,
                tail_block,
            )
            .expect("Cond branch should have worked!");
        });

        fn_ref.with_bb_as(bulk_block, || {
            f();
            unsafe { val_ptr.write(self.0()) };
            // Safety: unconditional branch
            let b = unsafe { fn_ref.curr_bb_builder() };
            b.build_unconditional_branch(head_block)
                .expect("jmp should build");
        });
    }
}
