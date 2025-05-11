use std::path::PathBuf;

use pyo3::prelude::*;

use rust_code_analysis::{AstCallback, AstCfg, AstPayload, LANG, action, guess_language};


use rust_code_analysis::{Callback, ParserTrait, rm_comments};

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



/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_code_analysis_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(comment_removal, m)?)?;
    Ok(())
}
