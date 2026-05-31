use crate::{
    backend::FnRef,
    val::{S, Val},
};

use std::ops::Range;

pub trait Looper {
    type Decision;
    type Input;
    fn fn_ref(&self) -> FnRef;
    fn init(&self) -> Self::Decision;
    fn decide(&self, val: &Self::Decision) -> Val<bool>;
    fn next(&self, val: &mut Self::Decision);
    fn load(&self, val: &Self::Decision) -> Self::Input;

    fn for_each(&self, mut f: impl FnMut(Self::Input)) {
        let fn_ref = self.fn_ref();
        let init_block = fn_ref.get_current_bb();
        let mut init = self.init();
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
            b.build_conditional_branch(self.decide(&init).typed(), bulk_block, tail_block)
                .expect("Cond branch should have worked!");
        });
        fn_ref.with_bb_as(bulk_block, || {
            f(self.load(&init));
            self.next(&mut init);
            // Safety: unconditional branch
            let b = unsafe { fn_ref.curr_bb_builder() };
            b.build_unconditional_branch(head_block)
                .expect("jmp should build");
        })
    }
}

macro_rules! impl_range {
    ($($tipes: ty),* => $one: literal) => {
        $(
            impl Looper for Range<Val<$tipes>> {
                type Decision = Val<S<$tipes>>;
                type Input = Val<$tipes>;
                fn fn_ref(&self) -> FnRef {
                    self.start.fn_ref().clone()
                }

                fn init(&self) -> Self::Decision {
                    self.start.copy().with_storage()
                }
                fn decide(&self, val: &Self::Decision) -> Val<bool> {
                    val.get_ref().load().lt(self.end.copy())
                }
                fn next(&self, val: &mut Self::Decision) {
                    let ret = val.get_mut().load() + val.fn_ref().constant($one);
                    val.get_mut().store(ret);
                }
                fn load(&self, val: &Self::Decision) -> Self::Input {
                    val.get_ref().load()
                }
            }
        )*
    };
}

impl_range!(f32, f64 => 1.0);
impl_range!(i8, u8, i16, u16, i32, u32, i64, u64 => 1);
