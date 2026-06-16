use crate::{
    ContextRef, StructType, StructValue,
    backend::{FnRef, UntypedValue},
    tipe::{F16, Ty, V},
    val::Val,
};

type F16x2 = V<F16, 2>;

macro_rules! impl_mma_struct {
    ($(#[$attrs: meta])* pub struct $name: ident([$elem: ty; $len: literal])) => {
        $(#[$attrs])*
        #[derive(Clone, Copy)]
        pub struct $name([$elem; $len]);
        impl Ty for $name {
            type LLType = StructType;
            type LLVal = StructValue;

            fn raw_ty(ctx: ContextRef) -> Self::LLType {
                ctx.struct_type(&[<$elem>::raw_ty(ctx).into(); $len], false)
            }

            fn type_val(val: UntypedValue) -> Self::LLVal {
                val.0.into_struct_value()
            }

            fn const_val(self, fn_ref: FnRef) -> Val<Self>
            where
                Self: Copy,
            {
                let raw_vals = self.0.map(|v| Ty::const_val(v, fn_ref.clone()).typed());
                let b = unsafe { fn_ref.curr_bb_builder() };
                let mut raw = Self::raw_ty(fn_ref.ctx()).get_undef();
                for (index, val) in raw_vals.into_iter().enumerate() {
                    raw = b
                        .build_insert_value(
                            raw,
                            val,
                            index.try_into().expect("usize -> u32 overflow"),
                            "insert",
                        )
                        .expect("Should be able to insert element")
                        .into_struct_value();
                }
                return unsafe { Val::new(fn_ref, UntypedValue(raw.into())) };
            }
        }
    };
}

impl_mma_struct!(
    #[repr(align(8))]
    pub struct MmaAccumF16M16N8([F16x2; 2])
);

impl_mma_struct!(
    #[repr(align(16))]
    pub struct MmaAccumF32M16N8([f32; 4])
);

impl_mma_struct!(
    #[repr(align(32))]
    pub struct MmaAccumF64M16N8([f64; 4])
);

impl_mma_struct!(
    #[repr(align(16))]
    pub struct LegacyMmaAccumF16M8N8([F16x2; 4])
);

impl_mma_struct!(
    #[repr(align(32))]
    pub struct LegacyMmaAccumF32M8N8([f32; 8])
);

impl_mma_struct!(
    #[repr(align(16))]
    pub struct MmaAccumF64M8N8([f64; 2])
);
