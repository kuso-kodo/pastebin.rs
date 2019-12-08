use crate::models::pastes::*;
use crate::models::users::User;
use crate::models::uuid::*;
use crate::utils::new_api_result;
use crate::utils::APIResponse::*;
use crate::utils::Result;
use crate::ConnPool;
use serde::{Deserialize, Serialize};
use tide::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PasteRequest {
    paste: PasteID,
}

#[derive(Serialize, Deserialize)]
pub struct NewPaste {
    title: Option<String>,
    content: String,
    lang: i32,
    author_name: Option<String>,
}

impl NewPaste {
    fn into(self) -> Paste {
        Paste::new(
            PasteID(Uuid::new_v4()),
            self.title,
            self.lang,
            self.content,
            self.author_name.unwrap_or_else(|| "Anonymous".to_string()),
        )
    }
}

pub async fn get(req: Request<ConnPool>) -> Result {
    let id: String = req.param("id")?;
    let pool = req.state();
    let paste = Paste::get_paste_by_id(PasteID(Uuid::parse_str(&id)?), &pool).await?;
    Valid(new_api_result(&paste))
}

pub async fn list(req: Request<ConnPool>) -> Result {
    let username: String = req.param("username")?;
    let pool = req.state();
    let user_id = User::get_user(username, &pool).await?;
    let pastes = Paste::get_paste_list_by_user_name(user_id.username().to_string(), &pool).await?;
    Valid(new_api_result(&pastes))
}

pub async fn new(mut req: Request<ConnPool>) -> Result {
    let request: NewPaste = req.body_json().await?;
    let pool = req.state();
    let paste = request.into().insert(&pool).await?;
    Valid(new_api_result(&paste))
}
