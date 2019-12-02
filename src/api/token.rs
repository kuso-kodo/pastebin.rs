use crate::ConnPool;
use tide::*;
use crate::models::uuid::*;
use crate::models::api_tokens::*;
use serde::{Serialize, Deserialize};
use http::status::StatusCode;
use crate::utils::error::Error;

#[derive(Serialize, Deserialize)]
struct Token {
    token: APITokenID
}

pub async fn logout(mut req: Request<ConnPool>) -> Response {
    match req.body_json::<Token>().await {
        Ok(token) => {
            let pool = req.state();
            let token = APIToken::get_token(token.token, &pool).await.unwrap();
            match token.delete(&pool).await {
                Ok(result) => Response::new(200).body_json(&result).unwrap(),
                Err(_) => Response::new(StatusCode::BAD_REQUEST.as_u16())
                                .body_json(&Error::new(StatusCode::BAD_REQUEST.canonical_reason().unwrap())).unwrap()
            }
        },
        Err(_) => Response::new(StatusCode::BAD_REQUEST.as_u16())
                            .body_json(&Error::new(StatusCode::BAD_REQUEST.canonical_reason().unwrap())).unwrap()
    }
}