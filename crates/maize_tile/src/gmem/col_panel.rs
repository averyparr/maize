use maize_core::{
    codegen::{loops::TransformLooper, typed_func::FnCodegen},
    ty::{Addrspace, DereferencableTy, M, R, SizedTy, U32},
    val::Val,
};

use crate::{DW, W, Window, gmem::Matrix, group::Group};

#[derive(Clone, Copy)]
pub struct ColPanel<'a, ColWindow, Ptr> {
    pub(crate) col_window: ColWindow,
    pub ptr: Val<'a, Ptr>,
    pub num_rows: Val<'a, U32>,
    pub stride_per_row: Val<'a, U32>,
}

impl<'a, ColWindow: Window + 'a, Ptr> ColPanel<'a, ColWindow, Ptr> {
    pub fn row_size(&self) -> Val<'a, U32> {
        self.num_rows
    }
    pub fn col_size(&self) -> Val<'a, U32> {
        self.col_window.size(self.ptr.cx())
    }
}

#[derive(Clone, Copy)]
pub struct ColPanelIterLooper<'a, ColWindow, Ptr> {
    col_window: ColWindow,
    ptr: Val<'a, Ptr>,
    init_col: Val<'a, U32>,
    cols_per_iter: Val<'a, U32>,
    num_rows: Val<'a, U32>,
    stride_per_row: Val<'a, U32>,
    last_col: Val<'a, U32>,
}

impl<'ctx, ColWindow, Ptr> TransformLooper<'ctx> for ColPanelIterLooper<'ctx, ColWindow, Ptr>
where
    ColWindow: Window<ElemT = Ptr::Pointee>,
    Ptr: DereferencableTy<Pointee: SizedTy>,
{
    type DecisionItemT = U32;
    type ItemT = ColPanel<'ctx, ColWindow, Ptr>;
    fn cx(&self) -> &'ctx FnCodegen {
        self.ptr.cx()
    }

    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        self.init_col
    }

    fn decision_fn(
        &self,
        decision_val: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, maize_core::ty::Bool> {
        decision_val.lt(self.last_col)
    }

    fn transform(&self, decision_val: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        let offset = decision_val;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            col_window: self.col_window,
            ptr: panel_init_ptr,
            num_rows: self.num_rows,
            stride_per_row: self.stride_per_row,
        }
    }

    fn update_fn<'a>(
        &self,
        decision_val: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT> {
        unsafe { decision_val.add_unchecked(self.cols_per_iter) }
    }

    fn step_n(&mut self, n: usize) {
        let n: u32 = n.try_into().expect("usize -> u32 overflow");
        self.init_col = self.init_col + self.cols_per_iter * n;
    }
}

impl<'a, 'b, Ptr, T> Matrix<'a, Ptr>
where
    Ptr: DereferencableTy<Pointee = T>,
    T: SizedTy + 'b,
{
    fn inner_collective_aligned_col_panel_iter<const N: u32>(
        &'b mut self,
        group: impl Group + 'a,
    ) -> (
        ColPanelIterLooper<'a, W<T, N>, Ptr>,
        ColPanel<'a, DW<'a, Ptr::Pointee>, Ptr>,
    ) {
        let (index, size) = group.index_size();
        let cols_per_iter = size * N;
        let epilogue_size = self.ncols % cols_per_iter;
        let bulk_cols = self.ncols - epilogue_size;
        let init_col = index * N;
        let launder_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |ptr| ptr) };
        let bulk_iter = ColPanelIterLooper {
            col_window: W::new(),
            ptr: launder_ptr,
            init_col,
            cols_per_iter,
            num_rows: self.nrows,
            last_col: bulk_cols,
            stride_per_row: self.ncols,
        };

        let rest_ptr =
            unsafe { Ptr::on_underlying_raw(&self.ptr, |ptr| ptr.add(bulk_cols + init_col)) };
        let epilogue_width = init_col
            .lt(epilogue_size)
            .then(|| (epilogue_size - init_col).min(index.const_like(N)))
            .or(index.const_like(0));
        let rest = ColPanel {
            col_window: DW::new(epilogue_width),
            ptr: rest_ptr,
            num_rows: self.nrows,
            stride_per_row: self.ncols,
        };
        (bulk_iter, rest)
    }
}

impl<'a, 'c, 'b, T> Matrix<'a, R<&'c T>>
where
    'c: 'b,
    T: SizedTy,
{
    pub fn collective_aligned_col_panel_iter<const N: u32>(
        &'b self,
        group: impl Group + 'a,
    ) -> (
        ColPanelIterLooper<'a, W<T, N>, R<&'b T>>,
        ColPanel<'a, DW<'a, T>, R<&'b T>>,
    ) {
        self.reborrow()
            .inner_collective_aligned_col_panel_iter(group)
    }
}

impl<'a, 'c, 'b, T, Space: Addrspace> Matrix<'a, M<&'c mut T, Space>>
where
    'c: 'b,
    T: SizedTy,
{
    pub fn collective_aligned_col_panel_iter<const N: u32>(
        &'b self,
        group: impl Group + 'a,
    ) -> (
        ColPanelIterLooper<'a, W<T, N>, R<&'b T, Space>>,
        ColPanel<'a, DW<'a, T>, R<&'b T, Space>>,
    ) {
        self.reborrow()
            .inner_collective_aligned_col_panel_iter(group)
    }
    pub fn collective_aligned_col_panel_iter_mut<const N: u32>(
        &'b mut self,
        group: impl Group + 'a,
    ) -> (
        ColPanelIterLooper<'a, W<T, N>, M<&'b mut T, Space>>,
        ColPanel<'a, DW<'a, T>, M<&'b mut T, Space>>,
    ) {
        self.reborrow_mut()
            .inner_collective_aligned_col_panel_iter(group)
    }
}
