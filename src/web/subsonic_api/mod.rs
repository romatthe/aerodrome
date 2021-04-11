pub mod browsing;

use serde::Serialize;

// TODO Determine correct version
pub const SUBSONIC_VERSION: &str = "1.8.0";
pub const SUBSONIC_XMLNS: &str = "http://subsonic.org/restapi";

const SUBSONIC_ERROR_GENERIC: u32 = 0;
const SUBSONIC_ERROR_PARAM_MISSING: u32 = 10;
const SUBSONIC_ERROR_INCOMPATIBLE_CLIENT: u32 = 20;
const SUBSONIC_ERROR_INCOMPATIBLE_SERVER: u32 = 30;
const SUBSONIC_ERROR_USERNAME_PASSWORD: u32 = 40;
const SUBSONIC_ERROR_TOKEN_UNSUPPORTED: u32 = 41;
const SUBSONIC_ERROR_UNAUTHORIZED: u32 = 50;
const SUBSONIC_ERROR_TRIAL_OVER: u32 = 60; // Unused, for obvious reasons
const SUBSONIC_ERROR_NOT_FOUND: u32 = 70;

pub enum SubsonicError {
    Generic,               // 0
    ParameterMissing,      // 10
    IncompatibleClient,    // 20
    IncompatibleServer,    // 30
    WrongUsernamePassword, // 40
    TokenUnsupported,      // 41
    Unauthorized,          // 50
    TrialOver,             // 60
    DataNotFound,          // 70
}

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
                message: message.to_owned(),
            },
        }
    }

    fn from(error: SubsonicError, message: &str) -> Self {
        let code = match error {
            SubsonicError::Generic => SUBSONIC_ERROR_GENERIC,
            SubsonicError::ParameterMissing => SUBSONIC_ERROR_PARAM_MISSING,
            SubsonicError::IncompatibleClient => SUBSONIC_ERROR_INCOMPATIBLE_CLIENT,
            SubsonicError::IncompatibleServer => SUBSONIC_ERROR_INCOMPATIBLE_SERVER,
            SubsonicError::WrongUsernamePassword => SUBSONIC_ERROR_USERNAME_PASSWORD,
            SubsonicError::TokenUnsupported => SUBSONIC_ERROR_TOKEN_UNSUPPORTED,
            SubsonicError::Unauthorized => SUBSONIC_ERROR_UNAUTHORIZED,
            SubsonicError::TrialOver => SUBSONIC_ERROR_TRIAL_OVER,
            SubsonicError::DataNotFound => SUBSONIC_ERROR_NOT_FOUND,
        };

        ErrorResponse {
            xmlns: SUBSONIC_XMLNS,
            status: "failed",
            version: SUBSONIC_VERSION,
            error: Error {
                code,
                message: message.to_owned(),
            },
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "error")]
pub struct Error {
    code: u32,
    message: String,
}
