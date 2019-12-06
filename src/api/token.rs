use crate::models::api_tokens::*;
use crate::models::uuid::*;
use crate::utils::APIResponse::*;
use crate::utils::Result;
use crate::ConnPool;
use serde::{Deserialize, Serialize};
use tide::*;
use crate::utils::new_api_result;

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: APITokenID,
}

pub async fn logout(mut req: Request<ConnPool>) -> Result {
    let token: Token = req.body_json().await?;
    let pool = req.state();
    let token = APIToken::get_token(token.token, &pool).await?;
    let result = token.delete(&pool).await?;
    Valid(new_api_result(&result))
}
