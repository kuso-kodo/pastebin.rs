use super::api_tokens::*;
use super::uuid::{APITokenID, UserID};
use crate::schema::users;
use diesel::result::Error;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ConnPool;
use diesel::prelude::*;

#[derive(Debug, Serialize, AsChangeset, Identifiable, Queryable)]
pub struct User {
    id: UserID,
    username: String,
    #[serde(skip_serializing)]
    password: String,
}

impl User {
    pub fn id(&self) -> UserID {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password
    }

    pub async fn get_user(name: String, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        pool.run(move |conn| users.filter(username.eq(&name)).first(&conn))
            .await
    }

    pub async fn update(self, pool: &ConnPool) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        pool.run(move |conn| {
            diesel::update(users.find(self.id))
                .set(&self)
                .get_result(&conn)
        })
        .await
    }

    pub async fn delete(self, pool: &ConnPool) -> Result<usize, Error> {
        use crate::schema::users::dsl::*;
        pool.run(move |conn| diesel::delete(users.find(self.id)).execute(&conn))
            .await
    }

    pub async fn new_token(self, pool: &ConnPool) -> Result<APIToken, Error> {
        use crate::schema::api_tokens::dsl::*;
        let new_token = NewApiToken::new(APITokenID(Uuid::new_v4()), self.id);
        pool.run(move |conn| {
            diesel::insert_into(api_tokens)
                .values(&new_token)
                .get_result(&conn)
        })
        .await
    }
}

#[derive(Debug, Serialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct RealNewUser {
    id: UserID,
    username: String,
    password: String,
}

impl RealNewUser {
    pub fn new(new_user: NewUser) -> Self {
        Self {
            id: UserID(Uuid::new_v4()),
            username: new_user.username,
            password: new_user.password,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String,
}

impl NewUser {
    pub fn new(username: String, password: String) -> NewUser {
        NewUser { username, password }
    }

    pub async fn insert(self, pool: &ConnPool) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let real_user = RealNewUser::new(self);
        pool.run(move |conn| {
            diesel::insert_into(users)
                .values(&real_user)
                .get_result(&conn)
        })
        .await
    }
}
