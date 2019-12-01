use crate::ConnPool;
use crate::models::users::NewUser;
use tide::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Error {
    error_info: String
}

impl Error {
    fn new(error_info: &str) -> Self {
        Error {
            error_info: error_info.to_string()
        }
    }
}

pub async fn register(mut req: Request<ConnPool>) -> Response {
    let new_user: NewUser = req.body_json().await.unwrap();
    let pool = req.state();
    match new_user.insert(&pool).await {
        Ok(result) => Response::new(200).body_json(&result).unwrap(),
        Err(_) => Response::new(409).body_json(&Error::new("Register failed.")).unwrap()
    }
}