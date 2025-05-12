use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

mod backend;
use backend::comment::{CommentRemovalPayload, comment_removal_rust};
use backend::metrics::{MetricsPayload, metrics_rust};

/// comment_removal(file_name: str, code: str) -> str
/// 
/// Removes comments from the provided code.
/// Imitates the behavior of the `comment_removal` REST API endpoint of `rust-code-analysis-web`.
/// 
/// Parameters
/// ----------
/// file_name : str
///     The name of the file being processed (used to infer the language)
/// code : str
///     The source code string from which comments will be removed
/// 
/// Returns
/// -------
/// A string containing the code with comments removed.
#[pyfunction]
fn comment_removal(file_name: String, code: String) -> PyResult<String> {
    let payload = CommentRemovalPayload { file_name, code };
    let response = comment_removal_rust(payload);

    response
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .map_err(PyErr::new::<PyValueError, _>)
}

/// metrics(file_name: str, code: str, unit: bool) -> dict
///
/// Calculates various code metrics for the provided code.
/// Imitates the behavior of the `metrics` REST API endpoint of `rust-code-analysis-web`.
///
/// Parameters
/// ----------
/// file_name : str
///     The name of the file being analyzed (used to infer the language)
/// code : str
///     The source code string to analyze
/// unit : bool
///     A boolean flag. True returns only top level metrics, False returns metrics recursively.
///
/// Returns
/// -------
/// A dictionary containing the calculated metrics.
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

/// rust-code-analysis-python
/// =========================
///
/// Implements Python bindings for the rust-code-analysis crate.
///
/// This module provides two main functions:
/// * `comment_removal`: Removes comments from source code
/// * `metrics`: Calculates various code metrics
#[pymodule]
fn rust_code_analysis_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(comment_removal, m)?)?;
    m.add_function(wrap_pyfunction!(metrics, m)?)?;
    Ok(())
}
