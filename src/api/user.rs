use crate::models::*;
use crate::utils::APIResponse::*;
use crate::utils::Error;
use crate::utils::Result;
use crate::utils::*;
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
        Ok(result) => Valid(new_api_result(&result)),
        Err(_) => Invalid(Error::from_http_status(StatusCode::CONFLICT)),
    }
}

pub async fn login(mut req: Request<ConnPool>) -> Result {
    let login_info: LoginInfo = req.body_json().await?;
    let pool = req.state();
    let result = User::get_user(login_info.username, &pool).await?;
    if result.password() == login_info.password {
        let token = result.new_token(&pool).await.unwrap();
        Valid(new_api_result(&token))
    } else {
        Invalid(Error::from_http_status(StatusCode::CONFLICT))
    }
}
