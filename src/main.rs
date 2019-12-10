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
    // Static files
    app.at("/static/*")
        .get(|req| async { serve_static_files(req).await.unwrap() });
    // APIs
    app.at("/api/register").post(crate::api::user::register);
    app.at("/api/login").post(crate::api::user::login);
    app.at("/api/logout").post(crate::api::token::logout);
    app.at("/api/new").post(crate::api::paste::new);
    app.at("/api/user/:username").get(crate::api::paste::list);
    app.at("/api/paste/:id").get(crate::web::paste::get);
    // Web Pages
    app.at("/").get(crate::web::paste::new);
    app.at("/user/:username").get(crate::web::paste::list);
    app.at("/paste/:id").get(crate::web::paste::get);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
