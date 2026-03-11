use rtc_types::{
    codegen::{
        loops::{Looper, TransformLooper},
        typed_func::FnCodegen,
    },
    intrinsics::cuda::cp_async::CpAsyncToken,
    kernel_assert, kernel_print,
    ty::{
        DereferencableTy, M, P, SizedTy, U32, Void,
        cuda::{Global, Shared},
    },
    val::Val,
};

use crate::{
    Tile, W, WarpSmemLoadTileTy, Window,
    gmem::{ColPanel, RowPanel},
    group::{ConstSizeGroup, Group, NullGroup},
};

pub struct GmemTile<'a, RowWindow, ColWindow, Ptr> {
    row_window: RowWindow,
    col_window: ColWindow,
    pub ptr: Val<'a, Ptr>,
    stride_per_row: Val<'a, U32>,
}

impl<'a, RowWindow: Window, ColWindow, Ptr, T> GmemTile<'a, RowWindow, ColWindow, Ptr>
where
    T: SizedTy,
    RowWindow: Window<ElemT = T> + 'a,
    ColWindow: Window<ElemT = T> + 'a,
    Ptr: DereferencableTy<Pointee = T>,
{
    pub fn row_size(&self) -> Val<'a, U32> {
        self.row_window.size(self.ptr.cx())
    }
    pub fn col_size(&self) -> Val<'a, U32> {
        self.col_window.size(self.ptr.cx())
    }

    pub fn collective_cp_async<'b, SmemTile: WarpSmemLoadTileTy<ElemT = T>, G: ConstSizeGroup>(
        &self,
        _: &CpAsyncToken,
        tile: Val<'a, M<&'b mut Tile<SmemTile>, Shared>>,
        group: G,
        cp_size: u8,
        zero_fill_oob: bool,
    ) -> Val<'a, M<&'b mut Tile<SmemTile>, Shared>>
    where
        Ptr: DereferencableTy<Space = Global>,
    {
        let smem_nrows = SmemTile::ROWS;
        let smem_ncols = SmemTile::COLS;
        let row_size = self.row_size();
        let col_size = self.col_size();
        let cx = tile.cx();
        let dyn_rows = if let Some(const_size) = row_size.try_const_int() {
            assert_eq!(
                const_size, smem_nrows,
                "Cannot copy between tiles of different row size"
            );
            false
        } else {
            let smem_rows = cx.constant_from(smem_nrows);
            (!row_size.le(smem_rows)).branch(|| {
                kernel_print!(
                    "Attempted to copy a global tile with {} rows into a smem tile with {} rows\n",
                    row_size,
                    smem_rows,
                );
                kernel_assert!(row_size.le(smem_rows));
            });
            true
        };
        let dyn_cols = if let Some(const_size) = col_size.try_const_int() {
            assert_eq!(
                const_size, smem_ncols,
                "Cannot copy between tiles of different row size"
            );
            false
        } else {
            let smem_ncols = cx.constant_from(smem_ncols);
            (!col_size.le(smem_ncols)).branch(|| {
                kernel_print!(
                    "Attempted to copy a global tile with {} cols into a smem tile with {} cols\n",
                    col_size,
                    smem_ncols,
                );
                kernel_assert!(col_size.le(smem_ncols));
            });
            true
        };

        let cp_async_zfill = || match cp_size {
            4 => cx.get_intrinsic::<Void, (P<*mut T, Shared>, P<*const T, Global>, U32)>(
                "llvm.nvvm.cp.async.ca.shared.global.4.s",
                true,
            ),
            8 => cx.get_intrinsic::<Void, (P<*mut T, Shared>, P<*const T, Global>, U32)>(
                "llvm.nvvm.cp.async.ca.shared.global.8.s",
                true,
            ),
            16 => cx.get_intrinsic::<Void, (P<*mut T, Shared>, P<*const T, Global>, U32)>(
                "llvm.nvvm.cp.async.ca.shared.global.16.s",
                true,
            ),
            _ => panic!("Only 4,8,16 size cp.async is allowed"),
        };
        let cp_async = || match cp_size {
            4 => cx.get_intrinsic::<Void, (P<*mut T, Shared>, P<*const T, Global>)>(
                "llvm.nvvm.cp.async.ca.shared.global.4",
                true,
            ),
            8 => cx.get_intrinsic::<Void, (P<*mut T, Shared>, P<*const T, Global>)>(
                "llvm.nvvm.cp.async.ca.shared.global.8",
                true,
            ),
            16 => cx.get_intrinsic::<Void, (P<*mut T, Shared>, P<*const T, Global>)>(
                "llvm.nvvm.cp.async.ca.shared.global.16",
                true,
            ),
            _ => panic!("Only 4,8,16 size cp.async is allowed"),
        };

        let elem_size = T::SIZE;
        assert!(cp_size as u32 % elem_size == 0);
        let numel_per_cp = cp_size as u32 / elem_size;
        let row_size_bytes = elem_size * smem_ncols;
        let nthreads_per_row = row_size_bytes / cp_size as u32;
        let threads_in_group = group.const_size();

        let multiple_rows_at_once = threads_in_group % nthreads_per_row == 0;
        let loop_per_row = nthreads_per_row % threads_in_group == 0;
        assert!(multiple_rows_at_once || loop_per_row);

        let (index, _) = group.index_size();

        if multiple_rows_at_once {
            let rows_at_once = threads_in_group / nthreads_per_row;
            let copies_per_thread = smem_nrows / rows_at_once;
            let column_index = index % nthreads_per_row;
            let row_offset = index / nthreads_per_row;

            let do_copy_row = |row, just_zero| {
                let col_offset_elements = column_index * numel_per_cp;
                let smem_row_ptr = unsafe {
                    tile.as_ptr_mut()
                        .ptr_cast::<SmemTile::ElemT>()
                        .add(row * smem_ncols + col_offset_elements)
                };
                let gmem_ptr = unsafe {
                    Ptr::to_raw(&self.ptr).add(row * self.stride_per_row + col_offset_elements)
                };
                if just_zero {
                    let zero = cx.constant_from(0u32);
                    cx.call_void_fn(cp_async_zfill(), (smem_row_ptr, gmem_ptr, zero));
                } else {
                    if dyn_cols {
                        let copy_size = col_size.const_like(cp_size as u32);
                        let valid_bytes = col_offset_elements
                            .lt(col_size)
                            .then(|| ((col_size - col_offset_elements) * T::SIZE).min(copy_size))
                            .or(col_size.zero());
                        cx.call_void_fn(cp_async_zfill(), (smem_row_ptr, gmem_ptr, valid_bytes));
                    } else {
                        cx.call_void_fn(cp_async(), (smem_row_ptr, gmem_ptr));
                    }
                }
            };

            if dyn_rows {
                let zero = cx.constant_from(0u32);
                let copies_per_thread = cx.constant_from(copies_per_thread);
                (zero..copies_per_thread).for_every_value(|idx| {
                    let row = row_offset + idx * rows_at_once;
                    let maybe_else = row.lt(row_size).branch(|| do_copy_row(row, false));
                    if zero_fill_oob {
                        maybe_else.or_else(|| do_copy_row(row, true));
                    }
                });
            } else {
                for idx in 0..copies_per_thread {
                    let row = row_offset + idx * rows_at_once;
                    do_copy_row(row, false);
                }
            }
        } else {
            assert!(loop_per_row);
            let copies_per_row_per_thread = nthreads_per_row / threads_in_group;
            let col_offset_elements = numel_per_cp * index;
            let stride_per_iter = numel_per_cp * threads_in_group;
            let do_copy_row = |row, just_zero| {
                for col_cp_idx in 0..copies_per_row_per_thread {
                    let col_offset = col_offset_elements + col_cp_idx * stride_per_iter;
                    let offset = row * self.stride_per_row + col_offset;

                    let gmem_ptr = unsafe { Ptr::to_raw(&self.ptr).add(offset) };
                    let smem_ptr = unsafe {
                        tile.as_ptr_mut()
                            .ptr_cast::<SmemTile::ElemT>()
                            .add(row * smem_ncols + col_offset)
                    };
                    if just_zero {
                        let zero = cx.constant_from(0u32);
                        cx.call_void_fn(cp_async_zfill(), (smem_ptr, gmem_ptr, zero));
                    } else {
                        if dyn_cols {
                            let copy_size = col_size.const_like(cp_size as u32);
                            let valid_bytes = col_offset
                                .lt(col_size)
                                .then(|| ((col_size - col_offset) * T::SIZE).min(copy_size))
                                .or(col_size.zero());
                            cx.call_void_fn(cp_async_zfill(), (smem_ptr, gmem_ptr, valid_bytes));
                        } else {
                            cx.call_void_fn(cp_async(), (smem_ptr, gmem_ptr));
                        }
                    }
                }
            };

            if dyn_rows {
                for row in 0..smem_nrows {
                    let row = cx.constant_from(row);
                    let maybe_else = row.lt(row_size).branch(|| do_copy_row(row, false));
                    if zero_fill_oob {
                        maybe_else.or_else(|| do_copy_row(row, true));
                    }
                }
            } else {
                for row in 0..smem_nrows {
                    let row = cx.constant_from(row);
                    do_copy_row(row, false);
                }
            }
        }

        tile
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

impl<'ctx, RowWindow, ColWindow, Ptr> TransformLooper<'ctx>
    for RowTileLooper<'ctx, RowWindow, ColWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
    ColWindow: Window<ElemT = Ptr::Pointee>,
{
    type DecisionItemT = U32;
    type ItemT = GmemTile<'ctx, RowWindow, ColWindow, Ptr>;
    fn cx(&self) -> &'ctx FnCodegen {
        self.ptr.cx()
    }

    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        self.init_row
    }

    fn decision_fn(
        &self,
        curr_row: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, rtc_types::ty::Bool> {
        curr_row.lt(self.last_row)
    }

    fn transform(&self, curr_row: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        let offset = curr_row * self.stride_per_row;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            row_window: self.row_window,
            col_window: self.col_window,
            ptr: panel_init_ptr,
            stride_per_row: self.stride_per_row,
        }
    }

    fn update_fn<'a>(
        &self,
        curr_row: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT> {
        unsafe { curr_row.add_unchecked(self.rows_per_iter) }
    }

    fn step_n(&mut self, n: usize) {
        let n: u32 = n.try_into().expect("usize -> u32 overflow");
        self.init_row = self.init_row + self.rows_per_iter * n;
    }
}

