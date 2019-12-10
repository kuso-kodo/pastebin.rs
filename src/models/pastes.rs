use super::uuid::PasteID;
use crate::schema::pastes;
use crate::ConnPool;
use chrono;
use chrono::SubsecRound;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{AsChangeset, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, AsChangeset, Identifiable, Queryable, Insertable)]
#[table_name = "pastes"]
pub struct Paste {
    id: PasteID,
    title: Option<String>,
    lang: String,
    content: String,
    author_name: String,
    created_at: chrono::NaiveDateTime,
}

impl Paste {
    /// Construct a new paste.
    pub fn new(
        id: PasteID,
        title: Option<String>,
        lang: String,
        content: String,
        author_name: String,
    ) -> Self {
        Paste {
            id,
            title,
            lang,
            content,
            author_name,
            created_at: chrono::Local::now().round_subsecs(0).naive_local(),
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

    pub fn author_name(&self) -> &str {
        &self.author_name
    }

    /// Get a paste instance by its ID.
    pub async fn get_paste_by_id(paste_id: PasteID, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::pastes::dsl::*;
        pool.run(move |conn| pastes.filter(id.eq(&paste_id)).first(&conn))
            .await
    }

    /// Get all pastes by the user iD.
    pub async fn get_paste_list_by_user_name(
        p_name: String,
        pool: &ConnPool,
    ) -> Result<Vec<Self>, Error> {
        use crate::schema::pastes::dsl::*;
        pool.run(move |conn| {
            pastes
                .filter(author_name.eq(&p_name))
                .order(created_at.desc())
                .load(&conn)
        })
        .await
    }

    /// Insert our paste into the database.
    pub async fn insert(self, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::pastes::dsl::*;
        pool.run(move |conn| diesel::insert_into(pastes).values(&self).get_result(&conn))
            .await
    }
}
