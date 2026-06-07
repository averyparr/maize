use crate::{
    backend::VoidType,
    intrinsics::{Intrinsic, cuda::CUDA, impl_argless_intrinsics, impl_intrinsics},
    tipe::{A, V},
    val::Val,
};

impl_argless_intrinsics!(
    TcGen05WaitLd: "llvm.nvvm.tcgen05.wait.ld"() -> VoidType,
    TcGen05WaitSt: "llvm.nvvm.tcgen05.wait.st"() -> VoidType,
);

impl CUDA {
    pub fn tcgen05_wait_ld(&self) {
        let t = TcGen05WaitLd(self.0.clone()).call(());
    }
    pub fn tcgen05_wait_st(&self) {
        let t = TcGen05WaitSt(self.0.clone()).call(());
    }
}

impl_intrinsics!(
    TcGen05St16x64bx1:"llvm.nvvm.tcgen05.st.16x64b.x1"(A<*mut u8, 6>, i32, bool) -> VoidType,
    TcGen05St16x64bx2:"llvm.nvvm.tcgen05.st.16x64b.x2"(A<*mut u8, 6>, V<i32, 2>, bool) -> VoidType,
    TcGen05St16x64bx4:"llvm.nvvm.tcgen05.st.16x64b.x4"(A<*mut u8, 6>, V<i32, 4>, bool) -> VoidType,
    TcGen05St16x64bx8:"llvm.nvvm.tcgen05.st.16x64b.x8"(A<*mut u8, 6>, V<i32, 8>, bool) -> VoidType,
    TcGen05St16x64bx16:"llvm.nvvm.tcgen05.st.16x64b.x16"(A<*mut u8, 6>, V<i32, 16>, bool) -> VoidType,
    TcGen05St16x64bx32:"llvm.nvvm.tcgen05.st.16x64b.x32"(A<*mut u8, 6>, V<i32, 32>, bool) -> VoidType,
    TcGen05St16x64bx64:"llvm.nvvm.tcgen05.st.16x64b.x64"(A<*mut u8, 6>, V<i32, 64>, bool) -> VoidType,
    TcGen05St16x64bx128:"llvm.nvvm.tcgen05.st.16x64b.x128"(A<*mut u8, 6>, V<i32, 128>, bool) -> VoidType,

    TcGen05St16x128bx1:"llvm.nvvm.tcgen05.st.16x128b.x1"(A<*mut u8, 6>, V<i32, 2>, bool) -> VoidType,
    TcGen05St16x128bx2:"llvm.nvvm.tcgen05.st.16x128b.x2"(A<*mut u8, 6>, V<i32, 4>, bool) -> VoidType,
    TcGen05St16x128bx4:"llvm.nvvm.tcgen05.st.16x128b.x4"(A<*mut u8, 6>, V<i32, 8>, bool) -> VoidType,
    TcGen05St16x128bx8:"llvm.nvvm.tcgen05.st.16x128b.x8"(A<*mut u8, 6>, V<i32, 16>, bool) -> VoidType,
    TcGen05St16x128bx16:"llvm.nvvm.tcgen05.st.16x128b.x16"(A<*mut u8, 6>, V<i32, 32>, bool) -> VoidType,
    TcGen05St16x128bx32:"llvm.nvvm.tcgen05.st.16x128b.x32"(A<*mut u8, 6>, V<i32, 64>, bool) -> VoidType,
    TcGen05St16x128bx64:"llvm.nvvm.tcgen05.st.16x128b.x64"(A<*mut u8, 6>, V<i32, 128>, bool) -> VoidType,
    // Doesn't exist!  Just as well, 256 regs per call is insane
    // TcGen05St16x128bx128:"llvm.nvvm.tcgen05.st.16x128b.x128"(A<*mut u8, 6>, V<i32, 256>, bool) -> VoidType,

    TcGen05St16x256bx1:"llvm.nvvm.tcgen05.st.16x256b.x1"(A<*mut u8, 6>, V<i32, 4>, bool) -> VoidType,
    TcGen05St16x256bx2:"llvm.nvvm.tcgen05.st.16x256b.x2"(A<*mut u8, 6>, V<i32, 8>, bool) -> VoidType,
    TcGen05St16x256bx4:"llvm.nvvm.tcgen05.st.16x256b.x4"(A<*mut u8, 6>, V<i32, 16>, bool) -> VoidType,
    TcGen05St16x256bx8:"llvm.nvvm.tcgen05.st.16x256b.x8"(A<*mut u8, 6>, V<i32, 32>, bool) -> VoidType,
    TcGen05St16x256bx16:"llvm.nvvm.tcgen05.st.16x256b.x16"(A<*mut u8, 6>, V<i32, 64>, bool) -> VoidType,
    TcGen05St16x256bx32:"llvm.nvvm.tcgen05.st.16x256b.x32"(A<*mut u8, 6>, V<i32, 128>, bool) -> VoidType,
    // Doesn't exist!  Too many regs per call
    // TcGen05St16x256bx64:"llvm.nvvm.tcgen05.st.16x256b.x64"(A<*mut u8, 6>, V<i32, 256>, bool) -> VoidType,
    // TcGen05St16x256bx128:"llvm.nvvm.tcgen05.st.16x256b.x128"(A<*mut u8, 6>, V<i32, 512>, bool) -> VoidType,

    TcGen05St32x32bx1:"llvm.nvvm.tcgen05.st.32x32b.x1"(A<*mut u8, 6>, i32, bool) -> VoidType,
    TcGen05St32x32bx2:"llvm.nvvm.tcgen05.st.32x32b.x2"(A<*mut u8, 6>, V<i32, 2>, bool) -> VoidType,
    TcGen05St32x32bx4:"llvm.nvvm.tcgen05.st.32x32b.x4"(A<*mut u8, 6>, V<i32, 4>, bool) -> VoidType,
    TcGen05St32x32bx8:"llvm.nvvm.tcgen05.st.32x32b.x8"(A<*mut u8, 6>, V<i32, 8>, bool) -> VoidType,
    TcGen05St32x32bx16:"llvm.nvvm.tcgen05.st.32x32b.x16"(A<*mut u8, 6>, V<i32, 16>, bool) -> VoidType,
    TcGen05St32x32bx32:"llvm.nvvm.tcgen05.st.32x32b.x32"(A<*mut u8, 6>, V<i32, 32>, bool) -> VoidType,
    TcGen05St32x32bx64:"llvm.nvvm.tcgen05.st.32x32b.x64"(A<*mut u8, 6>, V<i32, 64>, bool) -> VoidType,
    TcGen05St32x32bx128:"llvm.nvvm.tcgen05.st.32x32b.x128"(A<*mut u8, 6>, V<i32, 128>, bool) -> VoidType,

    TcGen05St16x32bx2x1:"llvm.nvvm.tcgen05.st.16x32bx2.x1"(A<*mut u8, 6>, i64, i32, bool) -> VoidType,
    TcGen05St16x32bx2x2:"llvm.nvvm.tcgen05.st.16x32bx2.x2"(A<*mut u8, 6>, i64, V<i32, 2>, bool) -> VoidType,
    TcGen05St16x32bx2x4:"llvm.nvvm.tcgen05.st.16x32bx2.x4"(A<*mut u8, 6>, i64, V<i32, 4>, bool) -> VoidType,
    TcGen05St16x32bx2x8:"llvm.nvvm.tcgen05.st.16x32bx2.x8"(A<*mut u8, 6>, i64, V<i32, 8>, bool) -> VoidType,
    TcGen05St16x32bx2x16:"llvm.nvvm.tcgen05.st.16x32bx2.x16"(A<*mut u8, 6>, i64, V<i32, 16>, bool) -> VoidType,
    TcGen05St16x32bx2x32:"llvm.nvvm.tcgen05.st.16x32bx2.x32"(A<*mut u8, 6>, i64, V<i32, 32>, bool) -> VoidType,
    TcGen05St16x32bx2x64:"llvm.nvvm.tcgen05.st.16x32bx2.x64"(A<*mut u8, 6>, i64, V<i32, 64>, bool) -> VoidType,
    TcGen05St16x32bx2x128:"llvm.nvvm.tcgen05.st.16x32bx2.x128"(A<*mut u8, 6>, i64, V<i32, 128>, bool) -> VoidType,

    TcGen05Ld16x64bx1:"llvm.nvvm.tcgen05.ld.16x64b.x1"(A<*mut u8, 6>, bool) -> i32,
    TcGen05Ld16x64bx2:"llvm.nvvm.tcgen05.ld.16x64b.x2"(A<*mut u8, 6>, bool) -> V<i32, 2>,
    TcGen05Ld16x64bx4:"llvm.nvvm.tcgen05.ld.16x64b.x4"(A<*mut u8, 6>, bool) -> V<i32, 4>,
    TcGen05Ld16x64bx8:"llvm.nvvm.tcgen05.ld.16x64b.x8"(A<*mut u8, 6>, bool) -> V<i32, 8>,
    TcGen05Ld16x64bx16:"llvm.nvvm.tcgen05.ld.16x64b.x16"(A<*mut u8, 6>, bool) -> V<i32, 16>,
    TcGen05Ld16x64bx32:"llvm.nvvm.tcgen05.ld.16x64b.x32"(A<*mut u8, 6>, bool) -> V<i32, 32>,
    TcGen05Ld16x64bx64:"llvm.nvvm.tcgen05.ld.16x64b.x64"(A<*mut u8, 6>, bool) -> V<i32, 64>,
    TcGen05Ld16x64bx128:"llvm.nvvm.tcgen05.ld.16x64b.x128"(A<*mut u8, 6>, bool) -> V<i32, 128>,

    TcGen05Ld16x128bx1:"llvm.nvvm.tcgen05.ld.16x128b.x1"(A<*mut u8, 6>, bool) -> V<i32, 2>,
    TcGen05Ld16x128bx2:"llvm.nvvm.tcgen05.ld.16x128b.x2"(A<*mut u8, 6>, bool) -> V<i32, 4>,
    TcGen05Ld16x128bx4:"llvm.nvvm.tcgen05.ld.16x128b.x4"(A<*mut u8, 6>, bool) -> V<i32, 8>,
    TcGen05Ld16x128bx8:"llvm.nvvm.tcgen05.ld.16x128b.x8"(A<*mut u8, 6>, bool) -> V<i32, 16>,
    TcGen05Ld16x128bx16:"llvm.nvvm.tcgen05.ld.16x128b.x16"(A<*mut u8, 6>, bool) -> V<i32, 32>,
    TcGen05Ld16x128bx32:"llvm.nvvm.tcgen05.ld.16x128b.x32"(A<*mut u8, 6>, bool) -> V<i32, 64>,
    TcGen05Ld16x128bx64:"llvm.nvvm.tcgen05.ld.16x128b.x64"(A<*mut u8, 6>, bool) -> V<i32, 128>,
    // Doesn't exist!  Just as well, 256 regs per call is insane
    // TcGen05Ld16x128bx128:"llvm.nvvm.tcgen05.ld.16x128b.x128"(A<*mut u8, 6>, bool) -> V<i32, 256>,

    TcGen05Ld16x256bx1:"llvm.nvvm.tcgen05.ld.16x256b.x1"(A<*mut u8, 6>, bool) -> V<i32, 4>,
    TcGen05Ld16x256bx2:"llvm.nvvm.tcgen05.ld.16x256b.x2"(A<*mut u8, 6>, bool) -> V<i32, 8>,
    TcGen05Ld16x256bx4:"llvm.nvvm.tcgen05.ld.16x256b.x4"(A<*mut u8, 6>, bool) -> V<i32, 16>,
    TcGen05Ld16x256bx8:"llvm.nvvm.tcgen05.ld.16x256b.x8"(A<*mut u8, 6>, bool) -> V<i32, 32>,
    TcGen05Ld16x256bx16:"llvm.nvvm.tcgen05.ld.16x256b.x16"(A<*mut u8, 6>, bool) -> V<i32, 64>,
    TcGen05Ld16x256bx32:"llvm.nvvm.tcgen05.ld.16x256b.x32"(A<*mut u8, 6>, bool) -> V<i32, 128>,
    // Doesn't exist!  Too many regs per call
    // TcGen05Ld16x256bx64:"llvm.nvvm.tcgen05.ld.16x256b.x64"(A<*mut u8, 6>, bool) -> V<i32, 256>,
    // TcGen05Ld16x256bx128:"llvm.nvvm.tcgen05.ld.16x256b.x128"(A<*mut u8, 6>, bool) -> V<i32, 512>,

    TcGen05Ld32x32bx1:"llvm.nvvm.tcgen05.ld.32x32b.x1"(A<*mut u8, 6>, bool) -> i32,
    TcGen05Ld32x32bx2:"llvm.nvvm.tcgen05.ld.32x32b.x2"(A<*mut u8, 6>, bool) -> V<i32, 2>,
    TcGen05Ld32x32bx4:"llvm.nvvm.tcgen05.ld.32x32b.x4"(A<*mut u8, 6>, bool) -> V<i32, 4>,
    TcGen05Ld32x32bx8:"llvm.nvvm.tcgen05.ld.32x32b.x8"(A<*mut u8, 6>, bool) -> V<i32, 8>,
    TcGen05Ld32x32bx16:"llvm.nvvm.tcgen05.ld.32x32b.x16"(A<*mut u8, 6>, bool) -> V<i32, 16>,
    TcGen05Ld32x32bx32:"llvm.nvvm.tcgen05.ld.32x32b.x32"(A<*mut u8, 6>, bool) -> V<i32, 32>,
    TcGen05Ld32x32bx64:"llvm.nvvm.tcgen05.ld.32x32b.x64"(A<*mut u8, 6>, bool) -> V<i32, 64>,
    TcGen05Ld32x32bx128:"llvm.nvvm.tcgen05.ld.32x32b.x128"(A<*mut u8, 6>, bool) -> V<i32, 128>,

    TcGen05Ld16x32bx2x1:"llvm.nvvm.tcgen05.ld.16x32bx2.x1"(A<*mut u8, 6>, i64,  bool) -> i32,
    TcGen05Ld16x32bx2x2:"llvm.nvvm.tcgen05.ld.16x32bx2.x2"(A<*mut u8, 6>, i64, bool) -> V<i32, 2>,
    TcGen05Ld16x32bx2x4:"llvm.nvvm.tcgen05.ld.16x32bx2.x4"(A<*mut u8, 6>, i64, bool) -> V<i32, 4>,
    TcGen05Ld16x32bx2x8:"llvm.nvvm.tcgen05.ld.16x32bx2.x8"(A<*mut u8, 6>, i64, bool) -> V<i32, 8>,
    TcGen05Ld16x32bx2x16:"llvm.nvvm.tcgen05.ld.16x32bx2.x16"(A<*mut u8, 6>, i64, bool) -> V<i32, 16>,
    TcGen05Ld16x32bx2x32:"llvm.nvvm.tcgen05.ld.16x32bx2.x32"(A<*mut u8, 6>, i64, bool) -> V<i32, 32>,
    TcGen05Ld16x32bx2x64:"llvm.nvvm.tcgen05.ld.16x32bx2.x64"(A<*mut u8, 6>, i64, bool) -> V<i32, 64>,
    TcGen05Ld16x32bx2x128:"llvm.nvvm.tcgen05.ld.16x32bx2.x128"(A<*mut u8, 6>, i64, bool) -> V<i32, 128>,
);

