pub mod browsing;

use serde::Serialize;
use crate::{SUBSONIC_VERSION, SUBSONIC_XMLNS};

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct ErrorResponse {
    xmlns: &'static str,
    status: &'static str,
    version: &'static str,
    error: Error,
}

impl ErrorResponse {
    fn new(code: u32, message: &str) -> Self {
        ErrorResponse {
            xmlns: SUBSONIC_XMLNS,
            status: "failed",
            version: SUBSONIC_VERSION,
            error: Error {
                code,
                message: message.to_owned()
            }
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "error")]
pub struct Error {
    code: u32,
    message: String,
}