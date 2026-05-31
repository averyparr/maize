mod fconv;

pub use fconv::*;

use crate::func::{FnArgs, FnRetTy};

pub trait Intrinsic {
    type Args: FnArgs;
    type Ret: FnRetTy;
    fn call(self, args: <Self::Args as FnArgs>::ArgValues) -> <Self::Ret as FnRetTy>::RetVal;
}

#[derive(Default, Clone, Copy)]
enum FloatRoundMode {
    #[default]
    Rn,
    Rz,
}
