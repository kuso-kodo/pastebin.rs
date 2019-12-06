use handlebars::Handlebars;

use crate::api::*;
use crate::utils::APIResponse::*;
use crate::ConnPool;
use tide::*;

lazy_static! {
    static ref HANDLEBARS: Handlebars = {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("paste", "src/web/template.hbs")
            .unwrap();
        handlebars
    };
}

pub async fn get(req: Request<ConnPool>) -> Response {
    match paste::get(req).await {
        Valid(v) => {
            let res = Response::new(200).body_string(HANDLEBARS.render("paste", &v.data).unwrap());
            res.set_header("Content-Type", "HTML")
        }
        Invalid(_) => Response::new(400).body_string("Error".to_string()),
    }
}