macro_rules! test_tcgen05_st {
    ($name: ident, $valty: ty, $contains: literal $(,$extra: ident)?) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            #[test]
            fn test_ptx_gen() {
                let func = $crate::func::implement_ptx_kernel::<(A<*mut u8, 6>, $valty)>(
                    $crate::backend::llvm::LLVM::new(),
                    concat!("test_", stringify!($name)),
                );
                {
                    let (tmem_ptr, val) = func.args();
                    $(let $extra = func.constant(64);)?
                    $name(tmem_ptr.copy(), val.copy(), $($extra.copy(),)? func.constant(false));
                    $name(tmem_ptr.copy(), val.copy(), $($extra.copy(),)? func.constant(true));
                }
                func.return_void();
                let bytes = func.compile($crate::SM::SM100a, $crate::Opt::O3);
                let string = str::from_utf8(&bytes).expect("Should be valid");
                assert!(string.contains($contains), "{string}");
            }
        }
    };
}

#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! test_tcgen05_st {
    ($($t: tt)*) => {};
}

macro_rules! gen_tcgen05_st {
    (vectorize: $name: ident => $intrinsic: ident: $valty: ty, $contains: literal $(,$extra: ident)?) => {
        pub fn $name(tmem_ptr: Val<A<*mut u8, 6>>, vals: Val<$valty>, $($extra: Val<i64>,)? unpack: Val<bool>) {
            $intrinsic.call((tmem_ptr, $($extra,)? vals.elements()[0].copy(), unpack))
        }
        test_tcgen05_st!($name, $valty, $contains $(,$extra)?);
    };
    ($name: ident => $intrinsic: ident: $valty: ty, $contains: literal $(,$extra: ident)?) => {
        pub fn $name(tmem_ptr: Val<A<*mut u8, 6>>, vals: Val<$valty>, $($extra: Val<i64>,)? unpack: Val<bool>) {
            $intrinsic.call((tmem_ptr, $($extra,)? vals, unpack))
        }
        test_tcgen05_st!($name, $valty, $contains $(,$extra)?);
    };
}

