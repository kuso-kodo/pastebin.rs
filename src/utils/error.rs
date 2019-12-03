use crate::utils::RResponse;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;
use tide::IntoResponse;
use tide::Response;

pub type Result = RResponse<Response, Error>;

#[derive(Serialize, Deserialize)]
pub struct Error {
    error_status: u16,
    error_info: String,
}

impl Error {
    pub fn new(error_status: u16, error_info: &str) -> Self {
        Error {
            error_status: error_status,
            error_info: error_info.to_string(),
        }
    }

    pub fn internal_error() -> Self {
        Self::new(500, "Internal Server Error.")
    }

    pub fn from_http_status(code: StatusCode) -> Self {
        Self::new(code.as_u16(), code.canonical_reason().unwrap())
    }
}

impl IntoResponse for Error {
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
    fn from(_: std::io::Error) -> Self {
        Self::from_http_status(StatusCode::BAD_REQUEST)
    }
}

impl std::convert::From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Self::new(500, &err.to_string())
    }
}
