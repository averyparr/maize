pub mod bf16_tile;
pub mod gmem;
pub mod group;
mod lane_to_coord;
mod ldsm;
pub mod mma;
use std::marker::PhantomData;

use rtc_types::{
    inkwell::{
        context::ContextRef,
        types::{ArrayType, BasicType},
        values::{AnyValueEnum, ArrayValue},
    },
    intrinsics::cuda::ldsm::{call_ldsm_x1, call_ldsm_x4},
    ty::{AlignedTy, AnyTy, BF16, M, SizedTy, Ty, U32, U128, V, ValTy, cuda::Shared},
    val::Val,
};

pub struct W<T, const N: u32>(PhantomData<T>);
impl<T, const N: u32> Clone for W<T, N> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
impl<T, const N: u32> Copy for W<T, N> {}
impl<T, const N: u32> W<T, N> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

trait FixedWidthWindow: Copy {
    type ElemT: ValTy;
    const WIDTH: u32;
}

impl<T: ValTy, const N: u32> FixedWidthWindow for W<T, N> {
    type ElemT = T;
    const WIDTH: u32 = N;
}

pub trait WarpTileTy {
    const ROWS: u32;
    const COLS: u32;
    type ElemT: SizedTy;
    type FragT: SizedTy;
}

pub struct BF16_16x16;
impl WarpTileTy for BF16_16x16 {
    const COLS: u32 = 16;
    const ROWS: u32 = 16;
    type ElemT = BF16;
    type FragT = V<BF16, { Self::COLS as usize / 2 }>;
}
pub struct BF16_8x8;
impl WarpTileTy for BF16_8x8 {
    const COLS: u32 = 8;
    const ROWS: u32 = 8;
    type ElemT = BF16;
    type FragT = V<BF16, 2>;
}

pub trait WarpSmemLoadTileTy: WarpTileTy + Sized {
    fn collective_load<'a, 'b>(
        ptr: &mut Val<'a, Shared<M<&'b mut Tile<Self>>>>,
        lane: Val<'a, U32>,
    ) -> Val<'a, Self::FragT>;
}

impl WarpSmemLoadTileTy for BF16_16x16 {
    fn collective_load<'a, 'b>(
        ptr: &mut Val<'a, Shared<M<&'b mut Tile<Self>>>>,
        lane: Val<'a, U32>,
    ) -> Val<'a, Self::FragT> {
        let tile_ptr = ptr.reborrow_mut().as_mut_ptr();
        let elem_ptr = tile_ptr.ptr_cast::<U128>();
        let row_offset_in_subtile = lane % 8;
        let subtile_id = lane / 8;
        let offset = row_offset_in_subtile + subtile_id * 8;
        let offset_elem_ptr = unsafe { elem_ptr.add(offset) };

        let i32_ret = unsafe { call_ldsm_x4(offset_elem_ptr) };
        unsafe { i32_ret.bitcast() }
    }
}

impl WarpSmemLoadTileTy for BF16_8x8 {
    fn collective_load<'a, 'b>(
        ptr: &mut Val<'a, Shared<M<&'b mut Tile<Self>>>>,
        lane: Val<'a, U32>,
    ) -> Val<'a, Self::FragT> {
        let tile_ptr = ptr.reborrow_mut().as_mut_ptr();
        let elem_ptr = tile_ptr.ptr_cast::<U128>();
        let row_offset_in_subtile = lane % 8;
        let subtile_id = lane / 8;
        let offset = row_offset_in_subtile + subtile_id * 8;
        let offset_elem_ptr = unsafe { elem_ptr.add(offset) };

        let i32_ret = unsafe { call_ldsm_x1(offset_elem_ptr) };
        unsafe { i32_ret.bitcast() }
    }
}

pub struct Tile<T>(PhantomData<T>);

impl<T: WarpTileTy> AnyTy for Tile<T> {
    type AnyType<'ctx> = ArrayType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        T::ElemT::ty(ctx).array_type(T::COLS * T::ROWS)
    }
}

impl<T: WarpTileTy> ValTy for Tile<T> {
    type Value<'ctx> = ArrayValue<'ctx>;

    fn undef<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).get_undef()
    }

    fn zeros<'ctx>(ctx: ContextRef<'ctx>) -> Self::Value<'ctx> {
        Self::ty(ctx).const_zero()
    }

    fn try_type_val<'ctx>(val: AnyValueEnum<'ctx>) -> Option<Self::Value<'ctx>> {
        match val {
            AnyValueEnum::ArrayValue(val) => Some(val),
            _ => None,
        }
    }
}

impl<T: WarpTileTy> AlignedTy for Tile<T> {
    const ALIGN: u32 = T::ElemT::ALIGN;
}

impl<T: WarpTileTy> SizedTy for Tile<T> {
    const SIZE: u32 = T::ROWS * T::COLS * T::ElemT::SIZE;
}
