use pyo3::prelude::*;
use mylib::{Rectangle};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn rj(x: f32, y: f32, x_span: f32, y_span: f32)-> PyResult<String>{
    Ok(Rectangle::new(x, y, x_span, y_span).j().unwrap())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pybind22(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(rj, m)?)?;
    Ok(())
}