impl<'ctx, RowWindow, ColWindow, Ptr> TransformLooper<'ctx>
    for ColTileLooper<'ctx, RowWindow, ColWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
    ColWindow: Window<ElemT = Ptr::Pointee>,
{
    type DecisionItemT = U32;
    type ItemT = GmemTile<'ctx, RowWindow, ColWindow, Ptr>;
    fn cx(&self) -> &'ctx FnCodegen {
        self.ptr.cx()
    }

    fn init_decision(&self) -> Val<'ctx, Self::DecisionItemT> {
        self.init_col
    }

    fn decision_fn(
        &self,
        curr_col: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, rtc_types::ty::Bool> {
        curr_col.lt(self.num_cols)
    }

    fn transform(&self, curr_col: Val<'ctx, Self::DecisionItemT>) -> Self::ItemT {
        let offset = curr_col;
        let panel_init_ptr = unsafe { Ptr::on_underlying_raw(&self.ptr, |p| p.add(offset)) };
        Self::ItemT {
            row_window: self.row_window,
            col_window: self.col_window,
            ptr: panel_init_ptr,
            stride_per_row: self.num_cols,
        }
    }

    fn update_fn<'a>(
        &self,
        curr_col: Val<'ctx, Self::DecisionItemT>,
    ) -> Val<'ctx, Self::DecisionItemT> {
        unsafe { curr_col.add_unchecked(self.cols_per_iter) }
    }

    fn step_n(&mut self, n: usize) {
        let n: u32 = n.try_into().expect("usize -> u32 overflow");
        self.init_col = self.init_col + self.cols_per_iter * n;
    }
}

