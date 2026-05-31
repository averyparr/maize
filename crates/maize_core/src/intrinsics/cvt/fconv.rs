use super::{FloatRoundMode, Intrinsic};

use crate::{
    func::{FnArgs, FnRetTy},
    tipe::{BF16, F8E4M3, F8E5M2, F16, V},
};

macro_rules! impl_intrinsics {
    ($($struct_name: ident : $name: literal ($($args: ty),*) -> $ret: ty),* $(,)?) => {
        $(
            #[derive(Default)]
            pub struct $struct_name;
            impl Intrinsic for $struct_name {
                type Args = ($($args,)*);
                type Ret = $ret;
                fn call(self, args: <Self::Args as FnArgs>::ArgValues) -> <Self::Ret as FnRetTy>::RetVal {
                    let fn_ref = args.0.fn_ref().clone();
                    let func = fn_ref
                        .get_intrinsic::<Self::Ret, Self::Args>($name)
                        .expect("This intrinsic should exist");
                    fn_ref.call_extern(func, args, None)
                }
            }
        )*
    };
}

impl_intrinsics!(
    F32CvtBF16Rn: "llvm.nvvm.f2bf16.rn"(f32) -> BF16,
    F32CvtBF16Rz: "llvm.nvvm.f2bf16.rz"(f32) -> BF16,
    F32CvtBF16RnRelu: "llvm.nvvm.f2bf16.rn.relu"(f32) -> BF16,
    F32CvtBF16RzRelu: "llvm.nvvm.f2bf16.rz.relu"(f32) -> BF16,
    F32CvtBF16RnSatfinite: "llvm.nvvm.f2bf16.rn.satfinite"(f32) -> BF16,
    F32CvtBF16RzSatfinite: "llvm.nvvm.f2bf16.rz.satfinite"(f32) -> BF16,
    F32CvtBF16RnReluSatfinite: "llvm.nvvm.f2bf16.rn.relu.satfinite"(f32) -> BF16,
    F32CvtBF16RzReluSatfinite: "llvm.nvvm.f2bf16.rz.relu.satfinite"(f32) -> BF16,

    F16x2CvtE4M3x2Rn: "llvm.nvvm.f16x2.to.e4m3x2.rn" (V<F16, 2>) -> V<F8E4M3, 2>,
    F16x2CvtE4M3x2RnRelu: "llvm.nvvm.f16x2.to.e4m3x2.rn.relu" (V<F16, 2>) -> V<F8E4M3, 2>,
    F16x2CvtE5M2x2Rn: "llvm.nvvm.f16x2.to.e5m2x2.rn" (V<F16, 2>) -> V<F8E5M2, 2>,
    F16x2CvtE5M2x2RnRelu: "llvm.nvvm.f16x2.to.e5m2x2.rn.relu" (V<F16, 2>) -> V<F8E5M2, 2>,

    F32CvtF16Rn: "llvm.nvvm.f2f16.rn"(f32) -> F16,
    F32CvtF16Rz: "llvm.nvvm.f2f16.rz"(f32) -> F16,
    F32CvtF16RnRelu: "llvm.nvvm.f2f16.rn.relu"(f32) -> F16,
    F32CvtF16RzRelu: "llvm.nvvm.f2f16.rz.relu"(f32) -> F16,
    F32CvtF16RnSatfinite: "llvm.nvvm.f2f16.rn.satfinite"(f32) -> F16,
    F32CvtF16RzSatfinite: "llvm.nvvm.f2f16.rz.satfinite"(f32) -> F16,
    F32CvtF16RnReluSatfinite: "llvm.nvvm.f2f16.rn.relu.satfinite"(f32) -> F16,
    F32CvtF16RzReluSatfinite: "llvm.nvvm.f2f16.rz.relu.satfinite"(f32) -> F16,

    F32CvtTF32Rn: "llvm.nvvm.f2tf32.rn"(f32) -> f32,
    F32CvtTF32Rz: "llvm.nvvm.f2tf32.rz"(f32) -> f32,
    F32CvtTF32RnRelu: "llvm.nvvm.f2tf32.rn.relu"(f32) -> f32,
    F32CvtTF32RzRelu: "llvm.nvvm.f2tf32.rz.relu"(f32) -> f32,
    F32CvtTF32RnSatfinite: "llvm.nvvm.f2tf32.rn.satfinite"(f32) -> f32,
    F32CvtTF32RzSatfinite: "llvm.nvvm.f2tf32.rz.satfinite"(f32) -> f32,
    F32CvtTF32RnReluSatfinite: "llvm.nvvm.f2tf32.rn.relu.satfinite"(f32) -> f32,
    F32CvtTF32RzReluSatfinite: "llvm.nvvm.f2tf32.rz.relu.satfinite"(f32) -> f32,

    F32x2CvtE4M3x2Rn: "llvm.nvvm.ff.to.e4m3x2.rn"(f32, f32) -> V<F8E4M3, 2>,
    F32x2CvtE4M3x2RnRelu: "llvm.nvvm.ff.to.e4m3x2.rn.relu"(f32, f32) -> V<F8E4M3, 2>,
    F32x2CvtE5M2x2Rn: "llvm.nvvm.ff.to.e5m2x2.rn"(f32, f32) -> V<F8E5M2, 2>,
    F32x2CvtE5M2x2RnRelu: "llvm.nvvm.ff.to.e5m2x2.rn.relu"(f32, f32) -> V<F8E5M2, 2>,

    F32x2CvtBF16x2Rn: "llvm.nvvm.ff2bf16x2.rn"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2Rz: "llvm.nvvm.ff2bf16x2.rz"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2RnRelu: "llvm.nvvm.ff2bf16x2.rn.relu"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2RzRelu: "llvm.nvvm.ff2bf16x2.rz.relu"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2RnSatfinite: "llvm.nvvm.ff2bf16x2.rn.satfinite"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2RzSatfinite: "llvm.nvvm.ff2bf16x2.rz.satfinite"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2RnReluSatfinite: "llvm.nvvm.ff2bf16x2.rn.relu.satfinite"(f32, f32) -> V<BF16, 2>,
    F32x2CvtBF16x2RzReluSatfinite: "llvm.nvvm.ff2bf16x2.rz.relu.satfinite"(f32, f32) -> V<BF16, 2>,

    F32x2CvtF16x2Rn: "llvm.nvvm.ff2f16x2.rn"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2Rz: "llvm.nvvm.ff2f16x2.rz"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2RnRelu: "llvm.nvvm.ff2f16x2.rn.relu"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2RzRelu: "llvm.nvvm.ff2f16x2.rz.relu"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2RnSatfinite: "llvm.nvvm.ff2f16x2.rn.satfinite"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2RzSatfinite: "llvm.nvvm.ff2f16x2.rz.satfinite"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2RnReluSatfinite: "llvm.nvvm.ff2f16x2.rn.relu.satfinite"(f32, f32) -> V<F16, 2>,
    F32x2CvtF16x2RzReluSatfinite: "llvm.nvvm.ff2f16x2.rz.relu.satfinite"(f32, f32) -> V<F16, 2>,

    E4M3x2CvtF16x2Rn: "llvm.nvvm.e4m3x2.to.f16x2.rn"(V<F8E4M3, 2>) -> V<F16, 2>,
    E4M3x2CvtF16x2RnRelu: "llvm.nvvm.e4m3x2.to.f16x2.rn.relu"(V<F8E4M3, 2>) -> V<F16, 2>,
    E5M2x2CvtF16x2Rn: "llvm.nvvm.e5m2x2.to.f16x2.rn"(V<F8E5M2, 2>) -> V<F16, 2>,
    E5M2x2CvtF16x2RnRelu: "llvm.nvvm.e5m2x2.to.f16x2.rn.relu"(V<F8E5M2, 2>) -> V<F16, 2>,
);

