pub mod gemm;
pub mod gmem;
pub mod group;
mod lane_to_coord;
mod ldsm;
pub mod mma;
mod pair;
use std::marker::PhantomData;

pub use pair::TilePair;

use maize_core::{
    codegen::typed_func::FnCodegen,
    inkwell::{
        context::ContextRef,
        types::{ArrayType, BasicType},
        values::{AnyValueEnum, ArrayValue},
    },
    intrinsics::cuda::ldsm::call_ldsm_x4,
    ty::{
        Addrspace, AlignedTy, AnyTy, BF16, ContiguousUniformTy, M, P, R, SizedTy, StructReflectTy,
        Ty, U32, V, ValTy, cuda::Shared,
    },
    val::Val,
};

pub struct W<T, const N: u32>(PhantomData<T>);
pub struct DW<'a, T>(Val<'a, U32>, PhantomData<T>);
impl<T, const N: u32> Clone for W<T, N> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}
impl<T> Clone for DW<'_, T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<T, const N: u32> Copy for W<T, N> {}
impl<T> Copy for DW<'_, T> {}

pub trait Window: Copy {
    type ElemT: SizedTy;
    fn size<'v>(&self, cx: &'v FnCodegen) -> Val<'v, U32>
    where
        Self: 'v;
}

pub trait FixedWidthWindow: Window {
    const WIDTH: u32;
}

impl<T: SizedTy, const N: u32> Window for W<T, N> {
    type ElemT = T;
    fn size<'v>(&self, cx: &'v FnCodegen) -> Val<'v, U32>
    where
        Self: 'v,
    {
        cx.constant_from(N)
    }
}
impl<T: SizedTy> Window for DW<'_, T> {
    type ElemT = T;
    fn size<'v>(&self, _: &'v FnCodegen) -> Val<'v, U32>
    where
        Self: 'v,
    {
        self.0
    }
}
impl<T: SizedTy, const N: u32> FixedWidthWindow for W<T, N> {
    const WIDTH: u32 = N;
}

impl<T, const N: u32> W<T, N> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a, T> DW<'a, T> {
    pub fn new(val: Val<'a, U32>) -> Self {
        Self(val, PhantomData)
    }
}

pub trait TileTy {
    type ElemT: SizedTy;
}
pub trait ConstSizeTileTy: TileTy {
    const ROWS: u32;
    const COLS: u32;
}
pub trait WarpFragTileTy: ConstSizeTileTy {
    type FragT: ContiguousUniformTy<ElemT = Self::ElemT>;
}

pub struct BF16_16x16;
impl TileTy for BF16_16x16 {
    type ElemT = BF16;
}
impl ConstSizeTileTy for BF16_16x16 {
    const ROWS: u32 = 16;
    const COLS: u32 = 16;
}
impl WarpFragTileTy for BF16_16x16 {
    type FragT = V<BF16, { (Self::ROWS * Self::COLS / 32) as usize }>;
}
pub struct BF16_8x8;
impl TileTy for BF16_8x8 {
    type ElemT = BF16;
}
impl ConstSizeTileTy for BF16_8x8 {
    const ROWS: u32 = 8;
    const COLS: u32 = 8;
}
impl WarpFragTileTy for BF16_8x8 {
    type FragT = V<BF16, { (Self::ROWS * Self::COLS / 32) as usize }>;
}
impl StructReflectTy for Tile<BF16_16x16> {
    type RealStruct = [u16; 16 * 16];
}

pub trait WarpCollectiveTileTy: WarpFragTileTy + Sized {
    type LoadElement: ContiguousUniformTy<ElemT = Self::ElemT> + SizedTy + Copy;
    type StoreElement: ContiguousUniformTy<ElemT = Self::ElemT> + SizedTy + Copy;

