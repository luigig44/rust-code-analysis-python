use rust_code_analysis::{Callback, LANG, ParserTrait, action, guess_language, rm_comments};
use std::path::PathBuf;

/// Payload containing source code with comments to be removed.
#[derive(Debug)]
pub struct CommentRemovalPayload {
    /// Source code filename.
    pub file_name: String,
    /// Source code with comments to be removed.
    pub code: String,
}

pub type CommentRemovalResponse = Result<Vec<u8>, String>;

/// Unit structure to implement the `Callback` trait.
#[derive(Debug)]
pub struct CommentRemovalCallback;

impl Callback for CommentRemovalCallback {
    type Res = CommentRemovalResponse;
    type Cfg = ();

    fn call<T: ParserTrait>(_cfg: Self::Cfg, parser: &T) -> Self::Res {
        rm_comments(parser).ok_or("Failed to remove comments".to_string())
    }
}

pub fn comment_removal_rust(payload: CommentRemovalPayload) -> CommentRemovalResponse {
    let path = PathBuf::from(payload.file_name);
    let buf = payload.code.into_bytes();
    let (language, _) = guess_language(&buf, path);
    if let Some(language) = language {
        let language = if language == LANG::Cpp {
            LANG::Ccomment
        } else {
            language
        };
        action::<CommentRemovalCallback>(&language, buf, &PathBuf::from(""), None, ())
    } else {
        Err("The file extension doesn't correspond to a valid language".to_string())
    }
}
