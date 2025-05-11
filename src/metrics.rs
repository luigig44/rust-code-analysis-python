use std::path::PathBuf;

use rust_code_analysis::{guess_language, metrics, Callback, FuncSpace, ParserTrait, action};
use serde::Serialize;


/// Payload containing source code used to compute metrics.
#[derive(Debug)]
pub struct MetricsPayload {
    /// Payload identifier.
    pub id: String,
    /// Source code filename.
    pub file_name: String,
    /// Source code used to compute metrics.
    pub code: String,
    /// Flag to consider only unit space metrics.
    pub unit: bool,
}

/// Server response containing metrics for every space present in
/// the requested source code.
#[derive(Debug, Serialize)]
pub struct MetricsResponse {
    /// Server response identifier.
    pub id: String,
    /// Source code programming language.
    pub language: String,
    /// Metrics for every space contained in the requested source code.
    ///
    /// If `None`, an error occurred processing the request.
    pub spaces: Option<FuncSpace>,
}

/// Server request configuration.
#[derive(Debug)]
pub struct MetricsCfg {
    /// Request identifier.
    pub id: String,
    /// Path to the source file.
    pub path: PathBuf,
    /// Flag to consider only unit space metrics.
    pub unit: bool,
    /// Source code programming language.
    pub language: String,
}

/// Unit structure to implement the `Callback` trait.
pub struct MetricsCallback;

impl Callback for MetricsCallback {
    type Res = MetricsResponse;
    type Cfg = MetricsCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        let spaces = metrics(parser, &cfg.path);
        let spaces = if cfg.unit {
            if let Some(mut spaces) = spaces {
                spaces.spaces.clear();
                Some(spaces)
            } else {
                None
            }
        } else {
            spaces
        };

        MetricsResponse {
            id: cfg.id,
            language: cfg.language,
            spaces,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub id: String,
    pub error: &'static str,
}

pub fn metrics_rust(payload: MetricsPayload) -> Result<MetricsResponse, Error> {
    let path = PathBuf::from(&payload.file_name);
    let buf = payload.code.into_bytes();
    let (language, name) = guess_language(&buf, &path);
    if let Some(language) = language {
        let cfg = MetricsCfg {
            id: payload.id,
            path,
            unit: payload.unit,
            language: name.to_string(),
        };
        Ok(action::<MetricsCallback>(
            &language,
            buf,
            &PathBuf::from(""),
            None,
            cfg,
        ))
    } else {
        Err(Error {
            id: payload.id,
            error: "The file extension doesn't correspond to a valid language",
        })
    }
}