use crate::api::*;
use crate::utils::APIResponse::*;
use crate::ConnPool;
use tide::*;

use super::HANDLEBARS;

pub async fn get(req: Request<ConnPool>) -> Response {
    match paste::get(req).await {
        Valid(v) => {
            let data = v.data;
            let res = Response::new(200).body_string(HANDLEBARS.render("paste", &data).unwrap());
            res.set_header("Content-Type", "HTML")
        }
        Invalid(_) => Response::new(400).body_string("Error".to_string()),
    }
}
