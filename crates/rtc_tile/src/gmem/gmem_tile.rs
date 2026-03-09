use rtc_types::{
    codegen::{loops::TransformLooper, typed_func::FnCodegen},
    ty::{DereferencableTy, SizedTy, U32},
    val::Val,
};

use crate::{
    W, Window,
    gmem::{ColPanel, RowPanel},
    group::{Group, NullGroup},
};

pub struct GmemTile<'a, RowWindow, ColWindow, Ptr> {
    row_window: RowWindow,
    col_window: ColWindow,
    pub ptr: Val<'a, Ptr>,
    #[expect(unused)]
    stride_per_row: Val<'a, U32>,
}

impl<'a, RowWindow: Window + 'a, ColWindow: Window + 'a, Ptr>
    GmemTile<'a, RowWindow, ColWindow, Ptr>
{
    pub fn row_size(&self) -> Val<'a, U32> {
        self.row_window.size(self.ptr.cx())
    }
    pub fn col_size(&self) -> Val<'a, U32> {
        self.col_window.size(self.ptr.cx())
    }
}

pub struct RowTileLooper<'a, RowWindow, ColWindow, Ptr> {
    row_window: RowWindow,
    col_window: ColWindow,
    ptr: Val<'a, Ptr>,
    init_row: Val<'a, U32>,
    rows_per_iter: Val<'a, U32>,
    stride_per_row: Val<'a, U32>,
    last_row: Val<'a, U32>,
}

pub struct ColTileLooper<'a, RowWindow, ColWindow, Ptr> {
    row_window: RowWindow,
    col_window: ColWindow,
    ptr: Val<'a, Ptr>,
    init_col: Val<'a, U32>,
    cols_per_iter: Val<'a, U32>,
    num_cols: Val<'a, U32>,
}

impl<'ctx, RowWindow, ColWindow, Ptr> TransformLooper
    for RowTileLooper<'ctx, RowWindow, ColWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
    ColWindow: Window<ElemT = Ptr::Pointee>,
{
    type DecisionItemT = U32;
    type ItemT<'a>
        = GmemTile<'a, RowWindow, ColWindow, Ptr>
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
        self.init_row
    }

    fn decision_fn<'a>(
        &self,
        curr_row: Val<'a, Self::DecisionItemT>,
    ) -> Val<'a, rtc_types::ty::Bool>
    where
        Self: 'a,
    {
        curr_row.lt(self.last_row)
    }

    fn transform<'a>(&self, curr_row: Val<'a, Self::DecisionItemT>) -> Self::ItemT<'a>
    where
        Self: 'a,
    {
        let offset = curr_row * self.stride_per_row;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            row_window: self.row_window,
            col_window: self.col_window,
            ptr: panel_init_ptr,
            stride_per_row: self.stride_per_row,
        }
    }

    fn update_fn<'a>(&self, curr_row: Val<'a, Self::DecisionItemT>) -> Val<'a, Self::DecisionItemT>
    where
        Self: 'a,
    {
        unsafe { curr_row.add_unchecked(self.rows_per_iter) }
    }
}

impl<'ctx, RowWindow, ColWindow, Ptr> TransformLooper
    for ColTileLooper<'ctx, RowWindow, ColWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
    ColWindow: Window<ElemT = Ptr::Pointee>,
{
    type DecisionItemT = U32;
    type ItemT<'a>
        = GmemTile<'a, RowWindow, ColWindow, Ptr>
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
        self.init_col
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
            row_window: self.row_window,
            col_window: self.col_window,
            ptr: panel_init_ptr,
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

impl<'a, 'b, RowWindow, Ptr> RowPanel<'a, RowWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
{
    pub fn collective_gmem_tiles<const N: u32>(
        &'b mut self,
        group: impl Group + 'a,
    ) -> impl TransformLooper<
        ItemT<'b> = GmemTile<'b, RowWindow, W<Ptr::Pointee, N>, Ptr::Parametrized<'b>>,
    > {
        let (index, size) = group.index_size();
        let cols_per_iter = size * N;
        let curr_col = index * N;
        ColTileLooper {
            row_window: self.row_window,
            col_window: W::new(),
            ptr: Ptr::parametrize(&mut self.ptr),
            init_col: curr_col,
            cols_per_iter,
            num_cols: self.num_cols,
        }
    }

    pub fn gmem_tiles<const N: u32>(
        &'b mut self,
    ) -> impl TransformLooper<
        ItemT<'b> = GmemTile<'b, RowWindow, W<Ptr::Pointee, N>, Ptr::Parametrized<'b>>,
    > {
        self.collective_gmem_tiles(NullGroup(self.ptr.cx()))
    }
}

impl<'a, 'b, ColWindow, Ptr> ColPanel<'a, ColWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    ColWindow: Window<ElemT = Ptr::Pointee>,
{
    pub fn collective_gmem_tiles<const N: u32>(
        &'b mut self,
        group: impl Group + 'a,
    ) -> impl TransformLooper<
        ItemT<'b> = GmemTile<'b, W<Ptr::Pointee, N>, ColWindow, Ptr::Parametrized<'b>>,
    > {
        let (index, size) = group.index_size();
        let rows_per_iter = size * N;
        let init_row = index * N;
        RowTileLooper {
            row_window: W::new(),
            col_window: self.col_window,
            ptr: Ptr::parametrize(&mut self.ptr),
            init_row,
            rows_per_iter,
            stride_per_row: self.stride_per_row,
            last_row: self.num_rows,
        }
    }

    pub fn gmem_tiles<const N: u32>(
        &'b mut self,
    ) -> impl TransformLooper<
        ItemT<'b> = GmemTile<'b, W<Ptr::Pointee, N>, ColWindow, Ptr::Parametrized<'b>>,
    > {
        self.collective_gmem_tiles(NullGroup(self.ptr.cx()))
    }
}

impl<'a, Elem, const N: u32, const M: u32, Ptr> GmemTile<'a, W<Elem, M>, W<Elem, N>, Ptr> {
    pub fn do_something(self) {}
}
