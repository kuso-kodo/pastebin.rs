use crate::api::*;
use crate::utils::APIResponse::*;
use crate::ConnPool;
use serde_json::*;
use tide::*;

use super::HANDLEBARS;

/// Show the result with given id.
pub async fn get(req: Request<ConnPool>) -> Response {
    match paste::get(req).await {
        Valid(v) => {
            let data = v.data;
            let res = Response::new(200).body_string(HANDLEBARS.render("show", &data).unwrap());
            res.set_header("Content-Type", "HTML")
        }
        Invalid(_) => Response::new(400).body_string("Error".to_string()),
    }
}

/// New paste page.
pub async fn new(_req: Request<ConnPool>) -> Response {
    let res =
        Response::new(200).body_string(HANDLEBARS.render("new", &json!({"name": "foo"})).unwrap());
    res.set_header("Content-Type", "HTML")
}

/// Return all pastes created by a user.
pub async fn list(req: Request<ConnPool>) -> Response {
    match paste::list(req).await {
        Valid(v) => {
            let data = v.data;
            let res = Response::new(200).body_string(HANDLEBARS.render("list", &data).unwrap());
            res.set_header("Content-Type", "HTML")
        }
        Invalid(_) => Response::new(400).body_string("Error".to_string()),
    }
}
