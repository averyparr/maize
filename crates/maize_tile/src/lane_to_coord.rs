// use maize_core::{ty::U32, val::Val};

// use crate::bf16_tile::MmaBf16_16x16;

// pub trait LaneToIndex {
//     const GROUP_SIZE: u32;
//     fn lane_to_logical(
//         lane: Val<'_, U32>,
//     ) -> impl IntoIterator<Item = (Val<'_, U32>, Val<'_, U32>)>;
// }

// impl LaneToIndex for MmaBf16_16x16 {
//     const GROUP_SIZE: u32 = 32;
//     fn lane_to_logical(
//         lane: Val<'_, U32>,
//     ) -> impl IntoIterator<Item = (Val<'_, U32>, Val<'_, U32>)> {
//         let row = lane % 16;
//         let col_xormask = row % 2;
//         let col_block = (lane / 16) ^ col_xormask;
//         [(row, col_block)]
//     }
// }

// pub trait IndexToOffset {
//     fn logical_to_physical<'a>(self, row: Val<'a, U32>, col: Val<'a, U32>) -> Val<'a, U32>
//     where
//         Self: 'a;
// }

// #[derive(Clone)]
// struct RowMajorLay<'a> {
//     offset_per_row: Val<'a, U32>,
// }
// struct ConstRowMajor(u32);

// impl<'slf> IndexToOffset for RowMajorLay<'slf> {
//     fn logical_to_physical<'a>(self, row: Val<'a, U32>, col: Val<'a, U32>) -> Val<'a, U32>
//     where
//         Self: 'a,
//     {
//         self.offset_per_row * row + col
//     }
// }
// impl IndexToOffset for ConstRowMajor {
//     fn logical_to_physical<'a>(self, row: Val<'a, U32>, col: Val<'a, U32>) -> Val<'a, U32>
//     where
//         Self: 'a,
//     {
//         self.0 * row + col
//     }
// }
