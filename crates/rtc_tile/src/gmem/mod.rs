use std::u32;

use rtc_types::{
    codegen::{
        loops::{Loop, TransformLooper},
        typed_func::FnCodegen,
    },
    intrinsics::{IntrinsicCodegen, cuda::CUDA},
    kernel_assert,
    ty::{Bool, RawPtrTy, U16, U32},
    val::Val,
};

pub trait Group {
    fn index_size<'a>(self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a;
}

pub struct BidXGroup<'a>(pub IntrinsicCodegen<'a, CUDA>);
impl<'b> Group for BidXGroup<'b> {
    fn index_size<'a>(self) -> (Val<'a, U32>, Val<'a, U32>)
    where
        Self: 'a,
    {
        (self.0.tid_x(), self.0.bdim_x())
    }
}

pub struct Matrix<'a, Ptr> {
    ptr: Val<'a, Ptr>,
    nrows: Val<'a, U32>,
    ncols: Val<'a, U32>,
}

impl<'a, Ptr> Matrix<'a, Ptr> {
    pub fn new(ptr: Val<'a, Ptr>, nrows: Val<'a, U16>, ncols: Val<'a, U16>) -> Self {
        Self {
            ptr,
            nrows: nrows.cast(),
            ncols: ncols.cast(),
        }
    }
}

pub struct RowPanel<'a, Ptr> {
    pub ptr: Val<'a, Ptr>,
    panel_rows: u32,
    num_cols: Val<'a, U32>,
}

pub struct RowPanelIterLooper<'a, Ptr> {
    ptr: Val<'a, Ptr>,
    curr_row: Val<'a, U32>,
    panel_rows: u32,
    rows_per_iter: Val<'a, U32>,
    num_cols: Val<'a, U32>,
    last_row: Val<'a, U32>,
}

impl<'ctx, Ptr> TransformLooper for RowPanelIterLooper<'ctx, Ptr>
where
    Ptr: RawPtrTy,
{
    type DecisionItemT = U32;
    type ItemT<'a>
        = RowPanel<'a, Ptr>
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
            ptr: panel_init_ptr,
            panel_rows: self.panel_rows,
            num_cols: self.num_cols,
        }
    }

    fn update_fn<'a>(&self, curr_row: Val<'a, Self::DecisionItemT>) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        // SAFETY: I'm actually not sure if this is what I want, but if we overflow U32,
        // then I'd argue that our loop was broken in the first place and so it should be
        // OK to optimize on that basis.
        unsafe { curr_row.add_unchecked(self.rows_per_iter) }
    }
}

impl<'a, Ptr: RawPtrTy + 'a> Matrix<'a, Ptr> {
    pub fn row_panel_iter_by_group<G: Group + 'a>(
        &mut self,
        group: G,
        panel_rows: u32,
    ) -> impl Iterator<Item = RowPanel<'a, Ptr>> {
        let (index, size) = group.index_size();
        let rows_per_iter = size * panel_rows;
        let curr_row = index * panel_rows;
        let cannot_overflow = (self.nrows + rows_per_iter).lt(index.cx().constant(u32::MAX));
        kernel_assert!(cannot_overflow, "Would overflow U32 during iteration!");
        Loop::new(RowPanelIterLooper {
            ptr: self.ptr,
            curr_row,
            panel_rows,
            rows_per_iter,
            num_cols: self.ncols,
            last_row: self.nrows,
        })
    }
}
