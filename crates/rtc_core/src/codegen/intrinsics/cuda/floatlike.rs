use crate::{
    ty::{
        Ty, V,
        primitive::{F16, F32, F64},
    },
    val::Val,
};

pub trait FloatLike: Ty + Copy {
    /*
     *  |---------------------|
     *  |   Unary Functions   |
     *  |---------------------|
     */
    const EX2_APPROX: &str;
    const EX2_APPROX_FTZ: Option<&str>;

    const ABS: &str;
    const ABS_FTZ: Option<&str>;

    /*
     *  |---------------------|
     *  |   Binary Functions  |
     *  |---------------------|
     */

    const MIN: &str;
    const MIN_FTZ: Option<&str>;

    const MAX: &str;
    const MAX_FTZ: Option<&str>;
}

/// Separate because only F32 + F64 have these
pub trait F32F64Intrinsics: FloatLike {
    const LG2_APPROX: &str;
    const LG2_APPROX_FTZ: Option<&str>;

    const FLOOR: &str;
    const FLOOR_FTZ: Option<&str>;
    const CEIL: &str;
    const CEIL_FTZ: Option<&str>;

    const RSQRT_APPROX: &str;
    const RSQRT_APPROX_FTZ: Option<&str>;

    const SQRT_RN: &str;
    const SQRT_RZ: &str;
    const SQRT_RM: &str;
    const SQRT_RP: &str;

    const SQRT_RN_FTZ: Option<&str>;
    const SQRT_RZ_FTZ: Option<&str>;
    const SQRT_RM_FTZ: Option<&str>;
    const SQRT_RP_FTZ: Option<&str>;

    const SQRT_APPROX: Option<&str>;
    const SQRT_APPROX_FTZ: Option<&str>;

    const RCP_RN: &str;
    const RCP_RZ: &str;
    const RCP_RM: &str;
    const RCP_RP: &str;

    const RCP_RN_FTZ: Option<&str>;
    const RCP_RZ_FTZ: Option<&str>;
    const RCP_RM_FTZ: Option<&str>;
    const RCP_RP_FTZ: Option<&str>;

    const RCP_APPROX_FTZ: &str;
}

// Only f32, f16, f16x2, bf16, bf16x2 implement
pub trait MinMaxExtended: FloatLike {
    const MAX_FTZ_GUARANTEED: &str;
    const MAX_NAN: &str;
    const MAX_NAN_FTZ: &str;

    const MAX_XORSIGN_ABS: &str;
    const MAX_XORSIGN_ABS_FTZ: &str;
    const MAX_XORSIGN_ABS_NAN: &str;
    const MAX_XORSIGN_ABS_FTZ_NAN: &str;

    const MIN_FTZ_GUARANTEED: &str;
    const MIN_NAN: &str;
    const MIN_NAN_FTZ: &str;

    const MIN_XORSIGN_ABS: &str;
    const MIN_XORSIGN_ABS_FTZ: &str;
    const MIN_XORSIGN_ABS_NAN: &str;
    const MIN_XORSIGN_ABS_FTZ_NAN: &str;
}

/// Separate this because only F32 has support for these
pub trait SinCosFloat: FloatLike {
    const SIN_APPROX: &str;
    const SIN_APPROX_FTZ: Option<&str>;
    const COS_APPROX: &str;
    const COS_APPROX_FTZ: Option<&str>;
}

impl FloatLike for F32 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.f";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.f");

    const ABS: &str = "llvm.nvvm.fabs.f";
    const ABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.f");

    const MIN: &str = "llvm.nvvm.fmin.f";
    const MIN_FTZ: Option<&str> = Some("llvm.nvvm.fmin.ftz.f");

    const MAX: &str = "llvm.nvvm.fmax.f";
    const MAX_FTZ: Option<&str> = Some("llvm.nvvm.fmax.ftz.f");
}

impl MinMaxExtended for F32 {
    const MAX_FTZ_GUARANTEED: &str = "llvm.nvvm.fmax.ftz.f";
    const MAX_NAN: &str = "llvm.nvvm.fmax.nan.f";
    const MAX_NAN_FTZ: &str = "llvm.nvvm.fmax.ftz.nan.f";

