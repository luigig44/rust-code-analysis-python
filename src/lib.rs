use std::path::PathBuf;

use pyo3::prelude::*;

use pyo3::types::PyDict;
use rust_code_analysis::{AstCallback, AstCfg, AstPayload, LANG, action, guess_language};


use rust_code_analysis::{Callback, ParserTrait, rm_comments};
mod metrics;
use metrics::{metrics_rust, MetricsPayload, MetricsResponse};
/// Unit structure to implement the `Callback` trait.
#[derive(Debug)]
pub struct CommentRemoval;

impl Callback for CommentRemoval {
    type Res = Option<String>;
    type Cfg = ();

    fn call<T: ParserTrait>(_cfg: Self::Cfg, parser: &T) -> Self::Res {
        rm_comments(parser).map(|v| String::from_utf8_lossy(&v).into_owned())
    }
}


#[pyfunction]
fn comment_removal(file_name: String, code: String) -> PyResult<String> {
    let path = PathBuf::from(file_name);
    let buf = code.into_bytes();
    let (language, _) = guess_language(&buf, path);
    if let Some(language) = language {
        let language = if language == LANG::Cpp {
            LANG::Ccomment
        } else {
            language
        };
        action::<CommentRemoval>(
            &language,
            buf,
            &PathBuf::from(""),
            None,
            (),
        ).ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to remove comments"))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid language"))
    }
}

#[pyfunction]
fn metrics_py(file_name: String, code: String, unit: bool) -> PyResult<Py<PyAny>> {
    let payload = MetricsPayload {
        id: "1".to_string(),
        file_name,
        code,
        unit,
    };
    let response = metrics_rust(payload);
    Python::with_gil(|py| {
        response.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.error))
        .and_then(|response| {
            match response.spaces {
                None => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Failed to compute metrics")),
                Some(_) => Ok(response)
            }
        }).and_then(|response| {
            pythonize::pythonize(py, &response)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
                .map(|v| v.into())
        })
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_code_analysis_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(comment_removal, m)?)?;
    m.add_function(wrap_pyfunction!(metrics_py, m)?)?;
    Ok(())
}
