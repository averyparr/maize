use rtc_types::{
    codegen::typed_func::FnCodegen,
    intrinsics::{IntrinsicCodegen, IntrinsicsLibrary, cuda::CUDA},
    ty::U32,
    val::Val,
};

use crate::group::{CTA, Group};

#[derive(Clone, Copy)]
pub struct BlockX<'a>(IntrinsicCodegen<'a, CUDA>);
#[derive(Clone, Copy)]
pub struct BlockY<'a>(IntrinsicCodegen<'a, CUDA>);
#[derive(Clone, Copy)]
pub struct BlockZ<'a>(IntrinsicCodegen<'a, CUDA>);

impl<'a> BlockX<'a> {
    pub fn new(cx: &'a FnCodegen) -> Self {
        Self(IntrinsicCodegen::new(cx))
    }
    pub fn block_dim(&self) -> Val<'a, U32> {
        self.index_size().0
    }
    pub fn grid_dim(&self) -> Val<'a, U32> {
        self.index_size().1
    }
}
impl<'a> BlockY<'a> {
    pub fn new(cx: &'a FnCodegen) -> Self {
        Self(IntrinsicCodegen::new(cx))
    }
    pub fn block_dim(&self) -> Val<'a, U32> {
        self.index_size().0
    }
    pub fn grid_dim(&self) -> Val<'a, U32> {
        self.index_size().1
    }
}
impl<'a> BlockZ<'a> {
    pub fn new(cx: &'a FnCodegen) -> Self {
        Self(IntrinsicCodegen::new(cx))
    }
    pub fn block_dim(&self) -> Val<'a, U32> {
        self.index_size().0
    }
    pub fn grid_dim(&self) -> Val<'a, U32> {
        self.index_size().1
    }
}

impl<'ctx> Group for BlockX<'ctx> {
    type Scope = CTA;
    fn index_size<'a>(&self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        let bid_x = self.0.bid_x();
        let gdim_x = self.0.gdim_x();
        unsafe { self.0.assume(bid_x.lt(gdim_x)) };
        (bid_x, gdim_x)
    }
}

impl<'ctx> Group for BlockY<'ctx> {
    type Scope = CTA;
    fn index_size<'a>(&self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        let bid_y = self.0.bid_y();
        let gdim_y = self.0.gdim_y();
        unsafe { self.0.assume(bid_y.lt(gdim_y)) };
        (bid_y, gdim_y)
    }
}

impl<'ctx> Group for BlockZ<'ctx> {
    type Scope = CTA;
    fn index_size<'a>(&self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        let bid_z = self.0.bid_z();
        let gdim_z = self.0.gdim_z();
        unsafe { self.0.assume(bid_z.lt(gdim_z)) };
        (bid_z, gdim_z)
    }
}
