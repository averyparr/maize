use rtc_types::{
    ty::{Addrspace, DereferencableTy, M, R, U32, ValTy},
    val::Val,
};

pub struct Matrix<'a, Ptr> {
    pub(crate) ptr: Val<'a, Ptr>,
    pub(crate) nrows: Val<'a, U32>,
    pub(crate) ncols: Val<'a, U32>,
}

impl<'a, Ptr> Matrix<'a, Ptr> {
    pub fn new(ptr: Val<'a, Ptr>, nrows: Val<'a, U32>, ncols: Val<'a, U32>) -> Self
    where
        Ptr: DereferencableTy,
    {
        Self { ptr, nrows, ncols }
    }
}

impl<'a, 'b, T: ValTy, Space: Addrspace> Matrix<'a, R<&'b T, Space>> {
    pub fn reborrow<'c>(&'c self) -> Matrix<'a, R<&'c T, Space>>
    where
        'b: 'c,
    {
        Matrix {
            ptr: self.ptr.reborrow(),
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }
}

impl<'a, 'b, T: ValTy, Space: Addrspace> Matrix<'a, M<&'b mut T, Space>> {
    pub fn reborrow<'c>(&'c self) -> Matrix<'a, R<&'c T, Space>>
    where
        'b: 'c,
    {
        Matrix {
            ptr: self.ptr.reborrow(),
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }
    pub fn reborrow_mut<'c>(&'c mut self) -> Matrix<'a, M<&'c mut T, Space>>
    where
        'b: 'c,
    {
        Matrix {
            ptr: self.ptr.reborrow_mut(),
            nrows: self.nrows,
            ncols: self.ncols,
        }
    }
}
