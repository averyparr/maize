use rtc_types::{
    ty::{U16, U32},
    val::Val,
};

pub struct Matrix<'a, Ptr> {
    pub(crate) ptr: Val<'a, Ptr>,
    pub(crate) nrows: Val<'a, U32>,
    pub(crate) ncols: Val<'a, U32>,
}

impl<'a, Ptr> Matrix<'a, Ptr> {
    pub fn new(ptr: Val<'a, Ptr>, nrows: Val<'a, U32>, ncols: Val<'a, U32>) -> Self {
        Self { ptr, nrows, ncols }
    }
}
