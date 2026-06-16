use std::marker::PhantomData;

use inkwell::values::BasicValue;

use crate::{
    StructType, StructValue,
    backend::UntypedValue,
    tipe::{A, Ty},
    val::Val,
};

pub trait OffsetFields {
    /// Value at index i descibes where field i
    /// lies in the Rust layout of this struct,
    /// excluding zero-sized types
    fn field_ranks() -> &'static [usize];
    fn same_layout_as_packed() -> bool;
    fn nonzero_field_count() -> usize;
}

#[derive(Clone, Copy)]
pub struct Extractor<S, F>(PhantomData<(S, F)>, usize);

impl<S: Ty<LLType = StructType, LLVal = StructValue>, F: Ty> Extractor<S, F> {
    pub fn new(idx: usize) -> Self {
        Self(PhantomData, idx)
    }
    pub fn field(self, val: Val<S>) -> Val<F> {
        get_struct_val_at_idx(val, self.1)
    }

    pub fn field_ptr(self, val: Val<*const S>) -> Val<*const F> {
        get_struct_ptr_at_idx(val, self.1)
    }
    pub fn afield_ptr<const ADDRSPACE: u16>(
        self,
        val: Val<A<*const S, ADDRSPACE>>,
    ) -> Val<A<*const F, ADDRSPACE>> {
        get_struct_aptr_at_idx(val, self.1)
    }

    pub fn field_ptr_mut(self, val: Val<*mut S>) -> Val<*mut F> {
        get_struct_ptr_at_idx(val.as_const(), self.1).as_mut()
    }
    pub fn afield_ptr_mut<const ADDRSPACE: u16>(
        self,
        val: Val<A<*mut S, ADDRSPACE>>,
    ) -> Val<A<*mut F, ADDRSPACE>> {
        get_struct_aptr_at_idx(val.as_const(), self.1).as_mut()
    }

    pub fn field_ref(self, val: Val<&S>) -> Val<&F> {
        unsafe { get_struct_ptr_at_idx(val.as_ptr(), self.1).assume_ref() }
    }
    pub fn afield_ref<const ADDRSPACE: u16>(
        self,
        val: Val<A<&S, ADDRSPACE>>,
    ) -> Val<A<&F, ADDRSPACE>> {
        unsafe { get_struct_aptr_at_idx(val.as_ptr(), self.1).assume_ref() }
    }

    pub fn field_mut(self, val: Val<&mut S>) -> Val<&mut F> {
        unsafe {
            get_struct_ptr_at_idx(val.as_ptr(), self.1)
                .as_mut()
                .assume_mut()
        }
    }
    pub fn afield_mut<const ADDRSPACE: u16>(
        self,
        val: Val<A<&mut S, ADDRSPACE>>,
    ) -> Val<A<&mut F, ADDRSPACE>> {
        unsafe {
            get_struct_aptr_at_idx(val.as_ptr(), self.1)
                .as_mut()
                .assume_mut()
        }
    }
}

pub fn arrange_in_field_order<T: OffsetFields, U>(field_slices: &mut [U]) {
    let n = field_slices.len();
    assert!(n <= 64, "only structs with up to 64 fields are supported");
    let field_ranks = T::field_ranks();
    assert_eq!(n, field_ranks.len());

    let mut visited: u64 = 0;

    for start in 0..n {
        if visited & (1 << start) != 0 {
            continue;
        }
        visited |= 1 << start;

        // field_ranks is a gather permutation: result[i] = input[field_ranks[i]].
        // Walk the cycle swapping adjacent elements so each position ends up with
        // the element from field_ranks[i], not the scatter-style swap-to-start.
        let mut cur = start;
        let mut next = field_ranks[start];
        while next != start {
            visited |= 1 << next;
            field_slices.swap(cur, next);
            cur = next;
            next = field_ranks[next];
        }
    }
}

pub fn same_layout_as_packed<T: OffsetFields>() -> bool {
    T::same_layout_as_packed()
}

pub unsafe fn insert_struct_val_at_idx<
    S: crate::tipe::Ty<LLType = crate::StructType, LLVal = crate::StructValue>,
    U: crate::tipe::Ty,
