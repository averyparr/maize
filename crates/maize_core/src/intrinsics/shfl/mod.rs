use crate::{
    func::{FnArgs, FnRetTy},
    intrinsics::Intrinsic,
    tipe::{BF16, F8E4M3, F8E5M2, F16, Ty, V, VecTy},
    val::Val,
};

macro_rules! shfl_intrinsic {
    ($($struct_name: ident : $intrinsic: literal($($args: ty),*) -> $ret: ty),* $(,)?) => {
        $(
            pub struct $struct_name;
            impl Intrinsic for $struct_name {
                type Args = ($($args,)*);
                type Ret = $ret;
                fn call(
                    self,
                    args: <Self::Args as FnArgs>::ArgValues,
                ) -> <Self::Ret as FnRetTy>::RetVal {
                    let fn_ref = args.0.fn_ref().clone();
                    let func = fn_ref
                        .get_intrinsic::<Self::Ret, Self::Args>($intrinsic, false)
                        .expect("Intrinsic should exist");
                    fn_ref.call_extern(func, args, None)
                }
            }
        )*
    };
}

shfl_intrinsic!(
    ShflSyncBflyF32: "llvm.nvvm.shfl.sync.bfly.f32" (u32, f32, i32, u32) -> f32,
    ShflSyncBflyI32: "llvm.nvvm.shfl.sync.bfly.i32" (u32, i32, i32, u32) -> i32,
    ShflSyncDownF32: "llvm.nvvm.shfl.sync.down.f32" (u32, f32, i32, u32) -> f32,
    ShflSyncDownI32: "llvm.nvvm.shfl.sync.down.i32" (u32, i32, i32, u32) -> i32,
    ShflSyncUpF32: "llvm.nvvm.shfl.sync.up.f32" (u32, f32, i32, u32) -> f32,
    ShflSyncUpI32: "llvm.nvvm.shfl.sync.up.i32" (u32, i32, i32, u32) -> i32,
    ShflSyncIdxF32: "llvm.nvvm.shfl.sync.bfly.f32" (u32, f32, i32, u32) -> f32,
    ShflSyncIdxI32: "llvm.nvvm.shfl.sync.bfly.i32" (u32, i32, i32, u32) -> i32,
);

pub trait Shflable: Ty + Sized {
    fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self>;
    fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self>;
    fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self>;
    fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self>;
}

impl Shflable for f32 {
    fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyF32.call((mask, val, xor, c))
    }

    fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyF32.call((mask, val, diff, c))
    }

    fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyF32.call((mask, val, diff, c))
    }

    fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyF32.call((mask, val, idx, c))
    }
}

impl Shflable for i32 {
    fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyI32.call((mask, val, xor, c))
    }

    fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyI32.call((mask, val, diff, c))
    }

    fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyI32.call((mask, val, diff, c))
    }

    fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self> {
        let c = ((32 - width) << 8) | (width - 1);
        let c = mask.fn_ref().constant(c);
        ShflSyncBflyI32.call((mask, val, idx, c))
    }
}

macro_rules! shfl_impl_64b {
    ($($tipe: ty),*) => {
        $(
            impl Shflable for $tipe {
                fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self> {
                    let elements = val
                        .bitcast::<V<i32, 2>>()
                        .elements()
                        .map(|e| e.shfl_bfly(mask.copy(), xor.copy(), width));
                    Val::from_elements(elements).bitcast()
                }

                fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
                    let elements = val
                        .bitcast::<V<i32, 2>>()
                        .elements()
                        .map(|e| e.shfl_down(mask.copy(), diff.copy(), width));
                    Val::from_elements(elements).bitcast()
                }

                fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
                    let elements = val
                        .bitcast::<V<i32, 2>>()
                        .elements()
                        .map(|e| e.shfl_up(mask.copy(), diff.copy(), width));
                    Val::from_elements(elements).bitcast()
                }

                fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self> {
                    let elements = val
                        .bitcast::<V<i32, 2>>()
                        .elements()
                        .map(|e| e.shfl_idx(mask.copy(), idx.copy(), width));
                    Val::from_elements(elements).bitcast()
                }
            }
        )*
    };
}

shfl_impl_64b!(f64, i64, u64);

impl Shflable for u32 {
    fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self> {
        val.bitcast::<i32>().shfl_bfly(mask, xor, width).bitcast()
    }

    fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        val.bitcast::<i32>().shfl_down(mask, diff, width).bitcast()
    }

    fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        val.bitcast::<i32>().shfl_up(mask, diff, width).bitcast()
    }

    fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self> {
        val.bitcast::<i32>().shfl_idx(mask, idx, width).bitcast()
    }
}

macro_rules! shfl_impl_sub32b {
    ($cvt_ty: ty: $($tipes: ty),*) => {
    $(
        impl Shflable for $tipes {
            fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self> {
                val.bitcast::<$cvt_ty>()
                    .cvt::<i32>()
                    .shfl_bfly(mask, xor, width)
                    .cvt::<$cvt_ty>()
                    .bitcast()
            }

            fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
                val.bitcast::<$cvt_ty>()
                    .cvt::<i32>()
                    .shfl_down(mask, diff, width)
                    .cvt::<$cvt_ty>()
                    .bitcast()
            }

            fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
                val.bitcast::<$cvt_ty>()
                    .cvt::<i32>()
                    .shfl_up(mask, diff, width)
                    .cvt::<$cvt_ty>()
                    .bitcast()
            }

            fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self> {
                val.bitcast::<$cvt_ty>()
                    .cvt::<i32>()
                    .shfl_idx(mask, idx, width)
                    .cvt::<$cvt_ty>()
                    .bitcast()
            }
        }
    )*
    };
}

shfl_impl_sub32b!(u16: BF16, F16, u16, i16);
shfl_impl_sub32b!(u8: F8E4M3, F8E5M2);

impl<T, const N: usize> Shflable for V<T, N>
where
    T: Shflable + VecTy,
{
    fn shfl_bfly(mask: Val<u32>, val: Val<Self>, xor: Val<i32>, width: u32) -> Val<Self> {
        let elem_size = T::size();
        let mut ret = V::undef_val(mask.fn_ref().clone());
        match elem_size {
            1 => {
                let (v4, rest) = val.windows::<4>();
                let mut offset = 0;
                for subvec in v4 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_bfly(mask.copy(), xor.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<4>(to_pack, offset);
                    offset += 4;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_bfly(mask.copy(), xor.copy(), width), offset);
                    offset += 1;
                }
            }
            2 => {
                let (v2, rest) = val.windows::<2>();
                let mut offset = 0;
                for subvec in v2 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_bfly(mask.copy(), xor.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<2>(to_pack, offset);
                    offset += 2;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_bfly(mask.copy(), xor.copy(), width), offset);
                    offset += 1;
                }
            }
            _ => {
                assert!(elem_size.is_multiple_of(i32::size()));
                for (i, elem) in val.elements().into_iter().enumerate() {
                    ret = ret.insert_element(elem.shfl_bfly(mask.copy(), xor.copy(), width), i);
                }
            }
        }
        ret
    }

    fn shfl_down(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        let elem_size = T::size();
        let mut ret = V::undef_val(mask.fn_ref().clone());
        match elem_size {
            1 => {
                let (v4, rest) = val.windows::<4>();
                let mut offset = 0;
                for subvec in v4 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_down(mask.copy(), diff.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<4>(to_pack, offset);
                    offset += 4;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_down(mask.copy(), diff.copy(), width), offset);
                    offset += 1;
                }
            }
            2 => {
                let (v2, rest) = val.windows::<2>();
                let mut offset = 0;
                for subvec in v2 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_down(mask.copy(), diff.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<2>(to_pack, offset);
                    offset += 2;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_down(mask.copy(), diff.copy(), width), offset);
                    offset += 1;
                }
            }
            _ => {
                assert!(elem_size.is_multiple_of(i32::size()));
                for (i, elem) in val.elements().into_iter().enumerate() {
                    ret = ret.insert_element(elem.shfl_bfly(mask.copy(), diff.copy(), width), i);
                }
            }
        }
        ret
    }

    fn shfl_up(mask: Val<u32>, val: Val<Self>, diff: Val<i32>, width: u32) -> Val<Self> {
        let elem_size = T::size();
        let mut ret = V::undef_val(mask.fn_ref().clone());
        match elem_size {
            1 => {
                let (v4, rest) = val.windows::<4>();
                let mut offset = 0;
                for subvec in v4 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_up(mask.copy(), diff.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<4>(to_pack, offset);
                    offset += 4;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_up(mask.copy(), diff.copy(), width), offset);
                    offset += 1;
                }
            }
            2 => {
                let (v2, rest) = val.windows::<2>();
                let mut offset = 0;
                for subvec in v2 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_up(mask.copy(), diff.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<2>(to_pack, offset);
                    offset += 2;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_up(mask.copy(), diff.copy(), width), offset);
                    offset += 1;
                }
            }
            _ => {
                assert!(elem_size.is_multiple_of(i32::size()));
                for (i, elem) in val.elements().into_iter().enumerate() {
                    ret = ret.insert_element(elem.shfl_up(mask.copy(), diff.copy(), width), i);
                }
            }
        }
        ret
    }

    fn shfl_idx(mask: Val<u32>, val: Val<Self>, idx: Val<i32>, width: u32) -> Val<Self> {
        let elem_size = T::size();
        let mut ret = V::undef_val(mask.fn_ref().clone());
        match elem_size {
            1 => {
                let (v4, rest) = val.windows::<4>();
                let mut offset = 0;
                for subvec in v4 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_idx(mask.copy(), idx.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<4>(to_pack, offset);
                    offset += 4;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_idx(mask.copy(), idx.copy(), width), offset);
                    offset += 1;
                }
            }
            2 => {
                let (v2, rest) = val.windows::<2>();
                let mut offset = 0;
                for subvec in v2 {
                    let to_pack = subvec
                        .bitcast::<i32>()
                        .shfl_idx(mask.copy(), idx.copy(), width)
                        .bitcast();
                    ret = ret.insert_vec::<2>(to_pack, offset);
                    offset += 2;
                }
                for element in rest {
                    ret = ret
                        .insert_element(element.shfl_idx(mask.copy(), idx.copy(), width), offset);
                    offset += 1;
                }
            }
            _ => {
                assert!(elem_size.is_multiple_of(i32::size()));
                for (i, elem) in val.elements().into_iter().enumerate() {
                    ret = ret.insert_element(elem.shfl_idx(mask.copy(), idx.copy(), width), i);
                }
            }
        }
        ret
    }
}

