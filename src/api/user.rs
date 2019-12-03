use crate::models::users::*;
use crate::utils::response::from_json;
use crate::utils::Error;
use crate::utils::RResponse::*;
use crate::utils::Result;
use crate::ConnPool;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::*;

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

pub async fn register(mut req: Request<ConnPool>) -> Result {
    let new_user: NewUser = req.body_json().await?;
    let pool = req.state();
    match new_user.insert(&pool).await {
        Ok(result) => Valid(from_json(&result)),
        Err(_) => Invalid(Error::from_http_status(StatusCode::CONFLICT)),
    }
}

pub async fn login(mut req: Request<ConnPool>) -> Result {
    let login_info: LoginInfo = req.body_json().await?;
    let pool = req.state();
    let result = User::get_user(login_info.username, &pool).await?;
    if result.password() == login_info.password {
        let token = result.new_token(&pool).await.unwrap();
        Valid(from_json(&token))
    } else {
        Invalid(Error::from_http_status(StatusCode::CONFLICT))
    }
}
