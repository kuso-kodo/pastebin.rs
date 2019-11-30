use diesel::{Insertable, Queryable, AsChangeset, Identifiable};
use serde::{Serialize, Deserialize};
use crate::schema::users;
use super::uuid::UserID;
use diesel::result::Error;
use uuid::Uuid;

use crate::ConnPool;
use diesel::prelude::*;

#[derive(Debug, Serialize, AsChangeset, Identifiable, Queryable)]
pub struct User {
    id: UserID,
    username: String,
    #[serde(skip_serializing)]
    password: String
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
        pool.run(move |conn| {
            diesel::delete(users.find(self.id))
                .execute(&conn)
        })
        .await
    }
}

#[derive(Debug, Serialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct RealNewUser {
    id: UserID,
    username: String,
    password: String
}

impl RealNewUser {
    pub fn new(new_user: NewUser) -> Self {
        Self {
            id: UserID(Uuid::new_v4()),
            username: new_user.username, 
            password: new_user.password
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
    password: String
}

impl NewUser {
    pub fn new(
        username: String,
        password: String
    ) -> NewUser {
        NewUser {
            username,
            password
        }
    }

    pub async fn insert(self, pool: &ConnPool) -> Result<User, Error> {
        let real_user = RealNewUser::new(self);
        pool.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&real_user)
                .get_result(&conn)
        })
        .await
    }
}