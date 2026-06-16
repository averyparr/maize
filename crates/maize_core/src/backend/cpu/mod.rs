use std::fmt::Debug;

use crate::{
    backend::{FnCtx, FnRef},
    intrinsics::IntrinsicsLibrary,
};

pub mod cuda;

pub trait ToCPU: Debug {
    fn cpu(&self) -> &str;
    fn triple(&self) -> &str;
    fn features(&self) -> &str {
        ""
    }
    fn provide_intrinsics(&self, fn_ref: FnRef, f: &dyn Fn(&dyn IntrinsicsLibrary));
}
