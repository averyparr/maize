use rtc_types::{
    codegen::{
        loops::{Loop, TransformLooper},
        typed_func::FnCodegen,
    },
    ty::{Bool, RawPtrTy, U32},
    val::Val,
};

use crate::{FixedWidthWindow, W, gmem::Matrix, group::Group};

pub struct RowPanel<'a, Panel, Ptr> {
    panel: Panel,
    pub ptr: Val<'a, Ptr>,
    panel_rows: u32,
    num_cols: Val<'a, U32>,
}

pub struct RowPanelIterLooper<'a, Panel, Ptr> {
    panel: Panel,
    ptr: Val<'a, Ptr>,
    curr_row: Val<'a, U32>,
    panel_rows: u32,
    rows_per_iter: Val<'a, U32>,
    num_cols: Val<'a, U32>,
    last_row: Val<'a, U32>,
}

impl<'ctx, Panel, Ptr> TransformLooper for RowPanelIterLooper<'ctx, Panel, Ptr>
where
    Panel: FixedWidthWindow<ElemT = Ptr::PointeeTy>,
    Ptr: RawPtrTy,
{
    type DecisionItemT = U32;
    type ItemT<'a>
        = RowPanel<'a, Panel, Ptr>
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
        let panel_init_ptr = unsafe { self.ptr.add(offset) };
        Self::ItemT {
            panel: self.panel,
            ptr: panel_init_ptr,
            panel_rows: self.panel_rows,
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

impl<'a, Ptr: RawPtrTy + 'a> Matrix<'a, Ptr> {
    pub fn collective_row_panel_iter<const N: u32>(
        &mut self,
        group: impl Group + 'a,
    ) -> impl Iterator<Item = RowPanel<'a, W<Ptr::PointeeTy, N>, Ptr>>
    where
        Ptr::PointeeTy: Sized,
    {
        let panel_rows = 16;
        let (index, size) = group.index_size();
        let rows_per_iter = size * panel_rows;
        let curr_row = index * panel_rows;
        Loop::new(RowPanelIterLooper {
            panel: W::new(),
            ptr: self.ptr,
            curr_row,
            panel_rows,
            rows_per_iter,
            num_cols: self.ncols,
            last_row: self.nrows,
        })
    }
}