    const MAX_XORSIGN_ABS: &str = "llvm.nvvm.fmax.xorsign.abs.f";
    const MAX_XORSIGN_ABS_FTZ: &str = "llvm.nvvm.fmax.ftz.xorsign.abs.f";
    const MAX_XORSIGN_ABS_NAN: &str = "llvm.nvvm.fmax.nan.xorsign.abs.f";
    const MAX_XORSIGN_ABS_FTZ_NAN: &str = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f";

    const MIN_FTZ_GUARANTEED: &str = "llvm.nvvm.fmin.ftz.f";
    const MIN_NAN: &str = "llvm.nvvm.fmin.nan.f";
    const MIN_NAN_FTZ: &str = "llvm.nvvm.fmin.ftz.nan.f";

    const MIN_XORSIGN_ABS: &str = "llvm.nvvm.fmin.xorsign.abs.f";
    const MIN_XORSIGN_ABS_FTZ: &str = "llvm.nvvm.fmin.ftz.xorsign.abs.f";
    const MIN_XORSIGN_ABS_NAN: &str = "llvm.nvvm.fmin.nan.xorsign.abs.f";
    const MIN_XORSIGN_ABS_FTZ_NAN: &str = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f";
}

impl F32F64Intrinsics for F32 {
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.f";
    const LG2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.lg2.approx.ftz.f");

    const FLOOR: &str = "llvm.nvvm.floor.f";
    const FLOOR_FTZ: Option<&str> = Some("llvm.nvvm.floor.ftz.f");
    const CEIL: &str = "llvm.nvvm.ceil.f";
    const CEIL_FTZ: Option<&str> = Some("llvm.nvvm.ceil.ftz.f");

    const RSQRT_APPROX: &str = "llvm.nvvm.rsqrt.approx.f";
    const RSQRT_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.rsqrt.approx.ftz.f");

    const SQRT_RN: &str = "llvm.nvvm.sqrt.rn.f";
    const SQRT_RZ: &str = "llvm.nvvm.sqrt.rz.f";
    const SQRT_RM: &str = "llvm.nvvm.sqrt.rm.f";
    const SQRT_RP: &str = "llvm.nvvm.sqrt.rp.f";

    const SQRT_RN_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rn.ftz.f");
    const SQRT_RZ_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rz.ftz.f");
    const SQRT_RM_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rm.ftz.f");
    const SQRT_RP_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.rp.ftz.f");

    const SQRT_APPROX: Option<&str> = Some("llvm.nvvm.sqrt.approx.f");
    const SQRT_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.sqrt.approx.ftz.f");

    const RCP_RN: &str = "llvm.nvvm.rcp.rn.f";
    const RCP_RZ: &str = "llvm.nvvm.rcp.rz.f";
    const RCP_RM: &str = "llvm.nvvm.rcp.rm.f";
    const RCP_RP: &str = "llvm.nvvm.rcp.rp.f";

    const RCP_RN_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rn.ftz.f");
    const RCP_RZ_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rz.ftz.f");
    const RCP_RM_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rm.ftz.f");
    const RCP_RP_FTZ: Option<&str> = Some("llvm.nvvm.rcp.rp.ftz.f");

    const RCP_APPROX_FTZ: &str = "llvm.nvvm.rcp.approx.ftz.f";
}

impl SinCosFloat for F32 {
    const SIN_APPROX: &str = "llvm.nvvm.sin.approx.f";
    const SIN_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.sin.approx.ftz.f");
    const COS_APPROX: &str = "llvm.nvvm.cos.approx.f";
    const COS_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.cos.approx.ftz.f");
}

impl FloatLike for F64 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.d";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.d");

    const ABS: &str = "llvm.nvvm.fabs.d";
    const ABS_FTZ: Option<&str> = None;

    const MIN: &str = "llvm.nvvm.fmin.d";
    const MIN_FTZ: Option<&str> = None;

    const MAX: &str = "llvm.nvvm.fmax.d";
    const MAX_FTZ: Option<&str> = None;
}