>(
    val: crate::val::Val<S>,
    idx: usize,
    mem: crate::val::Val<U>,
) -> crate::val::Val<S> {
    let v = val.typed();
    let b = unsafe { val.fn_ref().curr_bb_builder() };
    let raw = b
        .build_insert_value(
            v,
            mem.typed(),
            idx.try_into().expect("usize -> u32 overflow"),
            "insert_struct",
        )
        .expect("Extract value should have succeeded");
    unsafe {
        Val::new(
            val.fn_ref().clone(),
            UntypedValue(raw.as_basic_value_enum()),
        )
    }
}

fn get_struct_val_at_idx<
    S: crate::tipe::Ty<LLType = crate::StructType, LLVal = crate::StructValue>,
    U: crate::tipe::Ty,
>(
    val: crate::val::Val<S>,
    idx: usize,
) -> crate::val::Val<U> {
    let v = val.typed();
    let b = unsafe { val.fn_ref().curr_bb_builder() };
    let raw = b
        .build_extract_value(
            v,
            idx.try_into().expect("usize -> u32 overflow"),
            "extract_struct",
        )
        .expect("Extract value should have succeeded");
    unsafe { crate::val::Val::new(val.fn_ref().clone(), crate::tipe::UntypedValue(raw)) }
}

fn get_struct_ptr_at_idx<
    S: crate::tipe::Ty<LLType = crate::StructType, LLVal = crate::StructValue>,
    U: crate::tipe::Ty,
>(
    val: crate::val::Val<*const S>,
    idx: usize,
) -> crate::val::Val<*const U> {
    let v = val.typed();
    let b = unsafe { val.fn_ref().curr_bb_builder() };
    let raw = unsafe {
        b.build_in_bounds_gep(
            S::raw_ty(val.fn_ref().ctx()),
            v,
            &[
                val.constant(0u32).typed(),
                val.constant::<u32>(idx.try_into().expect("usize -> u32 overflow"))
                    .typed(),
            ],
            "extract_struct",
        )
        .expect("Extract value should have succeeded")
    };
    unsafe { crate::val::Val::new(val.fn_ref().clone(), crate::tipe::UntypedValue(raw.into())) }
}

fn get_struct_aptr_at_idx<
    S: crate::tipe::Ty<LLType = crate::StructType, LLVal = crate::StructValue>,
    U: crate::tipe::Ty,
    const ADDRSPACE: u16,
>(
    val: crate::val::Val<A<*const S, ADDRSPACE>>,
    idx: usize,
) -> crate::val::Val<A<*const U, ADDRSPACE>> {
    let v = val.typed();
    let b = unsafe { val.fn_ref().curr_bb_builder() };
    let raw = unsafe {
        b.build_in_bounds_gep(
            S::raw_ty(val.fn_ref().ctx()),
            v,
            &[
                val.constant(0u32).typed(),
                val.constant::<u32>(idx.try_into().expect("usize -> u32 overflow"))
                    .typed(),
            ],
            "extract_struct",
        )
        .expect("Extract value should have succeeded")
    };
    unsafe { crate::val::Val::new(val.fn_ref().clone(), crate::tipe::UntypedValue(raw.into())) }
}

