use rtc_types::{
    intrinsics::{IntrinsicCodegen, cuda::CUDA},
    ty::U32,
    val::Val,
};

use crate::group::{ConstSizeGroup, Group};

#[derive(Clone, Copy)]
pub struct Warp<'a>(pub IntrinsicCodegen<'a, CUDA>);

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