    fn row_col_for_load(lane: Val<'_, U32>) -> impl Iterator<Item = (Val<'_, U32>, Val<'_, U32>)>;
    fn row_col_for_store(lane: Val<'_, U32>) -> impl Iterator<Item = (Val<'_, U32>, Val<'_, U32>)>;
    fn row_col_to_offset<'a>(row: Val<'a, U32>, col: Val<'a, U32>) -> Val<'a, U32> {
        Self::COLS * row + col
    }

    unsafe fn raw_collective_load<Space: Addrspace>(
        val: Val<'_, P<*const Self::LoadElement, Space>>,
    ) -> Val<'_, Self::LoadElement>;
    unsafe fn raw_collective_store<Space: Addrspace>(
        ptr: Val<'_, P<*mut Self::StoreElement, Space>>,
        val: Val<'_, Self::StoreElement>,
    );

    fn collective_load<'a, 'b, Space: Addrspace>(
        ptr: &Val<'a, R<&'b Tile<Self>, Space>>,
        lane: Val<'a, U32>,
    ) -> Val<'a, Self::FragT> {
        let loaded_vals = Self::row_col_for_load(lane)
            .map(|(row, col)| {
                let raw_ptr = ptr.as_ptr().ptr_cast::<Self::ElemT>();
                let offset = Self::row_col_to_offset(row, col);
                let raw_ptr = unsafe { raw_ptr.add(offset) };
                let raw_ptr = raw_ptr.ptr_cast::<Self::LoadElement>();
                unsafe { Self::raw_collective_load(raw_ptr) }
            })
            .map(|full| {
                let iter = 0..Self::LoadElement::size();
                iter.map(move |i| Self::LoadElement::element(full, i as _))
            })
            .flatten()
            .collect::<Vec<_>>();

        Self::FragT::try_from_elements(loaded_vals.into_iter())
            .expect("Incorreect number of values yielded!")
    }
    fn collective_store<'a, 'b, Space: Addrspace>(
        ptr: Val<'a, M<&'b mut Tile<Self>, Space>>,
        val: Val<'a, Self::FragT>,
        lane: Val<'a, U32>,
    ) {
        let mut elem_iter = Self::FragT::into_element_iter(val);
        for (row, col) in Self::row_col_for_store(lane) {
            let raw_ptr = ptr.as_ptr_mut().ptr_cast::<Self::ElemT>();
            let offset = Self::row_col_to_offset(row, col);
            let raw_ptr = unsafe { raw_ptr.add(offset) };
            let raw_ptr = raw_ptr.ptr_cast::<Self::StoreElement>();
            let elements_for_store = (0..Self::StoreElement::size())
                .map(|_| elem_iter.next().expect("Size should match!"));
            let val = Self::StoreElement::try_from_elements(elements_for_store)
                .expect("This iterator has the correct size");
            unsafe { Self::raw_collective_store(raw_ptr, val) };
        }
    }
}

impl WarpCollectiveTileTy for BF16_16x16 {
    type LoadElement = V<BF16, 8>;
    type StoreElement = V<BF16, 2>;

    fn row_col_for_load(lane: Val<'_, U32>) -> impl Iterator<Item = (Val<'_, U32>, Val<'_, U32>)> {
        let subtile_idx = lane / 8;
        let st_row = subtile_idx % 2;
        let st_col = subtile_idx / 2;
        let row = lane % 8 + 8 * st_row;
        let col = 8 * st_col;
        [(row, col)].into_iter()
    }

    fn row_col_for_store(lane: Val<'_, U32>) -> impl Iterator<Item = (Val<'_, U32>, Val<'_, U32>)> {
        let subtile_row = lane / 4;
        let subtile_col = (lane % 4) * 2;
        [
            (subtile_row + 0, subtile_col + 0),
            (subtile_row + 8, subtile_col + 0),
            (subtile_row + 0, subtile_col + 8),
            (subtile_row + 8, subtile_col + 8),
        ]
        .into_iter()
    }
    fn row_col_to_offset<'a>(row: Val<'a, U32>, col: Val<'a, U32>) -> Val<'a, U32> {
        let st_row = row % 8;
        let st_col = col % 8;
        let col_half = col / 8;
        let row_half = row / 8;
        ((col_half * 2 + row_half) * 8 + st_row * 8) + st_col
    }

    unsafe fn raw_collective_load<Space: Addrspace>(
        val: Val<'_, P<*const Self::LoadElement, Space>>,
    ) -> Val<'_, Self::LoadElement> {
        assert_eq!(Space::AS_U16, Shared::AS_U16);
        unsafe { call_ldsm_x4(val.addrspace_cast().ptr_cast()).bitcast() }
    }

    unsafe fn raw_collective_store<Space: Addrspace>(
        ptr: Val<'_, P<*mut Self::StoreElement, Space>>,
        val: Val<'_, Self::StoreElement>,
    ) {
        unsafe { ptr.write(val) };
    }
}

pub struct Tile<T>(PhantomData<T>);

impl<T: ConstSizeTileTy> AnyTy for Tile<T> {
    type AnyType<'ctx> = ArrayType<'ctx>;
    fn any_ty<'ctx>(ctx: ContextRef<'ctx>) -> Self::AnyType<'ctx> {
        T::ElemT::ty(ctx).array_type(T::COLS * T::ROWS)
    }
}

impl<T: ConstSizeTileTy> ValTy for Tile<T> {
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

impl<T: ConstSizeTileTy> AlignedTy for Tile<T> {
    const ALIGN: u32 = T::ElemT::ALIGN;
}

impl<T: ConstSizeTileTy> SizedTy for Tile<T> {
    const SIZE: u32 = T::ROWS * T::COLS * T::ElemT::SIZE;
}

impl<T> StructReflectTy for Tile<T>
where
    T: StructReflectTy + ConstSizeTileTy,
{
    type RealStruct = [T::ElemT; 0];
}
