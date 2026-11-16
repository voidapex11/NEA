use pyo3::prelude::*;
use crate::compute::is_oob;

#[derive(FromPyObject)]
#[derive(IntoPyObject)]
#[derive(Clone, Debug, PartialEq)]
pub struct Ant {
        pub x: f64,
        pub y: f64,
        pub vx: f64,
        pub vy: f64,
        pub trail_conc: f64,
        pub wait_left: i32,
        pub wait_max: i32,
}

//#[pymethods]
impl Ant {
        pub fn is_oob(&self) -> bool {
                return is_oob(self.x, self.y)
        }
        /*#[new]
        fn new(x: i32, y: i32) -> PyResult<Self> {
                return Ok(Ant { x, y });
        }

        fn draw(&self) -> PyResult<()> {
                Ok(())
        }*/
}