macro_rules! float_round_dispatch {
    ($($new_name: ident ($($args: ty),*) -> $ret: ty => $rn: ident | $rz: ident),* $(,)?) => {
        $(
            #[derive(Default)]
            pub struct $new_name {
                round: FloatRoundMode,
            }

            impl $new_name {
                pub fn rn() -> Self {
                    Self { round: FloatRoundMode::Rn }
                }
                pub fn rz() -> Self {
                    Self { round: FloatRoundMode::Rz }
                }
            }

            impl Intrinsic for $new_name {
                type Args = ($($args,)*);
                type Ret = $ret;
                fn call(
                    self,
                    args: <Self::Args as FnArgs>::ArgValues,
                ) -> <Self::Ret as FnRetTy>::RetVal {
                    match self.round {
                        FloatRoundMode::Rn => return $rn.call(args),
                        FloatRoundMode::Rz => return $rz.call(args),
                    }
                }
            }
        )*
    };
}

float_round_dispatch!(
    F32CvtBF16Relu (f32) -> BF16 => F32CvtBF16RnRelu | F32CvtBF16RzRelu,
    F32CvtBF16Satfinite (f32) -> BF16 => F32CvtBF16RnSatfinite | F32CvtBF16RzSatfinite,
    F32CvtBF16ReluSatfinite (f32) -> BF16 => F32CvtBF16RnReluSatfinite | F32CvtBF16RzReluSatfinite,

    F32x2CvtBF16x2Relu (f32, f32) -> V<BF16, 2> => F32x2CvtBF16x2RnRelu | F32x2CvtBF16x2RzRelu,
    F32x2CvtBF16x2Satfinite (f32, f32) -> V<BF16, 2> => F32x2CvtBF16x2RnSatfinite | F32x2CvtBF16x2RzSatfinite,
    F32x2CvtBF16x2ReluSatfinite (f32, f32) -> V<BF16, 2> => F32x2CvtBF16x2RnReluSatfinite | F32x2CvtBF16x2RzReluSatfinite,

    F32CvtF16Relu (f32) -> F16 => F32CvtF16RnRelu | F32CvtF16RzRelu,
    F32CvtF16Satfinite (f32) -> F16 => F32CvtF16RnSatfinite | F32CvtF16RzSatfinite,
    F32CvtF16ReluSatfinite (f32) -> F16 => F32CvtF16RnReluSatfinite | F32CvtF16RzReluSatfinite,

    F32x2CvtF16x2Relu (f32, f32) -> V<F16, 2> => F32x2CvtF16x2RnRelu | F32x2CvtF16x2RzRelu,
    F32x2CvtF16x2Satfinite (f32, f32) -> V<F16, 2> => F32x2CvtF16x2RnSatfinite | F32x2CvtF16x2RzSatfinite,
    F32x2CvtF16x2ReluSatfinite (f32, f32) -> V<F16, 2> => F32x2CvtF16x2RnReluSatfinite | F32x2CvtF16x2RzReluSatfinite,
);

