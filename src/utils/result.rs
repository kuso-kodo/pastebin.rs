use std::ops::Try;
use tide::IntoResponse;
use tide::Response;

/// Our common **EndPoint** type.
/// Acts like original `std::Result` type.
pub enum APIResponse<T, E> {
    Valid(T),
    Invalid(E),
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for APIResponse<T, E> {
    /// Convert a `APIResponse` type into `tide::Response`.
    fn into_response(self) -> Response {
        match self {
            APIResponse::Valid(r) => r.into_response(),
            APIResponse::Invalid(e) => e.into_response(),
        }
    }
}

// Implements the unstable `try_trait`.
// Hence nightly version of rust is needed for now.
impl<T, E> Try for APIResponse<T, E> {
    type Ok = T;
    type Error = E;
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            APIResponse::Valid(r) => Ok(r),
            APIResponse::Invalid(e) => Err(e),
        }
    }
    fn from_error(v: Self::Error) -> Self {
        APIResponse::Invalid(v)
    }
    fn from_ok(v: Self::Ok) -> Self {
        APIResponse::Valid(v)
    }
}

impl<T, E> From<std::result::Result<T, E>> for APIResponse<T, E> {
    /// Convert `std::result` into out `APIResponse`.
    fn from(res: std::result::Result<T, E>) -> Self {
        match res {
            Ok(v) => APIResponse::Valid(v),
            Err(e) => APIResponse::Invalid(e),
        }
    }
}
