use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

mod backend;
use backend::comment::{CommentRemovalPayload, comment_removal_rust};
use backend::metrics::{MetricsPayload, metrics_rust};

#[pyfunction]
fn comment_removal(file_name: String, code: String) -> PyResult<String> {
    let payload = CommentRemovalPayload { file_name, code };
    let response = comment_removal_rust(payload);

    response
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .map_err(PyErr::new::<PyValueError, _>)
}

#[pyfunction]
fn metrics(file_name: String, code: String, unit: bool) -> PyResult<Py<PyAny>> {
    let payload = MetricsPayload {
        file_name,
        code,
        unit,
    };
    let response = metrics_rust(payload);

    response
        .and_then(|response| {
            Python::with_gil(|py| {
                pythonize::pythonize(py, &response)
                    .map_err(|e| e.to_string())
                    .map(|v| v.into())
            })
        })
        .map_err(PyErr::new::<PyValueError, _>)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_code_analysis_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(comment_removal, m)?)?;
    m.add_function(wrap_pyfunction!(metrics, m)?)?;
    Ok(())
}
