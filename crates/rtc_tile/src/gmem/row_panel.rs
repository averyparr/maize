use rtc_types::{
    codegen::{loops::TransformLooper, typed_func::FnCodegen},
    ty::{Addrspace, Bool, DereferencableTy, M, R, SizedTy, U32},
    val::Val,
};

use crate::{DW, W, Window, gmem::Matrix, group::Group};

#[derive(Clone, Copy)]
pub struct RowPanel<'a, RowWindow, Ptr> {
    pub(crate) row_window: RowWindow,
    pub ptr: Val<'a, Ptr>,
    pub(crate) num_cols: Val<'a, U32>,
}

impl<'a, RowWindow: Window + 'a, Ptr> RowPanel<'a, RowWindow, Ptr> {
    pub fn row_size(&self) -> Val<'a, U32> {
        self.row_window.size(self.ptr.cx())
    }
    pub fn col_size(&self) -> Val<'a, U32> {
        self.num_cols
    }
}

pub struct RowPanelIterLooper<'a, RowWindow, Ptr> {
    row_window: RowWindow,
    ptr: Val<'a, Ptr>,
    init_row: Val<'a, U32>,
    rows_per_iter: Val<'a, U32>,
    num_cols: Val<'a, U32>,
    last_row: Val<'a, U32>,
}

impl<'ctx, RowWindow, Ptr> TransformLooper<'ctx> for RowPanelIterLooper<'ctx, RowWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
{
    type DecisionItemT = U32;
    type ItemT = RowPanel<'ctx, RowWindow, Ptr>;

    fn cx(&self) -> &'ctx FnCodegen {
        self.ptr.cx()
    }

    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        self.init_row
    }

    fn decision_fn(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Val<'ctx, Bool> {
        decision_val.lt(self.last_row)
    }

    fn transform(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        let offset = decision_val * self.num_cols;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            row_window: self.row_window,
            ptr: panel_init_ptr,
            num_cols: self.num_cols,
        }
    }

    fn update_fn<'a>(
        &self,
        decision_val: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT> {
        // SAFETY: We know that `self.last_row` is < u32::max,
        // so if this would overflow, then we want to break the
        // loop anyways. So, it should be OK to assume that it will never overflow
        unsafe { decision_val.add_unchecked(self.rows_per_iter) }
    }

    fn step_n(&mut self, n: usize) {
        let n: u32 = n.try_into().expect("usize -> u32 overflow");
        self.init_row = self.init_row + self.rows_per_iter * n;
    }
}

impl<'a, 'b, T, Ptr> Matrix<'a, Ptr>
where
    Ptr: DereferencableTy<Pointee = T>,
    T: SizedTy + 'b,
{
    fn inner_collective_aligned_row_panel_iter<const N: u32>(
        self,
        group: impl Group + 'a,
    ) -> (
        RowPanelIterLooper<'a, W<T, N>, Ptr>,
        RowPanel<'a, DW<'a, T>, Ptr>,
    )
    where
        Ptr: DereferencableTy<Pointee: SizedTy>,
    {
        let (index, size) = group.index_size();
        let rows_per_iter = size * N;
        let epilogue_size = self.nrows % rows_per_iter;
        let bulk_rows = self.nrows - epilogue_size;
        let epilogue_offset = bulk_rows * self.ncols;
        let init_row = index * N;
        let offset_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |raw| raw) };
        let bulk_iter = RowPanelIterLooper {
            row_window: W::new(),
            ptr: offset_ptr,
            init_row,
            rows_per_iter,
            num_cols: self.ncols,
            last_row: bulk_rows,
        };

        let rest_ptr = unsafe {
            Ptr::on_underlying_raw(&self.ptr, |raw| {
                raw.add(epilogue_offset + init_row * self.ncols)
            })
        };
        let epilogue_width = init_row
            .lt(epilogue_size)
            .then(|| (epilogue_size - init_row).min(rows_per_iter.const_like(N)))
            .or_else(|| rows_per_iter.const_like(0));
        let rest = RowPanel {
            row_window: DW::new(epilogue_width),
            ptr: rest_ptr,
            num_cols: self.ncols,
        };

        (bulk_iter, rest)
    }
}

impl<'a, 'c, 'b, T> Matrix<'a, R<&'c T>>
where
    'c: 'b,
    T: SizedTy,
{
    pub fn collective_aligned_row_panel_iter<const N: u32>(
        &'b self,
        group: impl Group + 'a,
    ) -> (
        RowPanelIterLooper<'a, W<T, N>, R<&'b T>>,
        RowPanel<'a, DW<'a, T>, R<&'b T>>,
    ) {
        self.reborrow()
            .inner_collective_aligned_row_panel_iter(group)
    }
}

impl<'a, 'c, 'b, T, Space: Addrspace> Matrix<'a, M<&'c mut T, Space>>
where
    'c: 'b,
    T: SizedTy,
{
    pub fn collective_aligned_row_panel_iter<const N: u32>(
        &'b self,
        group: impl Group + 'a,
    ) -> (
        RowPanelIterLooper<'a, W<T, N>, R<&'b T, Space>>,
        RowPanel<'a, DW<'a, T>, R<&'b T, Space>>,
    ) {
        self.reborrow()
            .inner_collective_aligned_row_panel_iter(group)
    }
    pub fn collective_aligned_row_panel_iter_mut<const N: u32>(
        &'b mut self,
        group: impl Group + 'a,
    ) -> (
        RowPanelIterLooper<'a, W<T, N>, M<&'b mut T, Space>>,
        RowPanel<'a, DW<'a, T>, M<&'b mut T, Space>>,
    ) {
        self.reborrow_mut()
            .inner_collective_aligned_row_panel_iter(group)
    }
}
