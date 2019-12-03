use crate::models::api_tokens::*;
use crate::models::uuid::*;
use crate::utils::response::from_json;
use crate::utils::Error;
use crate::utils::RResponse::*;
use crate::utils::Result;
use crate::ConnPool;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::*;

#[derive(Serialize, Deserialize)]
struct Token {
    token: APITokenID,
}

pub async fn logout(mut req: Request<ConnPool>) -> Result {
    let token: Token = req.body_json().await?;
    let pool = req.state();
    let token = APIToken::get_token(token.token, &pool).await?;
    let result = token.delete(&pool).await?;
    Valid(from_json(&result))
}
