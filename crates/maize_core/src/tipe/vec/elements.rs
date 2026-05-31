use crate::{
    backend::UntypedValue,
    tipe::{Ty, V, VecTy},
    val::Val,
};

impl<T: VecTy> Val<T> {
    pub fn splat<const N: usize>(&self) -> Val<V<T, N>> {
        let b = unsafe { self.fn_ref().curr_bb_builder() };
        let raw_vec = T::splat(self.typed(), N as _, self.fn_ref().ctx(), b);
        unsafe { Val::new(self.fn_ref().clone(), UntypedValue(raw_vec.into())) }
    }
}

impl<T: VecTy, const N: usize> Val<V<T, N>> {
    pub fn elements(self) -> [Val<T>; N] {
        std::array::from_fn(|i| {
            let fn_ref = self.fn_ref();
            let val = self.typed();
            let b = unsafe { fn_ref.curr_bb_builder() };
            let index = fn_ref.ctx().i32_type().const_int(i as _, false);
            let elem = b
                .build_extract_element(val, index, "extractelem")
                .expect("Build extract element should work");
            unsafe { Val::new(fn_ref.clone(), UntypedValue(elem)) }
        })
    }

    pub fn from_elements(elements: [Val<T>; N]) -> Self {
        let fn_ref = elements[0].fn_ref().clone();
        let mut raw = V::<T, N>::raw_ty(fn_ref.ctx()).const_zero();
        let b = unsafe { fn_ref.curr_bb_builder() };

        for (i, elem) in elements.iter().enumerate() {
            let index: Val<u32> = fn_ref.constant(i.try_into().expect("usize -> u32 overflow"));
            raw = b
                .build_insert_element(raw, elem.typed(), index.typed(), "from_elements")
                .expect("Insert element should succeed");
        }

        unsafe { Self::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub fn extract_vec<const E: usize>(self, offset: usize) -> Val<V<T, E>> {
        let fn_ref = self.fn_ref().clone();
        let intrins = fn_ref
            .get_intrinsic::<V<T, E>, (V<T, N>, i64)>("llvm.vector.extract", true)
            .expect("Should have a llvm vector extract");
        let offset = fn_ref.constant(offset as _);
        fn_ref.call_extern(intrins, (self, offset), None)
    }

    pub fn insert_vec<const E: usize>(self, other: Val<V<T, E>>, offset: usize) -> Self {
        let fn_ref = self.fn_ref().clone();
        let intrins = fn_ref
            .get_intrinsic::<V<T, N>, (V<T, N>, V<T, E>, i64)>("llvm.vector.insert", false)
            .expect("Should have a llvm vector extract");
        let offset = fn_ref.constant(offset as _);
        fn_ref.call_extern(intrins, (self, other, offset), None)
    }

    pub fn extract_element(self, offset: usize) -> Val<T> {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let index: Val<u32> = fn_ref.constant(offset as u32);
        let raw = b
            .build_extract_element(self.typed(), index.typed(), "extractelem")
            .expect("Extract element should suceed");
        unsafe { Val::new(fn_ref, UntypedValue(raw)) }
    }

    pub fn insert_element(self, element: Val<T>, offset: usize) -> Self {
        let fn_ref = self.fn_ref().clone();
        let b = unsafe { fn_ref.curr_bb_builder() };
        let index: Val<u32> = fn_ref.constant(offset as u32);
        let raw = b
            .build_insert_element(self.typed(), element.typed(), index.typed(), "extractelem")
            .expect("Extract element should suceed");
        unsafe { Val::new(fn_ref, UntypedValue(raw.into())) }
    }

    pub fn windows<const W: usize>(
        self,
    ) -> (
        impl Iterator<Item = Val<V<T, W>>>,
        impl Iterator<Item = Val<T>>,
    )
    where
        T: Copy,
    {
        let num_windows = N / W;
        let rem = N % W;
        let winself = self.copy();
        let windows = (0..num_windows)
            .map(|w| w * W)
            .map(move |offset| winself.copy().extract_vec(offset));
        let rest_offset = num_windows * W;
        let rest = (rest_offset..rest_offset + rem).map(move |o| {
            let b = unsafe { self.fn_ref().curr_bb_builder() };
            let index = self.fn_ref().constant(o as u32).typed();
            let raw_elem = b
                .build_extract_element(self.typed(), index, "rest_extract")
                .expect("extract element should succeed");
            unsafe { Val::new(self.fn_ref().clone(), UntypedValue(raw_elem)) }
        });
        (windows, rest)
    }

    pub fn map_elementwise<U: VecTy>(self, map: impl Fn(Val<T>) -> Val<U>) -> Val<V<U, N>> {
        Val::from_elements(self.elements().map(map))
    }

    pub fn map_windows<const W: usize, U: VecTy>(
        self,
        mapv: impl Fn(Val<V<T, W>>) -> Val<V<U, W>>,
        map: impl Fn(Val<T>) -> Val<U>,
    ) -> Val<V<U, N>> {
        let mut ret = V::undef_val(self.fn_ref().clone());
        let (vecs, elems) = self.windows();
        let mut offset = 0;
        for v in vecs {
            let trans = mapv(v);
            ret = ret.insert_vec(trans, offset);
            offset += W;
        }
        for e in elems {
            let trans = map(e);
            ret = ret.insert_element(trans, offset);
            offset += 1;
        }
        ret
    }
}
