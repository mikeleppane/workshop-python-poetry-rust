use pyo3::prelude::*;

pub mod chudnovsky;
use chudnovsky::chudnovsky;

#[pyfunction]
fn chudnovsky_pi(py: Python, digits: u32) -> PyResult<String> {
    py.allow_threads(move || chudnovsky(digits))
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_pidigits(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(chudnovsky_pi, m)?)?;
    Ok(())
}
