use super::uuid::{PasteID, UserID};
use crate::schema::pastes;
use diesel::result::Error;
use diesel::{AsChangeset, Identifiable, Queryable};
use serde::Serialize;

use crate::ConnPool;
use diesel::prelude::*;

#[derive(Debug, Serialize, AsChangeset, Identifiable, Queryable, Insertable)]
#[table_name = "pastes"]
pub struct Paste {
    id: PasteID,
    title: Option<String>,
    content: String,
    author_id: UserID,
}

impl Paste {
    pub fn new(id: PasteID, title: Option<String>, content: String, author_id: UserID) -> Self {
        Paste {
            id,
            title,
            content,
            author_id,
        }
    }

    pub fn id(&self) -> PasteID {
        self.id
    }

    pub fn title(&self) -> &str {
        match &self.title {
            Some(s) => s,
            None => "",
        }
    }

    pub fn set_title(&mut self, title: String) {
        if title == "".to_string() {
            self.title = Some(title);
        } else {
            self.title = None;
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content
    }

    pub fn author_id(&self) -> UserID {
        self.author_id
    }

    pub async fn get_paste_by_id(paste_id: PasteID, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::pastes::dsl::*;
        pool.run(move |conn| pastes.filter(id.eq(&paste_id)).first(&conn))
            .await
    }

    pub async fn get_paste_list_by_user_id(
        p_id: PasteID,
        pool: &ConnPool,
    ) -> Result<Vec<Self>, Error> {
        use crate::schema::pastes::dsl::*;
        pool.run(move |conn| pastes.filter(author_id.eq(&p_id)).load(&conn))
            .await
    }

    pub async fn insert(self, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::pastes::dsl::*;
        pool.run(move |conn| diesel::insert_into(pastes).values(&self).get_result(&conn))
            .await
    }
}
