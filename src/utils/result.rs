use super::Error;
use std::ops::Try;
use tide::IntoResponse;
use tide::Response;

pub struct ResultWithResponse<T, E> {
    pub result: Result<T, E>,
}

impl<T: IntoResponse, E: Send + Sync> IntoResponse for ResultWithResponse<T, E> {
    fn into_response(self) -> Response {
        match self.result {
            Ok(r) => r.into_response(),
            Err(_) => Error::internal_error().into_response(),
        }
    }
}

impl<T, E> Try for ResultWithResponse<T, E> {
    type Ok = T;
    type Error = E;
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        self.result
    }
    fn from_error(v: Self::Error) -> Self {
        Self { result: Err(v) }
    }
    fn from_ok(v: Self::Ok) -> Self {
        Self { result: Ok(v) }
    }
}

impl<T, E> From<std::result::Result<T, E>> for ResultWithResponse<T, E> {
    fn from(res: std::result::Result<T, E>) -> Self {
        Self { result: res }
    }
}
