use core::ops::Range;

use inkwell::{
    basic_block::BasicBlock,
    values::{BasicValue, PointerValue},
};

use crate::{
    codegen::FnCodegen,
    ty::{Bool, M, SizedTy, U32},
    val::Val,
};

pub trait Looper<'a> {
    type ItemT: SizedTy + Copy;
    fn init_fn(&self) -> Val<'a, Self::ItemT>;
    fn decision_fn(&self, val: &Val<'a, Self::ItemT>) -> Val<'a, Bool>;
    fn update_fn(val: Val<'a, Self::ItemT>) -> Val<'a, Self::ItemT>;
}

impl<'a> Looper<'a> for Range<Val<'a, U32>> {
    type ItemT = U32;
    fn init_fn(&self) -> Val<'a, Self::ItemT> {
        self.start
    }
    fn decision_fn(&self, val: &Val<'a, Self::ItemT>) -> Val<'a, Bool> {
        val.lt(self.end)
    }
    fn update_fn(val: Val<'a, Self::ItemT>) -> Val<'a, U32> {
        val.const_like(1) + val
    }
}

// fn gen_loop_from_looper<'a, L: Looper<'a>>(
//     looper: L,
//     for_each_fn: impl Fn(Val<'_, L::ItemT>) -> Val<'_, L::ItemT>,
// ) {
//     let mut init_val = looper.init_fn();
//     let cx = init_val.cx();
//     let mut init_mut = init_val.as_mut();

//     let header_block = cx.ctx().append_basic_block(cx.func(), "loop_header");
//     let _jmp_header = unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
//         .expect("pre -> header uni branch should work");
//     cx.set_bb(header_block);
//     let decision = looper.decision_fn(&init_mut.load());

//     let loop_block = cx.ctx().append_basic_block(cx.func(), "loop_block");
//     let done_block = cx.ctx().append_basic_block(cx.func(), "done_block");
//     let _jne = unsafe {
//         cx.with_builder(|b| {
//             b.build_conditional_branch(decision.get_ll_typed(), loop_block, done_block)
//         })
//     }
//     .expect("conditional jump should work");

//     cx.set_bb(loop_block);
//     init_mut.store(for_each_fn(init_mut.load()));
//     let _jmp = unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
//         .expect("Unconditional jump should work");
//     cx.set_bb(done_block);
// }

pub enum Loop<'a, L: Looper<'a>> {
    PreLoop(L),
    AfterLoop(
        &'a FnCodegen,
        PointerValue<'static>,
        BasicBlock<'static>,
        BasicBlock<'static>,
    ),
    Done,
}

impl<'a, L: Looper<'a>> Loop<'a, L> {
    pub fn new(looper: L) -> Self {
        Self::PreLoop(looper)
    }
}

impl<'a, L> Iterator for Loop<'a, L>
where
    L: Looper<'a>,
{
    type Item = Val<'a, L::ItemT>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::PreLoop(looper) => {
                let mut init_val = looper.init_fn().with_storage();
                let cx = init_val.cx();
                let ptr_to_val = init_val.alloca_ptr();
                let init_mut = init_val.as_mut();

                let header_block = cx.ctx().append_basic_block(cx.func(), "loop_header");
                let _jmp_header =
                    unsafe { cx.with_builder(|b| b.build_unconditional_branch(header_block)) }
                        .expect("pre -> header uni branch should work");
                cx.set_bb(header_block);
                let decision = looper.decision_fn(&init_mut.load());

                let loop_block = cx.ctx().append_basic_block(cx.func(), "loop_block");
                let done_block = cx.ctx().append_basic_block(cx.func(), "done_block");
                let _jne = unsafe {
                    cx.with_builder(|b| {
                        b.build_conditional_branch(decision.ll_typed(), loop_block, done_block)
                    })
                }
                .expect("conditional jump should work");

                cx.set_bb(loop_block);

                let ret = init_mut.load();
                *self = Loop::AfterLoop(cx, ptr_to_val, done_block, header_block);

                Some(ret)
            }
            Loop::AfterLoop(cx, ptr_to_val, done_block, header_block) => {
                let mut mut_ptr = unsafe {
                    Val::<'_, M<&mut L::ItemT>>::new(cx, ptr_to_val.as_basic_value_enum())
                };
                mut_ptr.store(L::update_fn(mut_ptr.load()));

                let _jmp =
                    unsafe { cx.with_builder(|b| b.build_unconditional_branch(*header_block)) }
                        .expect("uni br should succeed");
                cx.set_bb(*done_block);
                *self = Loop::Done;

                None
            }
            Loop::Done => None,
        }
    }
}
