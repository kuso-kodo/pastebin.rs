use serde::Serialize;
use tide::Response;

pub fn from_json(result: &impl Serialize) -> Response {
    Response::new(200).body_json(result).unwrap()
}