impl F32F64Intrinsics for F64 {
    const LG2_APPROX: &str = "llvm.nvvm.lg2.approx.d";
    const LG2_APPROX_FTZ: Option<&str> = None;

    const FLOOR: &str = "llvm.nvvm.floor.d";
    const FLOOR_FTZ: Option<&str> = None;
    const CEIL: &str = "llvm.nvvm.ceil.d";
    const CEIL_FTZ: Option<&str> = None;

    const RSQRT_APPROX: &str = "llvm.nvvm.rsqrt.approx.d";
    const RSQRT_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.rsqrt.approx.ftz.d");

    const SQRT_RN: &str = "llvm.nvvm.sqrt.rn.d";
    const SQRT_RZ: &str = "llvm.nvvm.sqrt.rz.d";
    const SQRT_RM: &str = "llvm.nvvm.sqrt.rm.d";
    const SQRT_RP: &str = "llvm.nvvm.sqrt.rp.d";

    const SQRT_RN_FTZ: Option<&str> = None;
    const SQRT_RZ_FTZ: Option<&str> = None;
    const SQRT_RM_FTZ: Option<&str> = None;
    const SQRT_RP_FTZ: Option<&str> = None;

    const SQRT_APPROX: Option<&str> = None;
    const SQRT_APPROX_FTZ: Option<&str> = None;

    const RCP_RN: &str = "llvm.nvvm.rcp.rn.d";
    const RCP_RZ: &str = "llvm.nvvm.rcp.rz.d";
    const RCP_RM: &str = "llvm.nvvm.rcp.rm.d";
    const RCP_RP: &str = "llvm.nvvm.rcp.rp.d";

    const RCP_RN_FTZ: Option<&str> = None;
    const RCP_RZ_FTZ: Option<&str> = None;
    const RCP_RM_FTZ: Option<&str> = None;
    const RCP_RP_FTZ: Option<&str> = None;

    const RCP_APPROX_FTZ: &str = "llvm.nvvm.rcp.approx.ftz.d";
}

impl FloatLike for F16 {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.f16";
    const EX2_APPROX_FTZ: Option<&str> = None;

    const ABS: &str = "llvm.nvvm.fabs.f16";
    const ABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.f16");

    const MIN: &str = "llvm.nvvm.fmin.f16";
    const MIN_FTZ: Option<&str> = Some("llvm.nvvm.fmin.ftz.f16");

    const MAX: &str = "llvm.nvvm.fmax.f16";
    const MAX_FTZ: Option<&str> = Some("llvm.nvvm.fmax.ftz.f16");
}

impl MinMaxExtended for F16 {
    const MAX_FTZ_GUARANTEED: &str = "llvm.nvvm.fmax.ftz.f16";
    const MAX_NAN: &str = "llvm.nvvm.fmax.nan.f16";
    const MAX_NAN_FTZ: &str = "llvm.nvvm.fmax.ftz.nan.f16";

    const MAX_XORSIGN_ABS: &str = "llvm.nvvm.fmax.xorsign.abs.f16";
    const MAX_XORSIGN_ABS_FTZ: &str = "llvm.nvvm.fmax.ftz.xorsign.abs.f16";
    const MAX_XORSIGN_ABS_NAN: &str = "llvm.nvvm.fmax.nan.xorsign.abs.f16";
    const MAX_XORSIGN_ABS_FTZ_NAN: &str = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.f16";

    const MIN_FTZ_GUARANTEED: &str = "llvm.nvvm.fmin.ftz.f16";
    const MIN_NAN: &str = "llvm.nvvm.fmin.nan.f16";
    const MIN_NAN_FTZ: &str = "llvm.nvvm.fmin.ftz.nan.f16";

