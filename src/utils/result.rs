use std::ops::Try;
use tide::IntoResponse;
use tide::Response;

pub enum RResponse<T, E> {
    Valid(T),
    Invalid(E),
}

impl<T: IntoResponse, E: IntoResponse> IntoResponse for RResponse<T, E> {
    fn into_response(self) -> Response {
        match self {
            RResponse::Valid(r) => r.into_response(),
            RResponse::Invalid(e) => e.into_response(),
        }
    }
}

impl<T, E> Try for RResponse<T, E> {
    type Ok = T;
    type Error = E;
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            RResponse::Valid(r) => Ok(r),
            RResponse::Invalid(e) => Err(e),
        }
    }
    fn from_error(v: Self::Error) -> Self {
        RResponse::Invalid(v)
    }
    fn from_ok(v: Self::Ok) -> Self {
        RResponse::Valid(v)
    }
}

impl<T, E> From<std::result::Result<T, E>> for RResponse<T, E> {
    fn from(res: std::result::Result<T, E>) -> Self {
        match res {
            Ok(v) => RResponse::Valid(v),
            Err(e) => RResponse::Invalid(e),
        }
    }
}
