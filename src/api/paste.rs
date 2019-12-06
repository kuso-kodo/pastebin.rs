use crate::models::pastes::*;
use crate::models::users::User;
use crate::models::uuid::*;
use crate::utils::response::from_json;
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
    author_id: Option<UserID>,
}

impl NewPaste {
    fn into(self) -> Paste {
        Paste::new(
            PasteID(Uuid::new_v4()),
            self.title,
            self.lang,
            self.content,
            self.author_id.unwrap_or_else(|| UserID(Uuid::nil())),
        )
    }
}

pub async fn get(mut req: Request<ConnPool>) -> Result {
    let request: PasteRequest = req.body_json().await?;
    let pool = req.state();
    let paste = Paste::get_paste_by_id(request.paste, &pool).await?;
    Valid(from_json(&paste))
}

pub async fn list(req: Request<ConnPool>) -> Result {
    let username: String = req.param("username")?;
    let pool = req.state();
    let user_id = User::get_user(username, &pool).await?;
    let pastes = Paste::get_paste_list_by_user_id(user_id.id(), &pool).await?;
    Valid(from_json(&pastes))
}

pub async fn new(mut req: Request<ConnPool>) -> Result {
    let request: NewPaste = req.body_json().await?;
    let pool = req.state();
    let paste = request.into().insert(&pool).await?;
    Valid(from_json(&paste))
}
