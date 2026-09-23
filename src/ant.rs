use pyo3::prelude::*;

#[pyclass]
#[derive(Clone,Debug)]
pub struct Ant {
        pub x: f64,
        pub y: f64,
        pub vx: f64,
        pub vy: f64
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
