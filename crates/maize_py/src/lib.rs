use pyo3::prelude::*;

mod dtype;
mod shape;
mod stride;
mod tile;
mod tuple;

#[pymodule]
fn _bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<dtype::DType>()?;
    m.add_class::<shape::Shape>()?;
    m.add_class::<stride::Stride>()?;
    m.add_class::<tile::Tile>()?;
    Ok(())
}
