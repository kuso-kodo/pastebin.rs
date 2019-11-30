use crate::ConnPool;
use crate::models::users::NewUser;
use tide::*;

pub async fn register(mut req: Request<ConnPool>) -> Response {
    let new_user: NewUser = req.body_json().await.unwrap();
    let pool = req.state();
    match new_user.insert(&pool).await {
        Ok(result) => Response::new(200).body_json(&result).unwrap(),
        Err(_) => Response::new(500)
    }
}