macro_rules! test_tcgen05_ld {
    ($name: ident, $valty: ty, $contains: literal $(, $extra: ident)?) => {
        #[cfg(test)]
        mod $name {
            use super::*;
            #[test]
            fn test_ptx_gen() {
                let func = $crate::func::implement_ptx_kernel::<(A<*mut u8, 6>, &mut $valty, &mut $valty)>(
                    $crate::backend::llvm::LLVM::new(),
                    concat!("test_", stringify!($name)),
                );
                {
                    let (tmem_ptr, val_mut1, val_mut2) = func.args();
                    $(let $extra = func.constant(64);)?
                    let with = $name(tmem_ptr.copy(), $($extra.copy(),)? func.constant(false));
                    let without = $name(tmem_ptr.copy(), $($extra.copy(),)? func.constant(true));
                    val_mut1.store(with);
                    val_mut2.store(without);
                }
                func.return_void();
                let bytes = func.compile($crate::SM::SM100a, $crate::Opt::O3);
                let string = str::from_utf8(&bytes).expect("Should be valid");
                assert!(string.contains($contains), "{string}");
            }
        }
    };
}

#[cfg(not(feature = "ptx-gen-tests"))]
macro_rules! test_tcgen05_ld {
    ($($t: tt)*) => {};
}

