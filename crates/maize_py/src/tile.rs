use pyo3::prelude::*;

use crate::{dtype::DType, shape::Shape};

#[pyclass(from_py_object)]
#[derive(PartialEq, Clone)]
pub struct Tile {
    shape: Shape,
    dtype: DType,
}

impl Tile {
    fn new(shape: Shape, dtype: DType) -> Self {
        Self { shape, dtype }
    }
}

#[pymethods]
impl Tile {
    #[new]
    fn py_new(shape: Shape, dtype: DType) -> Self {
        Self::new(shape, dtype.into())
    }

    fn __repr__(&self) -> String {
        let shape = self.shape.__repr__();
        let dtype = self.dtype.to_str();
        format!("Tile(shape={}, dtype={})", shape, dtype)
    }
}
