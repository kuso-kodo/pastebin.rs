use serde::Serialize;
use tide::Response;

/// Generate a response from a Serializable type.
pub fn from_json(result: &impl Serialize) -> Response {
    Response::new(200).body_json(result).unwrap()
}
