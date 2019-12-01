use diesel::{Insertable, Queryable, Identifiable};
use serde::{Serialize};
use crate::schema::api_tokens;
use super::uuid::UserID;
use super::uuid::APITokenID;
use super::users::User;

#[derive(Debug, Serialize, Identifiable, Queryable, Associations, Insertable)]
#[table_name = "api_tokens"]
#[primary_key(token)]
#[belongs_to(User)]
pub struct APIToken {
    token: APITokenID,
    user_id: UserID
}

#[derive(Insertable)]
#[table_name = "api_tokens"]
pub struct NewApiToken {
    token: APITokenID,
    user_id: UserID
}

impl NewApiToken {
  pub fn new(token: APITokenID, user_id: UserID) -> Self {
    NewApiToken { token, user_id }
  }
}