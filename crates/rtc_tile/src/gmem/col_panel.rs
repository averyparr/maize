use rtc_types::{
    codegen::loops::TransformLooper,
    ty::{Addrspace, DereferencableTy, M, R, SizedTy, U32, ValTy},
    val::Val,
};

use crate::{FixedWidthWindow, W, gmem::Matrix, group::Group};

pub struct ColPanel<'a, ColWindow, Ptr> {
    #[expect(unused, reason = "This will be used later")]
    col_window: ColWindow,
    pub ptr: Val<'a, Ptr>,
    #[expect(unused, reason = "This will be used later")]
    num_rows: Val<'a, U32>,
    #[expect(unused, reason = "This will be used later")]
    stride_per_row: Val<'a, U32>,
}

pub struct ColPanelIterLooper<'a, ColWindow, Ptr> {
    col_window: ColWindow,
    ptr: Val<'a, Ptr>,
    curr_col: Val<'a, U32>,
    cols_per_iter: Val<'a, U32>,
    num_rows: Val<'a, U32>,
    num_cols: Val<'a, U32>,
}

impl<'ctx, ColWindow, Ptr> TransformLooper for ColPanelIterLooper<'ctx, ColWindow, Ptr>
where
    ColWindow: FixedWidthWindow<ElemT = Ptr::Pointee>,
    Ptr: DereferencableTy<Pointee: SizedTy>,
{
    type DecisionItemT = U32;
    type ItemT<'a>
        = ColPanel<'a, ColWindow, Ptr>
    where
        Self: 'a;
    fn cx<'a>(&self) -> &'a rtc_types::codegen::typed_func::FnCodegen
    where
        Self: 'a,
    {
        self.ptr.cx()
    }

    fn init_decision<'a>(&self) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        self.curr_col
    }

    fn decision_fn<'a>(
        &self,
        curr_col: Val<'a, Self::DecisionItemT>,
    ) -> Val<'a, rtc_types::ty::Bool>
    where
        Self: 'a,
    {
        curr_col.lt(self.num_cols)
    }

    fn transform<'a>(&self, curr_col: Val<'a, Self::DecisionItemT>) -> Self::ItemT<'a>
    where
        Self: 'a,
    {
        let offset = curr_col;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            col_window: self.col_window,
            ptr: panel_init_ptr,
            num_rows: self.num_rows,
            stride_per_row: self.num_cols,
        }
    }

    fn update_fn<'a>(&self, curr_col: Val<'a, Self::DecisionItemT>) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        unsafe { curr_col.add_unchecked(self.cols_per_iter) }
    }
}

impl<'a, 'b, T, Space: Addrspace> Matrix<'a, R<&'b T, Space>> {
    pub fn collective_col_panel_iter<const N: u32>(
        &mut self,
        group: impl Group + 'a,
    ) -> ColPanelIterLooper<'a, W<T, N>, R<&'b T, Space>>
    where
        T: ValTy,
    {
        let (index, size) = group.index_size();
        let cols_per_iter = size * N;
        let curr_col = index * N;
        ColPanelIterLooper {
            col_window: W::new(),
            ptr: self.ptr,
            curr_col,
            cols_per_iter,
            num_rows: self.nrows,
            num_cols: self.ncols,
        }
    }
}

impl<'a, 'b, T, Space: Addrspace> Matrix<'a, M<&'b mut T, Space>> {
    pub fn collective_col_panel_iter<'c, const N: u32>(
        &'c mut self,
        group: impl Group + 'a,
    ) -> ColPanelIterLooper<'a, W<T, N>, M<&'c mut T, Space>>
    where
        T: ValTy,
    {
        let (index, size) = group.index_size();
        let cols_per_iter = size * N;
        let curr_col = index * N;
        ColPanelIterLooper {
            col_window: W::new(),
            ptr: self.ptr.reborrow_mut(),
            curr_col,
            cols_per_iter,
            num_rows: self.nrows,
            num_cols: self.ncols,
        }
    }
}