macro_rules! gen_tcgen05_ld {
    (test: $name: ident, $valty: ty, $contains: literal $(,$extra: ident)?) => {
    };
    (vectorize: $name: ident => $intrinsic: ident: $valty: ty, $contains: literal $(,$extra: ident)?) => {
        pub fn $name(tmem_ptr: Val<A<*mut u8, 6>>, $($extra: Val<i64>,)? unpack: Val<bool>) -> Val<$valty> {
            Val::from_elements([$intrinsic.call((tmem_ptr, $($extra,)? unpack))])
        }
        test_tcgen05_ld!($name, $valty, $contains $(, $extra)?);
    };
    ($name: ident => $intrinsic: ident: $valty: ty, $contains: literal $(,$extra: ident)?) => {
        pub fn $name(tmem_ptr: Val<A<*mut u8, 6>>, $($extra: Val<i64>,)? unpack: Val<bool>) -> Val<$valty> {
            $intrinsic.call((tmem_ptr, $($extra,)? unpack))
        }
        test_tcgen05_ld!($name, $valty, $contains $(, $extra)?);
    };
}

gen_tcgen05_st!(vectorize: tcgen05_st_16x64b_x1 => TcGen05St16x64bx1: V<i32,1>, "tcgen05.st.sync.aligned.16x64b.x1.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x2 => TcGen05St16x64bx2: V<i32, 2>, "tcgen05.st.sync.aligned.16x64b.x2.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x4 => TcGen05St16x64bx4: V<i32, 4>, "tcgen05.st.sync.aligned.16x64b.x4.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x8 => TcGen05St16x64bx8: V<i32, 8>, "tcgen05.st.sync.aligned.16x64b.x8.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x16 => TcGen05St16x64bx16: V<i32, 16>, "tcgen05.st.sync.aligned.16x64b.x16.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x32 => TcGen05St16x64bx32: V<i32, 32>, "tcgen05.st.sync.aligned.16x64b.x32.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x64 => TcGen05St16x64bx64: V<i32, 64>, "tcgen05.st.sync.aligned.16x64b.x64.b32");
gen_tcgen05_st!(tcgen05_st_16x64b_x128 => TcGen05St16x64bx128: V<i32, 128>, "tcgen05.st.sync.aligned.16x64b.x128.b32");