    const MIN_XORSIGN_ABS: &str = "llvm.nvvm.fmin.xorsign.abs.f16";
    const MIN_XORSIGN_ABS_FTZ: &str = "llvm.nvvm.fmin.ftz.xorsign.abs.f16";
    const MIN_XORSIGN_ABS_NAN: &str = "llvm.nvvm.fmin.nan.xorsign.abs.f16";
    const MIN_XORSIGN_ABS_FTZ_NAN: &str = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.f16";
}

impl FloatLike for V<F16, 2> {
    const EX2_APPROX: &str = "llvm.nvvm.ex2.approx.v2f16";
    const EX2_APPROX_FTZ: Option<&str> = Some("llvm.nvvm.ex2.approx.ftz.v2f16");

    const ABS: &str = "llvm.nvvm.fabs.v2f16";
    const ABS_FTZ: Option<&str> = Some("llvm.nvvm.fabs.ftz.v2f16");

    const MIN: &str = "llvm.nvvm.fmin.v2f16";
    const MIN_FTZ: Option<&str> = Some("llvm.nvvm.fmin.ftz.v2f16");

    const MAX: &str = "llvm.nvvm.fmax.f16";
    const MAX_FTZ: Option<&str> = Some("llvm.nvvm.fmax.ftz.v2f16");
}

impl MinMaxExtended for V<F16, 2> {
    const MAX_FTZ_GUARANTEED: &str = "llvm.nvvm.fmax.ftz.v2f16";
    const MAX_NAN: &str = "llvm.nvvm.fmax.nan.v2f16";
    const MAX_NAN_FTZ: &str = "llvm.nvvm.fmax.ftz.nan.v2f16";

    const MAX_XORSIGN_ABS: &str = "llvm.nvvm.fmax.xorsign.abs.v2f16";
    const MAX_XORSIGN_ABS_FTZ: &str = "llvm.nvvm.fmax.ftz.xorsign.abs.v2f16";
    const MAX_XORSIGN_ABS_NAN: &str = "llvm.nvvm.fmax.nan.xorsign.abs.v2f16";
    const MAX_XORSIGN_ABS_FTZ_NAN: &str = "llvm.nvvm.fmax.ftz.nan.xorsign.abs.v2f16";

    const MIN_FTZ_GUARANTEED: &str = "llvm.nvvm.fmin.ftz.v2f16";
    const MIN_NAN: &str = "llvm.nvvm.fmin.nan.v2f16";
    const MIN_NAN_FTZ: &str = "llvm.nvvm.fmin.ftz.nan.v2f16";

    const MIN_XORSIGN_ABS: &str = "llvm.nvvm.fmin.xorsign.abs.v2f16";
    const MIN_XORSIGN_ABS_FTZ: &str = "llvm.nvvm.fmin.ftz.xorsign.abs.v2f16";
    const MIN_XORSIGN_ABS_NAN: &str = "llvm.nvvm.fmin.nan.xorsign.abs.v2f16";
    const MIN_XORSIGN_ABS_FTZ_NAN: &str = "llvm.nvvm.fmin.ftz.nan.xorsign.abs.v2f16";
}

