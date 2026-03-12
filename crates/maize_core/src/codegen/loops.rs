use core::ops::Range;

use crate::{
    codegen::FnCodegen,
    kernel_assert, struct_reflect,
    ty::{Bool, StructReflectTy, U32},
    val::Val,
};

pub trait TransformLooper<'ctx>: Sized {
    type DecisionItemT: StructReflectTy + Copy;
    type ItemT;
    fn cx(&self) -> &'ctx FnCodegen;
    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT>;
    fn decision_fn(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Val<'ctx, Bool>;
    fn transform(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT;
    fn update_fn<'a>(
        &self,
        decision_val: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT>;
    fn step_n(&mut self, n: usize);
}

impl<'ctx> TransformLooper<'ctx> for Range<Val<'ctx, U32>> {
    type DecisionItemT = U32;
    type ItemT = Val<'ctx, U32>;
    fn cx(&self) -> &'ctx FnCodegen {
        self.start.cx()
    }
    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        self.start
    }
    fn decision_fn(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Val<'ctx, Bool> {
        decision_val.lt(self.end)
    }
    fn transform(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        decision_val
    }
    fn update_fn<'a>(
        &self,
        decision_val: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT> {
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

impl<A, B> ZippedLooper<A, B> {
    pub fn unzip(self) -> (A, B) {
        (self.0, self.1)
    }
}

pub struct RepeatLoop<T> {
    val: T,
}

impl<'ctx, T: TransformLooper<'ctx> + Copy> TransformLooper<'ctx> for RepeatLoop<T> {
    type DecisionItemT = U32;
    type ItemT = T;

    fn cx(&self) -> &'ctx FnCodegen {
        self.val.cx()
    }

    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        self.cx().constant(0u32)
    }

    fn decision_fn(&self, _: Val<'ctx, Self::DecisionItemT>) -> Val<'ctx, Bool> {
        self.cx().constant(true)
    }

    fn transform(&self, _: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        self.val
    }

    fn update_fn<'a>(&self, _: Val<'ctx, Self::DecisionItemT>) -> Val<'ctx, Self::DecisionItemT> {
        self.init_decision()
    }

    fn step_n(&mut self, _: usize) {}
}

pub fn repeat_value<T>(val: T) -> RepeatLoop<T> {
    RepeatLoop { val }
}

impl<'ctx, A: TransformLooper<'ctx>, B: TransformLooper<'ctx>> TransformLooper<'ctx>
    for ZippedLooper<A, B>
where
    A::DecisionItemT: StructReflectTy,
    B::DecisionItemT: StructReflectTy,
{
    type DecisionItemT = ZippedPair<A::DecisionItemT, B::DecisionItemT>;

    type ItemT = (A::ItemT, B::ItemT);

    fn cx(&self) -> &'ctx FnCodegen {
        self.0.cx()
    }

    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        let a = self.0.init_decision();
        let b = self.1.init_decision();
        ZippedPair::from_fields(a, b)
    }

    fn decision_fn(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Val<'ctx, Bool> {
        let accessor = decision_val.into_accessor();
        let a = self.0.decision_fn(accessor.left);
        let b = self.1.decision_fn(accessor.right);
        a & b
    }

    fn transform(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        let accessor = decision_val.into_accessor();
        let a_val = self.0.transform(accessor.left);
        let b_val = self.1.transform(accessor.right);
        (a_val, b_val)
    }

    fn update_fn<'a>(
        &self,
        decision_val: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT> {
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

pub trait Looper<'ctx>: TransformLooper<'ctx> {
    fn on_first<F, U>(&mut self, f: F) -> U
    where
        F: FnOnce(Self::ItemT) -> U,
    {
        let decision_val = self.init_decision();
        let should_do = self.decision_fn(decision_val);
        kernel_assert!(should_do); // or else we cannot actually return!
        let transformed = self.transform(decision_val);
        let ret = f(transformed);
        self.step_n(1);
        ret
    }
    fn on_first_n_runtime<F>(&mut self, n: Val<'_, U32>, mut f: F)
    where
        F: FnMut(Self::ItemT),
    {
        let mut n_alloca = n.zero().with_storage();
        let mut val_alloca = self.init_decision().with_storage();
        let cx = val_alloca.cx();
        let init_mut = val_alloca.as_mut();
        let n_mut = n_alloca.as_mut();

        let header_block = cx.ctx().append_basic_block(cx.func(), "loop_header");
        let _jmp_header =
            unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
                .expect("pre -> header uni branch should work");
        cx.set_bb(header_block);
        let decision = self.decision_fn(init_mut.load()) & n_mut.load().lt(n);

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
    fn on_first_n<F>(&mut self, n: usize, mut f: F)
    where
        F: FnMut(Self::ItemT),
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

        cx.set_bb(final_bb);
        self.step_n(n);
    }
    fn for_every_value<F>(self, mut f: F)
    where
        F: FnMut(Self::ItemT),
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

    fn zip<'a, OtherLooper: Looper<'ctx>>(
        self,
        other: OtherLooper,
    ) -> ZippedLooper<Self, OtherLooper> {
        ZippedLooper(self, other)
    }
}

impl<'ctx, L> Looper<'ctx> for L where L: TransformLooper<'ctx> {}