#[macro_export]
macro_rules! reflect_on_struct {
    (
        $(#[$($annotations: meta)*])*
        $svis: vis struct $name: ident$(<$($lts: lifetime),* $(,)? $($tipes: ident),*> $(,)?)?
        $(where $($prelt: lifetime: $prelt_bound: tt,)* $($prety: ident: $prety_bound: tt,)*)?
        {
            $(
                $fvis: vis $field: ident: $ty: ty
            ),*
            $(,)?
        }
    ) => {
        $(#[$($annotations)*])*
        $svis struct $name$(<$($lts,)* $($tipes,)*>)?
        $(where $($prelt: $prelt_bound,)* $($prety: $prety_bound,)*)?
        {
            $(
                $fvis $field: $ty
            ),*
        }


        impl$(<$($lts,)* $($tipes: $crate::tipe::Ty,)*>)? $name$(<$($lts,)* $($tipes,)*>)?
        $(where $($prelt: $prelt_bound,)* $($prety: $prety_bound,)*)?
        {
            #[doc(hidden)]
            fn __get_field_idx_from_offset_size(offset: usize, size: usize) -> usize {
                let mut inc = 0;
                let mut matcher = [$(
                    (::std::mem::offset_of!(Self, $field), ::std::mem::size_of::<$ty>()),
                )*];
                $crate::tipe::reflection::arrange_in_field_order::<Self, _>(&mut matcher);
                for (idx, (foff, fsz)) in matcher.into_iter().enumerate() {
                    if offset == foff && size == fsz {
                        return idx;
                    }
                }
                ::std::unreachable!();
            }
            $(
              $fvis fn $field() -> $crate::tipe::reflection::Extractor<Self, $ty> {
                  let idx = Self::__get_field_idx_from_offset_size(
                      ::std::mem::offset_of!(Self, $field),
                      ::std::mem::size_of::<$ty>()
                  );
                  $crate::tipe::reflection::Extractor::new(idx)
              }
            )*
        }

        impl$(<$($lts,)* $($tipes,)*>)? $crate::tipe::reflection::OffsetFields for $name$(<$($lts,)* $($tipes,)*>)?
        $(where $($prelt: $prelt_bound,)* $($prety: $prety_bound,)*)?
        {
            fn field_ranks() -> &'static [usize] {
                static FIELD_OFFSETS: ::std::sync::OnceLock<&'static [usize]> = ::std::sync::OnceLock::new();
                FIELD_OFFSETS.get_or_init( || {
                    let mut idx = 0..;
                    let mut indexed_offsets = [
                        $((
                            idx.next().unwrap(),
                            ::std::mem::offset_of!(Self, $field),
                            ::std::mem::size_of::<$ty>(),
                        ),)*
                    ];
                    indexed_offsets.sort_by_key(|&(_, offset, size)| {
                        if size != 0 {
                            offset
                        } else {
                            ::std::usize::MAX
                        }
                    });
                    let res: ::std::vec::Vec<_> = indexed_offsets
                        .into_iter()
                        .map(|(idx, _, _)| idx)
                        .collect();
                    ::std::vec::Vec::leak(res)
                })
            }
            fn same_layout_as_packed() -> bool {
                ::std::mem::size_of::<Self>() == 0 $(+ ::std::mem::size_of::<$ty>())*
            }
            fn nonzero_field_count() -> usize {
                [$(
                    ::std::mem::size_of::<$ty>(),
                )*]
                .into_iter()
                .filter(|&sz| sz != 0)
                .count()
            }
        }

        impl$(<$($lts,)* $($tipes: $crate::tipe::Ty,)*>)? $crate::tipe::Ty for $name$(<$($lts,)* $($tipes,)*>)?
        $(where $($prelt: $prelt_bound,)* $($prety: $prety_bound,)*)?
        {
            type LLType = $crate::StructType;
            type LLVal = $crate::StructValue;

            fn raw_ty(ctx: $crate::ContextRef) -> Self::LLType {
                let mut field_types = [$(
                    $crate::BasicType::as_basic_type_enum(&<$ty>::raw_ty(ctx)),
                )*];
                let mut tipe_slice = field_types.as_mut_slice();
                $crate::tipe::reflection::arrange_in_field_order::<Self, _>(&mut tipe_slice);
                let nz_count = <Self as $crate::tipe::reflection::OffsetFields>::nonzero_field_count();
                ctx.struct_type(
                    &tipe_slice[..nz_count],
                    $crate::tipe::reflection::same_layout_as_packed::<Self>(),
                )
            }
            fn type_val(val: $crate::backend::UntypedValue) -> Self::LLVal {
                val.0.into_struct_value()
            }

            fn const_val(self, fn_ref: $crate::backend::FnRef) -> $crate::val::Val<Self>
            where
                Self: Sized,
            {
                let mut field_vals = [$(
                    $crate::BasicValue::as_basic_value_enum(&self.$field.const_val(fn_ref.clone()).typed()),
                )*];
                let mut val_slice = field_vals.as_mut_slice();
                $crate::tipe::reflection::arrange_in_field_order::<Self, _>(&mut val_slice);
                let nz_count = <Self as $crate::tipe::reflection::OffsetFields>::nonzero_field_count();

                let raw = Self::raw_ty(fn_ref.ctx()).const_named_struct(&field_vals[..nz_count]);

                unsafe { $crate::val::Val::new(fn_ref, $crate::backend::UntypedValue::new(raw)) }
            }
        }
    };
}

reflect_on_struct!(
    struct Inner<'a, A> {
        a: A,
        pub(crate) v: &'a [f32; 10],
    }
);

