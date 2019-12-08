#![feature(try_trait)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

mod api;
mod models;
mod schema;
mod utils;
mod web;

use async_std;
use diesel::pg::PgConnection;
use std::env;
use tide_naive_static_files::serve_static_files;
use utils::ConnectionPool;

type ConnPool = ConnectionPool<PgConnection>;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = ConnectionPool::new(&database_url);
    let mut app = tide::with_state(pool);
    app.at("/static/*")
        .get(|req| async { serve_static_files(req).await.unwrap() });
    app.at("/register").post(crate::api::user::register);
    app.at("/login").post(crate::api::user::login);
    app.at("/logout").post(crate::api::token::logout);
    app.at("/get/:id").get(crate::web::paste::get);
    app.at("/new").post(crate::api::paste::new);
    app.at("/:username/list").get(crate::api::paste::list);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