gen_tcgen05_st!(tcgen05_st_16x128b_x1 => TcGen05St16x128bx1: V<i32, 2>, "tcgen05.st.sync.aligned.16x128b.x1.b32");
gen_tcgen05_st!(tcgen05_st_16x128b_x2 => TcGen05St16x128bx2: V<i32, 4>, "tcgen05.st.sync.aligned.16x128b.x2.b32");
gen_tcgen05_st!(tcgen05_st_16x128b_x4 => TcGen05St16x128bx4: V<i32, 8>, "tcgen05.st.sync.aligned.16x128b.x4.b32");
gen_tcgen05_st!(tcgen05_st_16x128b_x8 => TcGen05St16x128bx8: V<i32, 16>, "tcgen05.st.sync.aligned.16x128b.x8.b32");
gen_tcgen05_st!(tcgen05_st_16x128b_x16 => TcGen05St16x128bx16: V<i32, 32>, "tcgen05.st.sync.aligned.16x128b.x16.b32");
gen_tcgen05_st!(tcgen05_st_16x128b_x32 => TcGen05St16x128bx32: V<i32, 64>, "tcgen05.st.sync.aligned.16x128b.x32.b32");
gen_tcgen05_st!(tcgen05_st_16x128b_x64 => TcGen05St16x128bx64: V<i32, 128>, "tcgen05.st.sync.aligned.16x128b.x64.b32");
// gen_tcgen05_st!(tcgen05_st_16x128b_x128 => TcGen05St16x128bx128: V<i32, 256>, "tcgen05.st.sync.aligned.16x128b.x128.b32");

