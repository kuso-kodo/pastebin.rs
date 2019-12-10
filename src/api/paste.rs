use crate::models::pastes::*;
use crate::models::users::User;
use crate::models::uuid::*;
use crate::utils::new_api_result;
use crate::utils::APIResponse::*;
use crate::utils::APIResponse;
use crate::utils::Result;
use crate::ConnPool;
use serde::{Deserialize, Serialize};
use tide::*;
use uuid::Uuid;
use http_service::Body;
use super::DOMAIN;
use super::PASTE_DIR;
#[derive(Serialize, Deserialize)]
pub struct PasteRequest {
    paste: PasteID,
}

#[derive(Serialize, Deserialize)]
pub struct NewPaste {
    title: Option<String>,
    content: String,
    lang: String,
    author_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PasteList {
    author: String,
    list: Vec<Paste>,
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
    let username: String = req.param("username")?;
    let paste_list = PasteList {
        author: username,
        list: pastes,
    };
    Valid(new_api_result(&paste_list))
}

pub async fn new(mut req: Request<ConnPool>) -> Result {
    let request: NewPaste = req.body_json().await?;
    let pool = req.state();
    let paste = request.into().insert(&pool).await?;
    Valid(new_api_result(&paste))
}

pub struct PngFile{
    data: Vec<u8>
}

impl IntoResponse for PngFile {
    /// Convert a `APIResponse` type into `tide::Response`.
    fn into_response(self) -> Response {
        Response::new(200).set_header("Content-Type", "image/png").body(Body::from(self.data))
    }
}

pub async fn get_qrcode(req: Request<ConnPool>) -> APIResponse<PngFile, crate::utils::error::Error> {
    extern crate qrcode_generator;
    use qrcode_generator::QrCodeEcc;
    let mut s: String = req.param("id")?;
    let length = s.len();
    s.truncate(length - 4);
    let s = format!("{}{}{}", *DOMAIN, *PASTE_DIR,s);
    let data: Vec<u8> = qrcode_generator::to_png_to_vec(s, QrCodeEcc::Low, 1024).unwrap();
    Valid(PngFile { data })
}