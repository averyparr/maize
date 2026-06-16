use std::ops::Index;

use crate::{control_flow::Looper, tipe::Ty, val::Val};

pub trait Indexable: Ty + Sized {
    type IndexT: Ty;
    type Target: Ty;
    fn index(val: &Val<Self>, index: Val<Self::IndexT>) -> Val<Self::Target>;
    fn len(val: &Val<Self>) -> Val<Self::IndexT>;
    // fn into_iter(val: Val<Self>) -> impl Looper<Input = Self::Target> {} // todo!
}

impl<I: Indexable> Val<I> {
    pub fn index(&self, index: Val<I::IndexT>) -> Val<I::Target> {
        I::index(self, index)
    }
}

pub fn len<I: Indexable>(val: &Val<I>) -> Val<I::IndexT> {
    I::len(&val)
}
