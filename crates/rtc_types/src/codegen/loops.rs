// use core::ops::Range;

// use crate::{
//     ty::{SizedTy, Ty, U32},
//     val::Val,
// };

// trait Looper {
//     type ItemT: SizedTy;
//     fn init(&self) -> Val<'_, Self::ItemT>;
//     fn for_each(self, f: impl Fn(Val<'_, Self::ItemT>));
// }

// impl<'a> Looper for Range<Val<'a, U32>> {
//     type ItemT = U32;
//     fn init(&self) -> Val<'_, Self::ItemT> {
//         self.start
//     }
//     fn for_each(self, f: impl Fn(Val<'_, Self::ItemT>)) {
//         let cx = self.start.cx();
//         let ctx = cx.ctx();
//         let item_ty = Self::ItemT::ty(ctx);
//         let mut range_val = self.init().with_storage();
//         let range_ptr = range_val.as_mut();
//     }
// }
