use pyo3::prelude::*;

#[pyclass]
pub struct Ant {
        pub x: f64,
        pub y: f64,
}

#[pymethods]
impl Ant {
        /*#[new]
        fn new(x: i32, y: i32) -> PyResult<Self> {
                return Ok(Ant { x, y });
        }

        fn draw(&self) -> PyResult<()> {
                Ok(())
        }*/
}
