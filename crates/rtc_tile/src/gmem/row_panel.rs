use rtc_types::{
    codegen::{loops::TransformLooper, typed_func::FnCodegen},
    ty::{Addrspace, Bool, DereferencableTy, M, R, SizedTy, U32, ValTy},
    val::Val,
};

use crate::{FixedWidthWindow, W, gmem::Matrix, group::Group};

pub struct RowPanel<'a, RowWindow, Ptr> {
    #[expect(unused, reason = "This will be used later")]
    row_window: RowWindow,
    pub ptr: Val<'a, Ptr>,
    #[expect(unused, reason = "This will be used later")]
    num_cols: Val<'a, U32>,
}

pub struct RowPanelIterLooper<'a, RowWindow, Ptr> {
    row_window: RowWindow,
    ptr: Val<'a, Ptr>,
    curr_row: Val<'a, U32>,
    rows_per_iter: Val<'a, U32>,
    num_cols: Val<'a, U32>,
    last_row: Val<'a, U32>,
}

impl<'ctx, RowWindow, Ptr> TransformLooper for RowPanelIterLooper<'ctx, RowWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: FixedWidthWindow<ElemT = Ptr::Pointee>,
{
    type DecisionItemT = U32;
    type ItemT<'a>
        = RowPanel<'a, RowWindow, Ptr>
    where
        Self: 'a;

    fn cx<'a>(&self) -> &'a FnCodegen
    where
        Self: 'a,
    {
        self.ptr.cx()
    }

    fn init_decision<'a>(&self) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        self.curr_row
    }

    fn decision_fn<'a>(&self, curr_row: Val<'a, Self::DecisionItemT>) -> Val<'a, Bool>
    where
        Self: 'a,
    {
        curr_row.lt(self.last_row)
    }

    fn transform<'a>(&self, curr_row: Val<'a, Self::DecisionItemT>) -> Self::ItemT<'a>
    where
        Self: 'a,
    {
        let offset = curr_row * self.num_cols;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            row_window: self.row_window,
            ptr: panel_init_ptr,
            num_cols: self.num_cols,
        }
    }

    fn update_fn<'a>(&self, curr_row: Val<'a, Self::DecisionItemT>) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        // SAFETY: We know that `self.last_row` is < u32::max,
        // so if this would overflow, then we want to break the
        // loop anyways. So, it should be OK to assume that it will never overflow
        unsafe { curr_row.add_unchecked(self.rows_per_iter) }
    }
}

impl<'a, 'b, T, Space: Addrspace> Matrix<'a, R<&'b T, Space>> {
    pub fn collective_row_panel_iter<'c, const N: u32>(
        &'c mut self,
        group: impl Group + 'a,
    ) -> RowPanelIterLooper<'a, W<T, N>, R<&'b T, Space>>
    where
        T: ValTy,
    {
        let (index, size) = group.index_size();
        let rows_per_iter = size * N;
        let curr_row = index * N;
        RowPanelIterLooper {
            row_window: W::new(),
            ptr: self.ptr,
            curr_row,
            rows_per_iter,
            num_cols: self.ncols,
            last_row: self.nrows,
        }
    }
}

impl<'a, 'b, T, Space: Addrspace> Matrix<'a, M<&'b mut T, Space>> {
    pub fn collective_row_panel_iter<'c, const N: u32>(
        &'c mut self,
        group: impl Group + 'a,
    ) -> RowPanelIterLooper<'a, W<T, N>, M<&'c mut T, Space>>
    where
        T: ValTy,
    {
        let (index, size) = group.index_size();
        let rows_per_iter = size * N;
        let curr_row = index * N;
        RowPanelIterLooper {
            row_window: W::new(),
            ptr: self.ptr.reborrow_mut(),
            curr_row,
            rows_per_iter,
            num_cols: self.ncols,
            last_row: self.nrows,
        }
    }
}
