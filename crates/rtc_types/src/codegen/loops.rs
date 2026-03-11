use core::ops::Range;

use crate::{
    codegen::FnCodegen,
    struct_reflect,
    ty::{Bool, StructReflectTy, U32},
    val::Val,
};

pub trait TransformLooper: Sized {
    type DecisionItemT: StructReflectTy + Copy;
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
    fn step_n(&mut self, n: usize);
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
        decision_val + decision_val.const_like(1)
    }
    fn step_n(&mut self, n: usize) {
        self.start = self.start
            + self
                .start
                .const_like(n.try_into().expect("usize -> u32 overflow"));
    }
}

struct_reflect!(
    pub struct ZippedPair<A, B> {
        pub left: A, pub right: B
    } => ziped_pair
);
impl<A: Clone, B: Clone> Clone for ZippedPair<A, B> {
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}
impl<A: Copy, B: Copy> Copy for ZippedPair<A, B> {}

pub struct ZippedLooper<A, B>(A, B);

impl<'val, A: TransformLooper, B: TransformLooper> TransformLooper for ZippedLooper<A, B>
where
    A::DecisionItemT: StructReflectTy,
    B::DecisionItemT: StructReflectTy,
{
    type DecisionItemT = ZippedPair<A::DecisionItemT, B::DecisionItemT>;

    type ItemT<'a>
        = (A::ItemT<'a>, B::ItemT<'a>)
    where
        Self: 'a;

    fn cx<'a>(&self) -> &'a FnCodegen
    where
        Self: 'a,
    {
        self.0.cx()
    }

    fn init_decision<'a>(&self) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        let a = self.0.init_decision();
        let b = self.1.init_decision();
        ZippedPair::from_fields(a, b)
    }

    fn decision_fn<'a>(&self, decision_val: Val<'a, Self::DecisionItemT>) -> Val<'a, Bool>
    where
        Self: 'a,
    {
        let accessor = decision_val.into_accessor();
        let a = self.0.decision_fn(accessor.left);
        let b = self.1.decision_fn(accessor.right);
        a & b
    }

    fn transform<'a>(&self, decision_val: Val<'a, Self::DecisionItemT>) -> Self::ItemT<'a>
    where
        Self: 'a,
    {
        let accessor = decision_val.into_accessor();
        let a_val = self.0.transform(accessor.left);
        let b_val = self.1.transform(accessor.right);
        (a_val, b_val)
    }

    fn update_fn<'a>(
        &self,
        decision_val: Val<'a, Self::DecisionItemT>,
    ) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        let accessor = decision_val.into_accessor();
        let new_a = self.0.update_fn(accessor.left);
        let new_b = self.1.update_fn(accessor.right);
        ZippedPair::from_fields(new_a, new_b)
    }
    fn step_n(&mut self, n: usize) {
        self.0.step_n(n);
        self.1.step_n(n);
    }
}

pub trait Looper: TransformLooper {
    fn on_first<'a, F>(&'a mut self, f: F)
    where
        F: FnOnce(Self::ItemT<'a>),
    {
        let decision_val = self.init_decision();
        let should_do = self.decision_fn(decision_val);
        should_do.branch(|| {
            let transformed = self.transform(decision_val);
            f(transformed)
        });
        self.step_n(1);
    }
    fn on_first_n<'a, F>(&'a mut self, n: usize, mut f: F)
    where
        F: FnMut(Self::ItemT<'a>),
    {
        let mut decision_val = self.init_decision();
        let cx = decision_val.cx();
        let final_bb = cx.ctx().append_basic_block(cx.func(), "early_skip");

        for _ in 0..n {
            let then_block = cx.ctx().append_basic_block(cx.func(), "first_n_block");
            let comparison = self.decision_fn(decision_val).ll_typed();
            let _ins = unsafe {
                cx.with_builder(|b| b.build_conditional_branch(comparison, then_block, final_bb))
                    .expect("Should be able to build conditional branch")
            };
            cx.set_bb(then_block);
            f(self.transform(decision_val));
            decision_val = self.update_fn(decision_val);
        }

        let _ins = unsafe { cx.with_builder(|b| b.build_unconditional_branch(final_bb)) }
            .expect("Unconditional branch should succeed");
        self.step_n(n);
    }
    fn for_every_value<'a, F>(self, mut f: F)
    where
        F: FnMut(Self::ItemT<'a>),
        Self: 'a,
    {
        let mut val_alloca = self.init_decision().with_storage();
        let cx = val_alloca.cx();
        let init_mut = val_alloca.as_mut();

        let header_block = cx.ctx().append_basic_block(cx.func(), "loop_header");
        let _jmp_header =
            unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
                .expect("pre -> header uni branch should work");
        cx.set_bb(header_block);
        let decision = self.decision_fn(init_mut.load());

        let loop_block = cx.ctx().append_basic_block(cx.func(), "loop_block");
        let done_block = cx.ctx().append_basic_block(cx.func(), "done_block");
        let _jne = unsafe {
            cx.with_builder(|b| {
                b.build_conditional_branch(decision.ll_typed(), loop_block, done_block)
            })
        }
        .expect("conditional jump should work");

        cx.set_bb(loop_block);

        let ret = self.transform(init_mut.load());
        f(ret);

        let curr_val = val_alloca.as_ref().load();
        val_alloca.as_mut().store(self.update_fn(curr_val));

        let _jmp = unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
            .expect("uni br should succeed");
        cx.set_bb(done_block);
    }

    fn zip<'a, OtherLooper: Looper + 'a>(
        self,
        other: OtherLooper,
    ) -> ZippedLooper<Self, OtherLooper> {
        ZippedLooper(self, other)
    }
}

impl<'val, L> Looper for L where L: TransformLooper {}