gen_tcgen05_st!(tcgen05_st_16x256b_x1 => TcGen05St16x256bx1: V<i32, 4>, "tcgen05.st.sync.aligned.16x256b.x1.b32");
gen_tcgen05_st!(tcgen05_st_16x256b_x2 => TcGen05St16x256bx2: V<i32, 8>, "tcgen05.st.sync.aligned.16x256b.x2.b32");
gen_tcgen05_st!(tcgen05_st_16x256b_x4 => TcGen05St16x256bx4: V<i32, 16>, "tcgen05.st.sync.aligned.16x256b.x4.b32");
gen_tcgen05_st!(tcgen05_st_16x256b_x8 => TcGen05St16x256bx8: V<i32, 32>, "tcgen05.st.sync.aligned.16x256b.x8.b32");
gen_tcgen05_st!(tcgen05_st_16x256b_x16 => TcGen05St16x256bx16: V<i32, 64>, "tcgen05.st.sync.aligned.16x256b.x16.b32");
gen_tcgen05_st!(tcgen05_st_16x256b_x32 => TcGen05St16x256bx32: V<i32, 128>, "tcgen05.st.sync.aligned.16x256b.x32.b32");
// gen_tcgen05_st!(tcgen05_st_16x256b_x64 => TcGen05St16x256bx64: V<i32, 256>, "tcgen05.st.sync.aligned.16x256b.x64.b32");
// gen_tcgen05_st!(tcgen05_st_16x256b_x128 => TcGen05St16x256bx128: V<i32, 512>, "tcgen05.st.sync.aligned.16x256b.x128.b32");

gen_tcgen05_st!(vectorize: tcgen05_st_32x32b_x1 => TcGen05St32x32bx1: V<i32,1>, "tcgen05.st.sync.aligned.32x32b.x1.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x2 => TcGen05St32x32bx2: V<i32, 2>, "tcgen05.st.sync.aligned.32x32b.x2.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x4 => TcGen05St32x32bx4: V<i32, 4>, "tcgen05.st.sync.aligned.32x32b.x4.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x8 => TcGen05St32x32bx8: V<i32, 8>, "tcgen05.st.sync.aligned.32x32b.x8.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x16 => TcGen05St32x32bx16: V<i32, 16>, "tcgen05.st.sync.aligned.32x32b.x16.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x32 => TcGen05St32x32bx32: V<i32, 32>, "tcgen05.st.sync.aligned.32x32b.x32.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x64 => TcGen05St32x32bx64: V<i32, 64>, "tcgen05.st.sync.aligned.32x32b.x64.b32");
gen_tcgen05_st!(tcgen05_st_32x32b_x128 => TcGen05St32x32bx128: V<i32, 128>, "tcgen05.st.sync.aligned.32x32b.x128.b32");

