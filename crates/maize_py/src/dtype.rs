use std::str::FromStr;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[derive(PartialEq, Clone, Copy)]
#[pyclass(from_py_object)]
pub enum DType {
    F32,
    F64,
}

impl DType {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::F32 => "F32",
            Self::F64 => "F64",
        }
    }
}

impl FromStr for DType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F32" => Ok(Self::F32),
            "F64" => Ok(Self::F64),
            _ => Err(()),
        }
    }
}

#[pymethods]
impl DType {
    #[staticmethod]
    pub fn try_from_str(val: &str) -> Option<Self> {
        Self::from_str(val).ok()
    }

    #[staticmethod]
    pub fn from_str(val: &str) -> PyResult<Self> {
        Self::try_from_str(val)
            .ok_or_else(|| PyValueError::new_err(format!("Invalid DType name '{val}'")))
    }
}