macro_rules! multi_cvt_relu_satfinite {
    ($($struct_name: ident => $base_rn: ident | $base_rz: ident | $relu: ident | $satfinite: ident | $relu_satfinite: ident),* $(,)?) => {
        $(
            #[derive(Default, Clone, Copy)]
            pub struct $struct_name {
                round: FloatRoundMode,
                relu: bool,
                satfinite: bool,
            }

            impl $struct_name {
                pub fn rn() -> Self {
                    Self {
                        round: FloatRoundMode::Rn,
                        ..Default::default()
                    }
                }
                pub fn rz() -> Self {
                    Self {
                        round: FloatRoundMode::Rz,
                        ..Default::default()
                    }
                }
                pub fn satfinite(self) -> Self {
                    Self {
                        satfinite: true,
                        ..self
                    }
                }
                pub fn relu(self) -> Self {
                    Self { relu: true, ..self }
                }
            }

            impl Intrinsic for $struct_name {
                type Args = <$base_rn as Intrinsic>::Args;
                type Ret = <$base_rn as Intrinsic>::Ret;
                fn call(self, args: <Self::Args as FnArgs>::ArgValues) -> <Self::Ret as FnRetTy>::RetVal {
                    match (self.relu, self.satfinite) {
                        (true, true) => return $relu_satfinite { round: self.round }.call(args),
                        (true, false) => return $relu { round: self.round }.call(args),
                        (false, true) => return $satfinite { round: self.round }.call(args),
                        (false, false) => match self.round {
                            FloatRoundMode::Rn => return $base_rn.call(args),
                            FloatRoundMode::Rz => return $base_rz.call(args),
                        },
                    }
                }
            }

        )*
    };
}

macro_rules! multi_cvt_relu {
    ($($struct_name: ident => $base: ident | $relu: ident),* $(,)?) => {
        $(
            #[derive(Default, Clone, Copy)]
            pub struct $struct_name {
                relu: bool,
            }

            impl $struct_name {
                pub fn relu() -> Self {
                    Self { relu: true }
                }
            }

            impl Intrinsic for $struct_name {
                type Args = <$base as Intrinsic>::Args;
                type Ret = <$base as Intrinsic>::Ret;
                fn call(self, args: <Self::Args as FnArgs>::ArgValues) -> <Self::Ret as FnRetTy>::RetVal {
                    if self.relu {
                        $relu.call(args)
                    } else {
                        $base.call(args)
                    }
                }
            }
        )*
    };
}

multi_cvt_relu_satfinite!(
    F32x2CvtBF16x2 => F32x2CvtBF16x2Rn | F32x2CvtBF16x2Rz | F32x2CvtBF16x2Relu | F32x2CvtBF16x2Satfinite | F32x2CvtBF16x2ReluSatfinite,
    F32x2CvtF16x2 => F32x2CvtF16x2Rn | F32x2CvtF16x2Rz | F32x2CvtF16x2Relu | F32x2CvtF16x2Satfinite | F32x2CvtF16x2ReluSatfinite,
    F32CvtBF16 => F32CvtBF16Rn | F32CvtBF16Rz | F32CvtBF16Relu | F32CvtBF16Satfinite | F32CvtBF16ReluSatfinite,
    F32CvtF16 => F32CvtF16Rn | F32CvtF16Rz | F32CvtF16Relu | F32CvtF16Satfinite | F32CvtF16ReluSatfinite,
);

multi_cvt_relu!(
    F32x2CvtE4M3x2 => F32x2CvtE4M3x2Rn | F32x2CvtE4M3x2RnRelu,
    F32x2CvtE5M2x2 => F32x2CvtE5M2x2Rn | F32x2CvtE5M2x2RnRelu,

    F16x2CvtE4M3x2 => F16x2CvtE4M3x2Rn | F16x2CvtE4M3x2RnRelu,
    F16x2CvtE5M2x2 => F16x2CvtE5M2x2Rn | F16x2CvtE5M2x2RnRelu,

    E4M3x2CvtF16x2 => E4M3x2CvtF16x2Rn | E4M3x2CvtF16x2RnRelu,
    E5M3x2CvtF16x2 => E5M2x2CvtF16x2Rn | E5M2x2CvtF16x2RnRelu,
);