gen_tcgen05_st!(vectorize: tcgen05_st_16x32bx2_x1 => TcGen05St16x32bx2x1: V<i32,1>, "tcgen05.st.sync.aligned.16x32bx2.x1.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x2 => TcGen05St16x32bx2x2: V<i32, 2>, "tcgen05.st.sync.aligned.16x32bx2.x2.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x4 => TcGen05St16x32bx2x4: V<i32, 4>, "tcgen05.st.sync.aligned.16x32bx2.x4.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x8 => TcGen05St16x32bx2x8: V<i32, 8>, "tcgen05.st.sync.aligned.16x32bx2.x8.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x16 => TcGen05St16x32bx2x16: V<i32, 16>, "tcgen05.st.sync.aligned.16x32bx2.x16.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x32 => TcGen05St16x32bx2x32: V<i32, 32>, "tcgen05.st.sync.aligned.16x32bx2.x32.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x64 => TcGen05St16x32bx2x64: V<i32, 64>, "tcgen05.st.sync.aligned.16x32bx2.x64.b32", offset);
gen_tcgen05_st!(tcgen05_st_16x32bx2_x128 => TcGen05St16x32bx2x128: V<i32, 128>, "tcgen05.st.sync.aligned.16x32bx2.x128.b32", offset);

gen_tcgen05_ld!(vectorize: tcgen05_ld_16x64b_x1 => TcGen05Ld16x64bx1: V<i32,1>, "tcgen05.ld.sync.aligned.16x64b.x1.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x2 => TcGen05Ld16x64bx2: V<i32, 2>, "tcgen05.ld.sync.aligned.16x64b.x2.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x4 => TcGen05Ld16x64bx4: V<i32, 4>, "tcgen05.ld.sync.aligned.16x64b.x4.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x8 => TcGen05Ld16x64bx8: V<i32, 8>, "tcgen05.ld.sync.aligned.16x64b.x8.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x16 => TcGen05Ld16x64bx16: V<i32, 16>, "tcgen05.ld.sync.aligned.16x64b.x16.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x32 => TcGen05Ld16x64bx32: V<i32, 32>, "tcgen05.ld.sync.aligned.16x64b.x32.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x64 => TcGen05Ld16x64bx64: V<i32, 64>, "tcgen05.ld.sync.aligned.16x64b.x64.b32");
gen_tcgen05_ld!(tcgen05_ld_16x64b_x128 => TcGen05Ld16x64bx128: V<i32, 128>, "tcgen05.ld.sync.aligned.16x64b.x128.b32");

gen_tcgen05_ld!(tcgen05_ld_16x128b_x1 => TcGen05Ld16x128bx1: V<i32, 2>, "tcgen05.ld.sync.aligned.16x128b.x1.b32");
gen_tcgen05_ld!(tcgen05_ld_16x128b_x2 => TcGen05Ld16x128bx2: V<i32, 4>, "tcgen05.ld.sync.aligned.16x128b.x2.b32");
gen_tcgen05_ld!(tcgen05_ld_16x128b_x4 => TcGen05Ld16x128bx4: V<i32, 8>, "tcgen05.ld.sync.aligned.16x128b.x4.b32");
gen_tcgen05_ld!(tcgen05_ld_16x128b_x8 => TcGen05Ld16x128bx8: V<i32, 16>, "tcgen05.ld.sync.aligned.16x128b.x8.b32");
gen_tcgen05_ld!(tcgen05_ld_16x128b_x16 => TcGen05Ld16x128bx16: V<i32, 32>, "tcgen05.ld.sync.aligned.16x128b.x16.b32");
gen_tcgen05_ld!(tcgen05_ld_16x128b_x32 => TcGen05Ld16x128bx32: V<i32, 64>, "tcgen05.ld.sync.aligned.16x128b.x32.b32");
gen_tcgen05_ld!(tcgen05_ld_16x128b_x64 => TcGen05Ld16x128bx64: V<i32, 128>, "tcgen05.ld.sync.aligned.16x128b.x64.b32");
// gen_tcgen05_ld!(tcgen05_ld_16x128b_x128 => TcGen05Ld16x128bx128: V<i32, 256>, "tcgen05.ld.sync.aligned.16x128b.x128.b32");

