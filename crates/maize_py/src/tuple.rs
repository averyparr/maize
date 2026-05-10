use pyo3::prelude::*;

#[derive(FromPyObject, Clone, PartialEq)]
pub(crate) enum TupleOrI64 {
    Tuple(Tuple),
    Root(i64),
}

#[pyclass(from_py_object)]
#[derive(Clone, PartialEq)]
pub struct Tuple(Box<[TupleOrI64]>);

#[pymethods]
impl Tuple {
    #[new]
    #[pyo3(signature = (*values))]
    pub(crate) fn new(values: Vec<TupleOrI64>) -> Self {
        Self(values.into_boxed_slice())
    }

    pub fn __repr__(&self) -> String {
        let mut ret = String::with_capacity(2 + 5 * self.0.len());

        ret.push('(');
        for v in &self.0 {
            match v {
                TupleOrI64::Tuple(tuple) => ret.push_str(&tuple.__repr__()),
                TupleOrI64::Root(r) => ret.push_str(&format!("{}", *r)),
            }
            ret.push_str(", ")
        }

        if ret.len() > 2 {
            ret.pop();
            ret.pop();
        }
        ret.push(')');

        ret
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
