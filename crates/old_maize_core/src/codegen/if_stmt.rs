use inkwell::basic_block::BasicBlock;

use crate::{
    codegen::FnCodegen,
    ty::{Bool, ValTy},
    val::Val,
};

pub struct ThenNoVal<'a> {
    cx: &'a FnCodegen,
    then_bb: BasicBlock<'static>,
    else_bb: BasicBlock<'static>,
    uni_bb: BasicBlock<'static>,
}

pub struct Then<'a, Ret: ValTy> {
    val_from_if: Val<'a, Ret>,
    raw: ThenNoVal<'a>,
}

impl<'a> Drop for ThenNoVal<'a> {
    fn drop(&mut self) {
        jump_from_block_to_other_block(self.else_bb, self.uni_bb);
    }
}

impl<'a, Ret: ValTy> Then<'a, Ret> {
    pub fn or(self, val: Val<'a, Ret>) -> Val<'a, Ret> {
        self.or_else(|| val)
    }
    pub fn or_else(mut self, f: impl FnOnce() -> Val<'a, Ret>) -> Val<'a, Ret> {
        let cx = self.val_from_if.cx();

        let (else_end_bb, else_ret) = cx.with_bb_as(self.raw.else_bb, f);

        let (uni_bb, raw_ret) = cx.with_bb_as(self.raw.uni_bb, || {
            let phi_ty = Ret::ty(cx.ctx());
            let phi_val = unsafe {
                cx.with_builder(|b| b.build_phi(phi_ty, "if_else_phi"))
                    .expect("phi from if/then/else should be able to be created")
            };
            phi_val.add_incoming(&[
                (&self.val_from_if.ll_typed(), self.raw.then_bb),
                (&else_ret.ll_typed(), else_end_bb),
            ]);
            phi_val
        });

        // The else function may have changed the basic blocks at the "end" of each of these,
        // so update
        self.raw.else_bb = else_end_bb;
        self.raw.uni_bb = uni_bb;

        // Should drop self which drops ThenNoVal which in turn unconditionally jumps from else to uni
        unsafe { Val::new(else_ret.cx(), raw_ret.as_basic_value()) }
    }
}

impl<'a> ThenNoVal<'a> {
    pub fn or(self) {
        // Drop self
    }
    pub fn or_else(mut self, f: impl FnOnce()) {
        let cx = self.cx;
        let (end_else_bb, _) = cx.with_bb_as(self.else_bb, f);
        self.else_bb = end_else_bb;
        // Should drop self which unconditionally jumps from else to uni
    }
}

fn jump_from_block_to_other_block<'ctx>(from: BasicBlock<'ctx>, to: BasicBlock<'ctx>) {
    if from.get_terminator().is_none() {
        let ctx = from.get_context();
        let builder = ctx.create_builder();
        builder.position_at_end(from);
        builder
            .build_unconditional_branch(to)
            .expect("Must be able to build unconditional branch between blocks");
    }
}

fn setup_branch_on<'ctx>(
    cond: Val<'ctx, Bool>,
) -> (
    BasicBlock<'static>, // then
    BasicBlock<'static>, // else
) {
    let cx = cond.cx();
    let false_val = cx.constant_from(false);
    let comp = unsafe {
        cx.with_builder(|b| {
            b.build_int_compare(
                inkwell::IntPredicate::NE,
                cond.ll_typed(),
                false_val.ll_typed(),
                "icmp",
            )
        })
    }
    .expect("icmp between bools should succeed");
    let then_block = cx.ctx().append_basic_block(cx.func(), "then_bb");
    let else_block = cx.ctx().append_basic_block(cx.func(), "else_bb");

    let _jmp =
        unsafe { cx.with_builder(|b| b.build_conditional_branch(comp, then_block, else_block)) }
            .expect("jne on an icmp should succeed");
    (then_block, else_block)
}

impl<'a> Val<'a, Bool> {
    fn then_inner<U>(&self, f: impl FnOnce() -> U) -> (ThenNoVal<'a>, U) {
        let cx = self.cx();
        let (then_bb, else_bb) = setup_branch_on(*self);
        let uni_bb = cx.ctx().append_basic_block(cx.func(), "post_if");

        // `f` is allowed to change the basic block so we must get a new copy
        let (then_bb, ret) = cx.with_bb_as(then_bb, f);
        jump_from_block_to_other_block(then_bb, uni_bb);
        cx.set_bb(uni_bb);

        let then_obj = ThenNoVal {
            cx,
            then_bb,
            else_bb,
            uni_bb,
        };
        (then_obj, ret)
    }

    pub fn branch<If: FnOnce()>(&self, f: If) -> ThenNoVal<'a> {
        let (ret, _rest) = self.then_inner(f);
        ret
    }
    pub fn then<Ret: ValTy>(self, f: impl FnOnce() -> Val<'a, Ret>) -> Then<'a, Ret> {
        let (raw, val_from_if) = self.then_inner(f);
        Then { val_from_if, raw }
    }
}