gen_tcgen05_ld!(tcgen05_ld_16x256b_x1 => TcGen05Ld16x256bx1: V<i32, 4>, "tcgen05.ld.sync.aligned.16x256b.x1.b32");
gen_tcgen05_ld!(tcgen05_ld_16x256b_x2 => TcGen05Ld16x256bx2: V<i32, 8>, "tcgen05.ld.sync.aligned.16x256b.x2.b32");
gen_tcgen05_ld!(tcgen05_ld_16x256b_x4 => TcGen05Ld16x256bx4: V<i32, 16>, "tcgen05.ld.sync.aligned.16x256b.x4.b32");
gen_tcgen05_ld!(tcgen05_ld_16x256b_x8 => TcGen05Ld16x256bx8: V<i32, 32>, "tcgen05.ld.sync.aligned.16x256b.x8.b32");
gen_tcgen05_ld!(tcgen05_ld_16x256b_x16 => TcGen05Ld16x256bx16: V<i32, 64>, "tcgen05.ld.sync.aligned.16x256b.x16.b32");
gen_tcgen05_ld!(tcgen05_ld_16x256b_x32 => TcGen05Ld16x256bx32: V<i32, 128>, "tcgen05.ld.sync.aligned.16x256b.x32.b32");
// gen_tcgen05_ld!(tcgen05_ld_16x256b_x64 => TcGen05Ld16x256bx64: V<i32, 256>, "tcgen05.ld.sync.aligned.16x256b.x64.b32");
// gen_tcgen05_ld!(tcgen05_ld_16x256b_x128 => TcGen05Ld16x256bx128: V<i32, 512>, "tcgen05.ld.sync.aligned.16x256b.x128.b32");

gen_tcgen05_ld!(vectorize: tcgen05_ld_32x32b_x1 => TcGen05Ld32x32bx1: V<i32,1>, "tcgen05.ld.sync.aligned.32x32b.x1.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x2 => TcGen05Ld32x32bx2: V<i32, 2>, "tcgen05.ld.sync.aligned.32x32b.x2.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x4 => TcGen05Ld32x32bx4: V<i32, 4>, "tcgen05.ld.sync.aligned.32x32b.x4.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x8 => TcGen05Ld32x32bx8: V<i32, 8>, "tcgen05.ld.sync.aligned.32x32b.x8.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x16 => TcGen05Ld32x32bx16: V<i32, 16>, "tcgen05.ld.sync.aligned.32x32b.x16.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x32 => TcGen05Ld32x32bx32: V<i32, 32>, "tcgen05.ld.sync.aligned.32x32b.x32.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x64 => TcGen05Ld32x32bx64: V<i32, 64>, "tcgen05.ld.sync.aligned.32x32b.x64.b32");
gen_tcgen05_ld!(tcgen05_ld_32x32b_x128 => TcGen05Ld32x32bx128: V<i32, 128>, "tcgen05.ld.sync.aligned.32x32b.x128.b32");

gen_tcgen05_ld!(vectorize: tcgen05_ld_16x32bx2_x1 => TcGen05Ld16x32bx2x1: V<i32, 1>, "tcgen05.ld.sync.aligned.16x32bx2.x1.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x2 => TcGen05Ld16x32bx2x2: V<i32, 2>, "tcgen05.ld.sync.aligned.16x32bx2.x2.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x4 => TcGen05Ld16x32bx2x4: V<i32, 4>, "tcgen05.ld.sync.aligned.16x32bx2.x4.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x8 => TcGen05Ld16x32bx2x8: V<i32, 8>, "tcgen05.ld.sync.aligned.16x32bx2.x8.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x16 => TcGen05Ld16x32bx2x16: V<i32, 16>, "tcgen05.ld.sync.aligned.16x32bx2.x16.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x32 => TcGen05Ld16x32bx2x32: V<i32, 32>, "tcgen05.ld.sync.aligned.16x32bx2.x32.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x64 => TcGen05Ld16x32bx2x64: V<i32, 64>, "tcgen05.ld.sync.aligned.16x32bx2.x64.b32", offset);
gen_tcgen05_ld!(tcgen05_ld_16x32bx2_x128 => TcGen05Ld16x32bx2x128: V<i32, 128>, "tcgen05.ld.sync.aligned.16x32bx2.x128.b32", offset);
