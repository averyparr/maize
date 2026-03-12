use maize_core::ty::{BF16, ContiguousUniformTy, F32, V};

use crate::mma::{SyncMMAOp, sm80::Sm80MmaBf16F32_16x8x16};

pub struct Sm80MmaBf16F32_16x16x16;

impl SyncMMAOp for Sm80MmaBf16F32_16x16x16 {
    type FragA = V<BF16, 8>;
    type FragB = V<BF16, 8>;
    type FragC = V<F32, 8>;

    fn call<'a>(
        a: maize_core::val::Val<'a, Self::FragA>,
        b: maize_core::val::Val<'a, Self::FragB>,
        c: maize_core::val::Val<'a, Self::FragC>,
    ) -> maize_core::val::Val<'a, Self::FragC> {
        let b_chunks = b.chunks_exact::<4>();
        let b0 = b_chunks[0];
        let b1 = b_chunks[1];
        let c_chunks = c.chunks_exact::<4>();
        let c0 = c_chunks[0];
        let c1 = c_chunks[1];
        let [d0, d1, d2, d3] = Sm80MmaBf16F32_16x8x16::call(a, b0, c0).elements();
        let [d4, d5, d6, d7] = Sm80MmaBf16F32_16x8x16::call(a, b1, c1).elements();
        V::try_from_elements([d0, d1, d2, d3, d4, d5, d6, d7].into_iter())
            .expect("Size should match")
    }
}
