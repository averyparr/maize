use crate::tuple::{Tuple, TupleOrI64};
use pyo3::prelude::*;

#[derive(FromPyObject, Clone, PartialEq)]
pub enum StrideOrU64 {
    Stride(Stride),
    Root(u64),
}

impl From<StrideOrU64> for TupleOrI64 {
    fn from(value: StrideOrU64) -> Self {
        match value {
            StrideOrU64::Stride(stride) => Self::Tuple(stride.0),
            StrideOrU64::Root(r) => {
                assert!(r <= i64::MAX as u64);
                Self::Root(r as i64)
            }
        }
    }
}

#[pyclass(from_py_object)]
#[derive(Clone, PartialEq)]
pub struct Stride(Tuple);

#[pymethods]
impl Stride {
    #[new]
    #[pyo3(signature = (*values))]
    fn new(values: Vec<StrideOrU64>) -> Self {
        Self(Tuple::new(values.into_iter().map(Into::into).collect()))
    }

    pub fn __repr__(&self) -> String {
        self.0.__repr__()
    }
}