reflect_on_struct!(
    struct Outer<'a, A>
    where
        'a: 'static,
        A: Copy,
    {
        inner: Inner<'a, A>,
        other: &'static u64,
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::offset_of;

    reflect_on_struct!(
        struct TwoFields {
            a: u32,
            b: u64,
        }
    );

    reflect_on_struct!(
        struct ThreeFields {
            a: u8,
            b: u32,
            c: u16,
        }
    );

    reflect_on_struct!(
        struct TwoSameSize {
            x: u8,
            y: u8,
        }
    );

    // Generic struct: layout depends on the concrete types chosen for A and B.
    reflect_on_struct!(
        struct Pair<A, B> {
            a: A,
            b: B,
        }
    );

    // Where-clause struct: layout handling should be identical to the unconstrained case.
    reflect_on_struct!(
        struct Bounded<A>
        where
            A: Copy,
        {
            val: A,
            tag: u32,
        }
    );

    // Copy needed for Val::copy() in the PTX tests.
    impl Clone for TwoFields {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Copy for TwoFields {}
    impl Clone for ThreeFields {
        fn clone(&self) -> Self {
            *self
        }
    }
    impl Copy for ThreeFields {}

    // Ground-truth invariant: field_ranks + arrange_in_field_order must yield offsets in
    // ascending order, regardless of how Rust internally laid out the struct.
    #[test]
    fn arrange_two_fields_sorts_offsets() {
        let mut offsets = [offset_of!(TwoFields, a), offset_of!(TwoFields, b)];
        arrange_in_field_order::<TwoFields, _>(&mut offsets);
        assert!(offsets[0] <= offsets[1], "offsets not sorted: {offsets:?}");
    }

    #[test]
    fn arrange_three_fields_sorts_offsets() {
        let mut offsets = [
            offset_of!(ThreeFields, a),
            offset_of!(ThreeFields, b),
            offset_of!(ThreeFields, c),
        ];
        arrange_in_field_order::<ThreeFields, _>(&mut offsets);
        assert!(
            offsets[0] <= offsets[1] && offsets[1] <= offsets[2],
            "offsets not sorted: {offsets:?}",
        );
    }

    // Direct test of the cycle-sort with an identity permutation: nothing should move.
    struct IdentityRanks;
    impl OffsetFields for IdentityRanks {
        fn field_ranks() -> &'static [usize] {
            &[0, 1, 2]
        }
        fn same_layout_as_packed() -> bool {
            true
        }
        fn nonzero_field_count() -> usize {
            3
        }
    }

    #[test]
    fn arrange_identity_is_noop() {
        let mut v = [10u32, 20, 30];
        arrange_in_field_order::<IdentityRanks, _>(&mut v);
        assert_eq!(v, [10, 20, 30]);
    }

    #[test]
    fn nonzero_field_count_two() {
        assert_eq!(TwoFields::nonzero_field_count(), 2);
    }

    #[test]
    fn nonzero_field_count_three() {
        assert_eq!(ThreeFields::nonzero_field_count(), 3);
    }

    // Two u8 fields cannot have alignment padding, so the struct must be packed.
    #[test]
    fn same_layout_as_packed_no_padding() {
        assert!(TwoSameSize::same_layout_as_packed());
    }

    // u32 + u64 always has padding regardless of field order.
    #[test]
    fn same_layout_as_packed_with_padding() {
        assert!(!TwoFields::same_layout_as_packed());
    }

    // --- generics: Pair<A, B> ---

    fn check_is_permutation(ranks: &[usize]) {
        let mut sorted = ranks.to_vec();
        sorted.sort();
        let expected: Vec<usize> = (0..ranks.len()).collect();
        assert_eq!(
            sorted, expected,
            "field_ranks is not a valid permutation: {ranks:?}"
        );
    }

    #[test]
    fn pair_field_ranks_are_valid_permutations() {
        check_is_permutation(<Pair<u32, u64>>::field_ranks());
        check_is_permutation(<Pair<u64, u32>>::field_ranks());
        check_is_permutation(<Pair<u8, u64>>::field_ranks());
    }

    // field_ranks[0] should identify the field with the smallest memory offset.
    #[test]
    fn pair_field_ranks_reflect_actual_layout() {
        let ranks = <Pair<u32, u64>>::field_ranks();
        let offsets = [offset_of!(Pair<u32, u64>, a), offset_of!(Pair<u32, u64>, b)];
        let expected_first_decl = if offsets[0] <= offsets[1] { 0 } else { 1 };
        assert_eq!(
            ranks[0], expected_first_decl,
            "field_ranks[0] does not match the lowest-offset field",
        );
    }

    // --- where-clause struct: Bounded<A> ---

    #[test]
    fn bounded_arrange_sorts_offsets() {
        let mut off = [offset_of!(Bounded<u64>, val), offset_of!(Bounded<u64>, tag)];
        arrange_in_field_order::<Bounded<u64>, _>(&mut off);
        assert!(off[0] <= off[1], "Bounded<u64> not sorted: {off:?}");
    }

    #[test]
    fn bounded_nonzero_field_count() {
        assert_eq!(<Bounded<u64>>::nonzero_field_count(), 2);
    }

    // --- PTX-gen tests (gate with --features ptx-gen-tests) ---
    #[cfg(feature = "ptx-gen-tests")]
    mod ptx {
        use super::*;
        use crate::{Opt, SM, backend::LLVM, func::implement_ptx_kernel, tipe::A};

        #[test]
        fn const_struct_stores_correct_values() {
            // ker.constant requires T: Copy; TwoFields has the impl above.
            let ker = implement_ptx_kernel::<(A<&mut u32, 1>, A<&mut u64, 1>)>(
                LLVM::new(),
                "const_struct_test",
            );
            {
                let s = ker.constant(TwoFields { a: 42, b: 100 });
                let (mut out_a, mut out_b) = ker.args();
                out_a.store(TwoFields::a().field(s.copy()));
                out_b.store(TwoFields::b().field(s.copy()));
            }
            ker.return_void();
            let bytes = ker.compile(SM::SM100, Opt::O0);
            let ptx = std::str::from_utf8(&bytes).unwrap();
            assert!(ptx.contains("st.global.b32"), "no b32 store:\n{ptx}");
            assert!(ptx.contains("st.global.b64"), "no b64 store:\n{ptx}");
            // LLVM folds extractvalue on constant aggregates unconditionally, so the literal
            // values must appear as immediates.
            assert!(
                ptx.contains("42") || ptx.contains("0x2a"),
                "constant 42 not found:\n{ptx}",
            );
            assert!(
                ptx.contains("100") || ptx.contains("0x64"),
                "constant 100 not found:\n{ptx}",
            );
        }

        #[test]
        fn field_extraction_two_fields() {
            // Val::new asserts type equality on every Val construction, so if the wrong LLVM
            // struct field index is used here the test panics with a clear type-mismatch.
            let ker = implement_ptx_kernel::<(A<&TwoFields, 1>, A<&mut u32, 1>, A<&mut u64, 1>)>(
                LLVM::new(),
                "extract_two_fields",
            );
            {
                let (src, mut out_a, mut out_b) = ker.args();
                let s = unsafe { src.load() };
                out_a.store(TwoFields::a().field(s.copy()));
                out_b.store(TwoFields::b().field(s.copy()));
            }
            ker.return_void();
            let bytes = ker.compile(SM::SM100, Opt::O0);
            let ptx = std::str::from_utf8(&bytes).unwrap();
            assert!(ptx.contains("ld.global"), "no global load:\n{ptx}");
            assert!(ptx.contains("st.global.b32"), "no b32 store:\n{ptx}");
            assert!(ptx.contains("st.global.b64"), "no b64 store:\n{ptx}");
        }

        #[test]
        fn field_extraction_three_fields() {
            // Exercises the 3-field cycle path. The type assertion in Val::new catches any
            // wrong-index extraction before even looking at the PTX.
            let ker = implement_ptx_kernel::<(
                A<&ThreeFields, 1>,
                A<&mut u8, 1>,
                A<&mut u32, 1>,
                A<&mut u16, 1>,
            )>(LLVM::new(), "extract_three_fields");
            {
                let (src, mut out_a, mut out_b, mut out_c) = ker.args();
                let s = unsafe { src.load() };
                out_a.store(ThreeFields::a().field(s.copy()));
                out_b.store(ThreeFields::b().field(s.copy()));
                out_c.store(ThreeFields::c().field(s.copy()));
            }
            ker.return_void();
            let bytes = ker.compile(SM::SM100, Opt::O0);
            let ptx = std::str::from_utf8(&bytes).unwrap();
            assert!(ptx.contains("ld.global"), "no global load:\n{ptx}");
            assert!(ptx.contains("st.global.b32"), "no b32 store:\n{ptx}");
            assert!(ptx.contains("st.global.b16"), "no b16 store:\n{ptx}");
        }
    }
}
