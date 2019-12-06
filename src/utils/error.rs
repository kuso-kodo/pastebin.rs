use crate::utils::APIResponse;
use crate::utils::APIResult;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;
use tide::IntoResponse;
use tide::Response;

pub type Result = APIResponse<APIResult, Error>;

/// The generic Error type in out application.
///
/// This struct implements a **IntoResponse** trait
/// provided by __tide__ , hence it can be converted
/// into Response directly. In the struct, the **error_status**
/// field contains the HTTP status code which specifies the
/// error type and **error_info** which may contain the
/// reason of the Error.
#[derive(Serialize, Deserialize)]
pub struct Error {
    error_status: u16,
    error_info: String,
}

impl Error {
    /// Construct a new Error using HTTP status code and error info code.
    pub fn new(error_status: u16, error_info: &str) -> Self {
        Error {
            error_status: error_status,
            error_info: error_info.to_string(),
        }
    }

    /// Simply returns a Internal Error.
    #[allow(unused)]
    pub fn internal_error() -> Self {
        Self::new(500, "Internal Server Error.")
    }

    /// Construct a new Error by HTTP::StatusCode.
    pub fn from_http_status(code: StatusCode) -> Self {
        Self::new(code.as_u16(), code.canonical_reason().unwrap())
    }
}

impl IntoResponse for Error {
    /// Convert the Error value into a `Response`.
    fn into_response(self) -> Response {
        Response::new(self.error_status)
            .body_json(&self.error_info)
            .unwrap()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.error_info, f)
    }
}

impl std::convert::From<std::io::Error> for Error {
    /// Returns `BAD_REQUEST` to client when an io::Error occurs.
    fn from(_: std::io::Error) -> Self {
        Self::from_http_status(StatusCode::BAD_REQUEST)
    }
}

impl std::convert::From<diesel::result::Error> for Error {
    /// Returns `INTERNAL_ERROR` to client when an diesel::result::Error occurs.
    fn from(_: diesel::result::Error) -> Self {
        Self::internal_error()
    }
}

impl std::convert::From<!> for Error {
    /// Returns `BAD_REQUEST` to client when an diesel::result::Error occurs.
    fn from(_: !) -> Self {
        Self::from_http_status(StatusCode::BAD_REQUEST)
    }
}

impl std::convert::From<uuid::parser::ParseError> for Error {
    /// Returns `BAD_REQUEST` to client when an diesel::result::Error occurs.
    fn from(_: uuid::parser::ParseError) -> Self {
        Self::from_http_status(StatusCode::BAD_REQUEST)
    }
}
