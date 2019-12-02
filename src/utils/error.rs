use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::IntoResponse;
use tide::Response;

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
