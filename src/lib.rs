use pyo3::prelude::*;
mod ant;
mod wall;
mod compute;
mod constants;

/// A Python module implemented in Rust.
#[pymodule]
mod nea {
        use pyo3::prelude::*;

        #[pymodule_export]
        use super::compute::tick_n;
        #[pymodule_export]
        use super::compute::init_pheromones;
        #[pymodule_export]
        use super::constants::get_dimensions;

        /// Formats the sum of two numbers as string.
        #[pyfunction]
        #[pyo3(warn(message = "2"))]
        fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
                println!("2");
                Ok((a + b+2).to_string())
        }

        //#[pymodule_export]
        //use super::ant::Ant;
}
