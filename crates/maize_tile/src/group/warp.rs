use maize_core::{
    codegen::typed_func::FnCodegen,
    intrinsics::{IntrinsicCodegen, cuda::CUDA},
    ty::U32,
    val::Val,
};

use crate::group::{ConstSizeGroup, Group};

#[derive(Clone, Copy)]
pub struct Warp<'a>(IntrinsicCodegen<'a, CUDA>);

impl<'a> Warp<'a> {
    pub fn new(cx: &'a FnCodegen) -> Self {
        Self(IntrinsicCodegen::new(cx))
    }
    pub fn lane(&self) -> Val<'a, U32> {
        self.0.sregs().laneid()
    }
}

impl Group for Warp<'_> {
    type Scope = super::Warp;
    fn index_size<'a>(&self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        let laneid = self.0.sregs().laneid();
        (laneid, laneid.const_like(32))
    }
}

impl ConstSizeGroup for Warp<'_> {
    fn const_size(&self) -> u32 {
        32
    }
}