impl<Float> Val<'_, Float>
where
    Float: FloatLike,
{
    pub fn exp2(&self) -> Self {
        let intrinsic_name = Float::EX2_APPROX;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn exp2_ftz(&self) -> Self {
        let intrinsic_name = Float::EX2_APPROX_FTZ.unwrap_or(Float::EX2_APPROX);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn abs(&self) -> Self {
        let intrinsic_name = Float::ABS;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn abs_ftz(&self) -> Self {
        let intrinsic_name = Float::ABS_FTZ.unwrap_or(Float::ABS);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn log2(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::LG2_APPROX;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn log2_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::LG2_APPROX_FTZ.unwrap_or(Float::LG2_APPROX);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn floor(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::FLOOR;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn floor_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::FLOOR_FTZ.unwrap_or(Float::FLOOR);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn ceil(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::CEIL;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn ceil_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::CEIL_FTZ.unwrap_or(Float::CEIL);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn rcp(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RN;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RN_FTZ.unwrap_or(Float::RCP_RN);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_rz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RZ;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_rz_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RZ_FTZ.unwrap_or(Float::RCP_RZ);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_rm(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RM;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_rm_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RM_FTZ.unwrap_or(Float::RCP_RM);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_rp(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RP;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_rp_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_RP_FTZ.unwrap_or(Float::RCP_RP);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rcp_approx_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RCP_APPROX_FTZ;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn rsqrt_approx(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RSQRT_APPROX;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn rsqrt_approx_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::RSQRT_APPROX_FTZ.unwrap_or(Float::RSQRT_APPROX);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn sqrt(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RN;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RN_FTZ.unwrap_or(Float::SQRT_RN);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_rz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RZ;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_rz_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RZ_FTZ.unwrap_or(Float::SQRT_RZ);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_rm(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RM;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_rm_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RM_FTZ.unwrap_or(Float::SQRT_RM);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_rp(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RP;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_rp_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_RP_FTZ.unwrap_or(Float::SQRT_RP);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_approx(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_APPROX.unwrap_or(Float::SQRT_RN);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sqrt_approx_ftz(&self) -> Self
    where
        Float: F32F64Intrinsics,
    {
        let intrinsic_name = Float::SQRT_APPROX_FTZ
            .or(Float::SQRT_APPROX)
            .unwrap_or(Float::SQRT_RN);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }

    pub fn min(&self, other: &Self) -> Self {
        let intrinsic_name = Float::MIN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_ftz(&self, other: &Self) -> Self {
        let intrinsic_name = Float::MIN_FTZ.unwrap_or(Float::MIN);
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_ftz_guaranteed(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_FTZ_GUARANTEED;
        // Safety: `Float` implements `MinMaxExtended`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_nan(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_NAN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_nan_ftz(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_NAN_FTZ;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        let intrinsic_name = Float::MAX;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_ftz(&self, other: &Self) -> Self {
        let intrinsic_name = Float::MAX_FTZ.unwrap_or(Float::MIN);
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_ftz_guaranteed(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_FTZ_GUARANTEED;
        // Safety: `Float` implements `MinMaxExtended`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_nan(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_NAN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_nan_ftz(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_NAN_FTZ;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }

    // Commented out because LLVM often panics if you
    // use nvptx64 as backend but not specifically
    // nvptx64-nvidia-cuda. This makes it hard
    // to tell when these will be available, and
    // I personly am not familiar with a use case.
    //
    pub fn max_by_abs(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_XORSIGN_ABS;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_by_abs_ftz(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_XORSIGN_ABS_FTZ;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_by_abs_nan(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_XORSIGN_ABS_NAN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn max_by_abs_ftz_nan(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MAX_XORSIGN_ABS_FTZ_NAN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_by_abs(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_XORSIGN_ABS;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_by_abs_ftz(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_XORSIGN_ABS_FTZ;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_by_abs_nan(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_XORSIGN_ABS_NAN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
    pub fn min_by_abs_ftz_nan(&self, other: &Self) -> Self
    where
        Float: MinMaxExtended,
    {
        let intrinsic_name = Float::MIN_XORSIGN_ABS_FTZ_NAN;
        // Safety: `Float` implements `FloatLike`
        unsafe {
            self.cm()
                .call_binary_function(*self, *other, intrinsic_name)
        }
    }
}

impl<Float> Val<'_, Float>
where
    Float: SinCosFloat,
{
    pub fn cos_approx(&self) -> Self {
        let intrinsic_name = Float::COS_APPROX;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn cos_approx_ftz(&self) -> Self {
        let intrinsic_name = Float::COS_APPROX_FTZ.unwrap_or(Float::COS_APPROX);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sin_approx(&self) -> Self {
        let intrinsic_name = Float::SIN_APPROX;
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
    pub fn sin_approx_ftz(&self) -> Self {
        let intrinsic_name = Float::SIN_APPROX_FTZ.unwrap_or(Float::SIN_APPROX);
        // Safety: `Float` implements `FloatLike`
        unsafe { self.cm().call_unary_function(*self, intrinsic_name) }
    }
}
