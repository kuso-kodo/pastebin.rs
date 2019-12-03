use super::users::User;
use super::uuid::APITokenID;
use super::uuid::UserID;
use crate::schema::api_tokens;
use diesel::result::Error;
use diesel::{Identifiable, Insertable, Queryable};
use serde::Serialize;

use crate::ConnPool;
use diesel::prelude::*;

#[derive(Debug, Serialize, Identifiable, Queryable, Associations, Insertable)]
#[table_name = "api_tokens"]
#[primary_key(token)]
#[belongs_to(User)]
pub struct APIToken {
    token: APITokenID,
    user_id: UserID,
}

impl APIToken {
    pub async fn get_token(id: APITokenID, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::api_tokens::dsl::*;
        pool.run(move |conn| api_tokens.filter(token.eq(&id)).first(&conn))
            .await
    }

    pub async fn check_token(id: APITokenID, pool: &ConnPool) -> bool {
        match Self::get_token(id, pool).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn delete(self, pool: &ConnPool) -> Result<usize, Error> {
        use crate::schema::api_tokens::dsl::*;
        pool.run(move |conn| diesel::delete(api_tokens.find(self.token)).execute(&conn))
            .await
    }
}

#[derive(Insertable)]
#[table_name = "api_tokens"]
pub struct NewApiToken {
    token: APITokenID,
    user_id: UserID,
}

impl NewApiToken {
    pub fn new(token: APITokenID, user_id: UserID) -> Self {
        NewApiToken { token, user_id }
    }

    pub async fn insert(self, pool: &ConnPool) -> Result<APIToken, Error> {
        use crate::schema::api_tokens::dsl::*;
        pool.run(move |conn| {
            diesel::insert_into(api_tokens)
                .values(&self)
                .get_result(&conn)
        })
        .await
    }
}
