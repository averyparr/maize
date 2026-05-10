use pyo3::prelude::*;

use crate::tuple::{Tuple, TupleOrI64};

#[derive(FromPyObject, Clone, PartialEq)]
pub enum ShapeOrU64 {
    Shape(Shape),
    Root(u64),
}

impl From<ShapeOrU64> for TupleOrI64 {
    fn from(value: ShapeOrU64) -> Self {
        match value {
            ShapeOrU64::Shape(shape) => Self::Tuple(shape.0),
            ShapeOrU64::Root(r) => {
                assert!(r <= i64::MAX as u64);
                Self::Root(r as i64)
            }
        }
    }
}

#[pyclass(from_py_object)]
#[derive(Clone, PartialEq)]
pub struct Shape(Tuple);

#[pymethods]
impl Shape {
    #[new]
    #[pyo3(signature = (*values))]
    pub(crate) fn new(values: Vec<ShapeOrU64>) -> Self {
        Self(Tuple::new(values.into_iter().map(Into::into).collect()))
    }

    pub fn __repr__(&self) -> String {
        self.0.__repr__()
    }
}
