use crate::models::pastes::*;
use crate::models::uuid::*;
use crate::utils::response::from_json;
use crate::utils::RResponse::*;
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
    author_id: Option<UserID>,
}

impl NewPaste {
    fn into(self) -> Paste {
        Paste::new(
            PasteID(Uuid::new_v4()),
            self.title,
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

pub async fn new(mut req: Request<ConnPool>) -> Result {
    let request: NewPaste = req.body_json().await?;
    let pool = req.state();
    let paste = request.into().insert(&pool).await?;
    Valid(from_json(&paste))
}