impl<Shfl: Shflable> Val<Shfl> {
    pub fn shfl_bfly(self, mask: Val<u32>, xor: Val<i32>, width: u32) -> Val<Shfl> {
        assert!(width <= 32);
        assert!(width & (width - 1) == 0, "width must be a power of 2");
        Shfl::shfl_bfly(mask, self, xor, width)
    }
    pub fn shfl_down(self, mask: Val<u32>, diff: Val<i32>, width: u32) -> Val<Shfl> {
        assert!(width <= 32);
        assert!(width & (width - 1) == 0, "width must be a power of 2");
        Shfl::shfl_down(mask, self, diff, width)
    }
    pub fn shfl_up(self, mask: Val<u32>, diff: Val<i32>, width: u32) -> Val<Shfl> {
        assert!(width <= 32);
        assert!(width & (width - 1) == 0, "width must be a power of 2");
        Shfl::shfl_up(mask, self, diff, width)
    }
    pub fn shfl_idx(self, mask: Val<u32>, idx: Val<i32>, width: u32) -> Val<Shfl> {
        assert!(width <= 32);
        assert!(width & (width - 1) == 0, "width must be a power of 2");
        Shfl::shfl_up(mask, self, idx, width)
    }

    pub fn shfl_bfly_uniform(self, xor: Val<i32>) -> Val<Shfl> {
        let mask = xor.fn_ref().constant(0xFFFFFFFFu32);
        self.shfl_bfly(mask, xor, 32)
    }
    pub fn shfl_down_uniform(self, diff: Val<i32>) -> Val<Shfl> {
        let mask = diff.fn_ref().constant(0xFFFFFFFFu32);
        self.shfl_down(mask, diff, 32)
    }
    pub fn shfl_up_uniform(self, diff: Val<i32>) -> Val<Shfl> {
        let mask = diff.fn_ref().constant(0xFFFFFFFFu32);
        self.shfl_up(mask, diff, 32)
    }
    pub fn shfl_idx_uniform(self, idx: Val<i32>) -> Val<Shfl> {
        let mask = idx.fn_ref().constant(0xFFFFFFFFu32);
        self.shfl_idx(mask, idx, 32)
    }
}
