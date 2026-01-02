use std::ops::Range;

use inkwell::{builder::Builder, types::VectorType, values::VectorValue};

use crate::{codegen::CodegenModule, ty::Ty};