impl<'a, 'b, RowWindow, Ptr> RowPanel<'a, RowWindow, Ptr>
where
    Ptr: DereferencableTy<Pointee: SizedTy>,
    RowWindow: Window<ElemT = Ptr::Pointee>,
{
    pub fn into_collective_gmem_tiles<const N: u32>(
        self,
        group: impl Group + 'a,
    ) -> ColTileLooper<'a, RowWindow, W<Ptr::Pointee, N>, Ptr> {
        let (index, size) = group.index_size();
        let cols_per_iter = size * N;
        let curr_col = index * N;
        ColTileLooper {
            row_window: self.row_window,
            col_window: W::new(),
            ptr: self.ptr,
            init_col: curr_col,
            cols_per_iter,
            num_cols: self.num_cols,
        }
    }
    pub fn collective_gmem_tiles<const N: u32>(
        &'b mut self,
        group: impl Group + 'a,
    ) -> ColTileLooper<'a, RowWindow, W<Ptr::Pointee, N>, Ptr::Parametrized<'b>> {
        let (index, size) = group.index_size();
        let cols_per_iter = size * N;
        let curr_col = index * N;
        ColTileLooper {
            row_window: self.row_window,
            col_window: W::new(),
            ptr: Ptr::parametrize_by_ref(&mut self.ptr),
            init_col: curr_col,
            cols_per_iter,
            num_cols: self.num_cols,
        }
    }

    pub fn gmem_tiles<const N: u32>(
        &'b mut self,
    ) -> ColTileLooper<'a, RowWindow, W<Ptr::Pointee, N>, Ptr::Parametrized<'b>> {
        self.collective_gmem_tiles(NullGroup(self.ptr.cx()))
    }

    pub fn into_gmem_tiles<const N: u32>(
        self,
    ) -> ColTileLooper<'a, RowWindow, W<Ptr::Pointee, N>, Ptr> {
        let cx = self.ptr.cx();
        self.into_collective_gmem_tiles::<N>(NullGroup(cx))
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
    ) -> RowTileLooper<'a, W<Ptr::Pointee, N>, ColWindow, Ptr::Parametrized<'b>> {
        let (index, size) = group.index_size();
        let rows_per_iter = size * N;
        let init_row = index * N;
        RowTileLooper {
            row_window: W::new(),
            col_window: self.col_window,
            ptr: Ptr::parametrize_by_ref(&mut self.ptr),
            init_row,
            rows_per_iter,
            stride_per_row: self.stride_per_row,
            last_row: self.num_rows,
        }
    }

    pub fn gmem_tiles<const N: u32>(
        &'b mut self,
    ) -> RowTileLooper<'a, W<Ptr::Pointee, N>, ColWindow, Ptr::Parametrized<'b>> {
        self.collective_gmem_tiles(NullGroup(self.ptr.cx()))
    }
}

impl<'a, Elem, const N: u32, const M: u32, Ptr> GmemTile<'a, W<Elem, { M }>, W<Elem, N>, Ptr> {
    pub fn do_something(self) {}
}
