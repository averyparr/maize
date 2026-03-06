use rtc_types::{
    intrinsics::{IntrinsicCodegen, cuda::CUDA},
    ty::U32,
    val::Val,
};

use crate::group::{CTA, Group};

pub struct BlockX<'a>(pub IntrinsicCodegen<'a, CUDA>);
pub struct BlockY<'a>(pub IntrinsicCodegen<'a, CUDA>);
pub struct BlockZ<'a>(pub IntrinsicCodegen<'a, CUDA>);

impl<'ctx> Group for BlockX<'ctx> {
    type Scope = CTA;
    fn index_size<'a>(self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        (self.0.bid_x(), self.0.gdim_x())
    }
}

impl<'ctx> Group for BlockY<'ctx> {
    type Scope = CTA;
    fn index_size<'a>(self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        (self.0.bid_y(), self.0.gdim_y())
    }
}

impl<'ctx> Group for BlockZ<'ctx> {
    type Scope = CTA;
    fn index_size<'a>(self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        (self.0.bid_z(), self.0.gdim_z())
    }
}
