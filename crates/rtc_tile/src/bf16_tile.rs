use rtc_types::{
    intrinsics::cuda::ldsm::call_ldsm_x4,
    ty::{BF16, M, SizedTy, U32, U128, V, cuda::Shared},
    val::Val,
};

use crate::{Tile, WarpSmemLoadTileTy, WarpTileTy};

pub struct MmaBf16_16x16;

impl WarpTileTy for MmaBf16_16x16 {
    const ROWS: u32 = 16;
    const COLS: u32 = 16;
    type ElemT = BF16;
    type FragT = V<Self::ElemT, 8>;
}

impl WarpSmemLoadTileTy for MmaBf16_16x16 {
    fn collective_load<'a, 'b>(
        ptr: &mut Val<'a, M<&'b mut Tile<Self>, Shared>>,
        lane: Val<'a, U32>,
    ) -> Val<'a, Self::FragT> {
        let row = lane % 16;
        let col_xormask = row % 2;
        let col_block = (lane / 16) ^ col_xormask;
        let tile_ptr = ptr.reborrow().as_ptr();
        let elem_ptr = tile_ptr.ptr_cast::<U128>();
        let offset_per_row = Self::COLS / (U128::SIZE / BF16::SIZE);
        let offset = col_block + row * offset_per_row;
        let offset_elem_ptr = unsafe { elem_ptr.add(offset) };

        let i32_ret = unsafe { call_ldsm_x4(offset_elem_ptr) };
        unsafe { i32_ret.bitcast() }
    }
}

// fn load_tile_with_selector()
