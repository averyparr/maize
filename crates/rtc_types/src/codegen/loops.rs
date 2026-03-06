use core::ops::Range;

use inkwell::basic_block::BasicBlock;

use crate::{
    codegen::FnCodegen,
    ty::{Bool, SizedTy, U32},
    val::{S, Val},
};

pub trait TransformLooper {
    type DecisionItemT: SizedTy + Copy;
    type ItemT<'a>
    where
        Self: 'a;
    fn cx<'a>(&self) -> &'a FnCodegen
    where
        Self: 'a;
    fn init_decision<'a>(&self) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a;
    fn decision_fn<'a>(&self, decision_val: Val<'a, Self::DecisionItemT>) -> Val<'a, Bool>
    where
        Self: 'a;
    fn transform<'a>(&self, decision_val: Val<'a, Self::DecisionItemT>) -> Self::ItemT<'a>
    where
        Self: 'a;
    fn update_fn<'a>(
        &self,
        decision_val: Val<'a, Self::DecisionItemT>,
    ) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a;
}

impl<'ctx> TransformLooper for Range<Val<'ctx, U32>> {
    type DecisionItemT = U32;
    type ItemT<'a>
        = Val<'a, U32>
    where
        Self: 'a;
    fn cx<'a>(&self) -> &'a FnCodegen
    where
        Self: 'a,
    {
        self.start.cx()
    }
    fn init_decision<'a>(&self) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        self.start
    }
    fn decision_fn<'a>(&self, decision_val: Val<'a, Self::DecisionItemT>) -> Val<'a, Bool>
    where
        Self: 'a,
    {
        decision_val.lt(self.end)
    }
    fn transform<'a>(&self, decision_val: Val<'a, Self::DecisionItemT>) -> Self::ItemT<'a>
    where
        Self: 'a,
    {
        decision_val
    }
    fn update_fn<'a>(
        &self,
        decision_val: Val<'a, Self::DecisionItemT>,
    ) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        self.cx().constant_from(1u32) + decision_val
    }
}

enum LoopStages<'a, L: TransformLooper> {
    PreLoop,
    AfterLoop(
        Val<'a, S<L::DecisionItemT>>,
        BasicBlock<'static>,
        BasicBlock<'static>,
    ),
    Done,
}
pub struct Loop<'a, L: TransformLooper> {
    looper: L,
    stage: LoopStages<'a, L>,
}

impl<'a, L: TransformLooper> Loop<'a, L> {
    pub fn new(looper: L) -> Self {
        Self {
            looper,
            stage: LoopStages::PreLoop,
        }
    }
}

impl<'a, L> Iterator for Loop<'a, L>
where
    L: TransformLooper + 'a,
{
    type Item = L::ItemT<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let looper = &mut self.looper;
        match &mut self.stage {
            LoopStages::PreLoop => {
                let mut val_alloca = looper.init_decision().with_storage();
                let cx = val_alloca.cx();
                let init_mut = val_alloca.as_mut();

                let header_block = cx.ctx().append_basic_block(cx.func(), "loop_header");
                let _jmp_header =
                    unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
                        .expect("pre -> header uni branch should work");
                cx.set_bb(header_block);
                let decision = looper.decision_fn(init_mut.load());

                let loop_block = cx.ctx().append_basic_block(cx.func(), "loop_block");
                let done_block = cx.ctx().append_basic_block(cx.func(), "done_block");
                let _jne = unsafe {
                    cx.with_builder(|b| {
                        b.build_conditional_branch(decision.ll_typed(), loop_block, done_block)
                    })
                }
                .expect("conditional jump should work");

                cx.set_bb(loop_block);

                let ret = looper.transform(init_mut.load());
                self.stage = LoopStages::AfterLoop(val_alloca, done_block, header_block);

                Some(ret)
            }
            LoopStages::AfterLoop(val_alloca, done_block, header_block) => {
                let cx = looper.cx();
                let curr_val = val_alloca.as_ref().load();
                val_alloca.as_mut().store(looper.update_fn(curr_val));

                let _jmp =
                    unsafe { cx.with_builder(|b| b.build_unconditional_branch(*header_block)) }
                        .expect("uni br should succeed");
                cx.set_bb(*done_block);
                self.stage = LoopStages::Done;

                None
            }
            LoopStages::Done => None,
        }
    }
}
