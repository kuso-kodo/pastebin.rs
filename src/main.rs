#[macro_use]
extern crate diesel;

mod utils;
mod models;
mod schema;
mod api;

use async_std;
use diesel::pg::PgConnection;
use std::env;
use utils::db::ConnectionPool;

type ConnPool = ConnectionPool<PgConnection>;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
    let pool = ConnectionPool::new(&database_url);
    let mut app = tide::with_state(pool);

    app.at("/register").post(crate::api::user::register);
    app.at("/login").post(crate::api::user::login);
    app.at("/logout").post(crate::api::token::logout);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}