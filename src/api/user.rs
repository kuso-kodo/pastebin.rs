use crate::models::users::*;
use crate::utils::Error;
use crate::ConnPool;
use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::*;

#[derive(Serialize, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

pub async fn register(mut req: Request<ConnPool>) -> Response {
    match req.body_json::<NewUser>().await {
        Ok(new_user) => {
            let pool = req.state();
            match new_user.insert(&pool).await {
                Ok(result) => Response::new(200).body_json(&result).unwrap(),
                Err(_) => Response::new(StatusCode::CONFLICT.as_u16())
                    .body_json(&Error::from_http_status(StatusCode::CONFLICT))
                    .unwrap(),
            }
        }
        Err(_) => Response::new(StatusCode::BAD_REQUEST.as_u16())
            .body_json(&Error::from_http_status(StatusCode::BAD_REQUEST))
            .unwrap(),
    }
}

pub async fn login(mut req: Request<ConnPool>) -> Response {
    match req.body_json::<LoginInfo>().await {
        Ok(login_info) => {
            let pool = req.state();
            match User::get_user(login_info.username, &pool).await {
                Ok(result) => {
                    if result.password() == login_info.password {
                        let token = result.new_token(&pool).await.unwrap();
                        Response::new(200).body_json(&token).unwrap()
                    } else {
                        Response::new(StatusCode::NOT_ACCEPTABLE.as_u16())
                            .body_json(&Error::from_http_status(StatusCode::CONFLICT))
                            .unwrap()
                    }
                }
                Err(_) => Response::new(StatusCode::CONFLICT.as_u16())
                    .body_json(&Error::from_http_status(StatusCode::CONFLICT))
                    .unwrap(),
            }
        }
        Err(_) => Response::new(StatusCode::BAD_REQUEST.as_u16())
            .body_json(&Error::from_http_status(StatusCode::BAD_REQUEST))
            .unwrap(),
    }
}
