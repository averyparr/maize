use rtc_types::{codegen::typed_func::FnCodegen, ty::U32, val::Val};

pub mod by_block;

pub struct GPU;
pub struct Cluster;
pub struct CTA;
pub struct Warp;
pub struct Thread;

pub trait GroupScope {}
impl GroupScope for GPU {}
impl GroupScope for Cluster {}
impl GroupScope for CTA {}
impl GroupScope for Warp {}
impl GroupScope for Thread {}

pub trait Group {
    type Scope: GroupScope;

    fn index_size<'a>(&self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a;
}

pub struct NullGroup<'a>(pub &'a FnCodegen);

impl<'ctx> Group for NullGroup<'ctx> {
    type Scope = Thread;
    fn index_size<'a>(&self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        (self.0.constant_from(0u32), self.0.constant_from(1u32))
    }
}
