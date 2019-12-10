#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use super::ConnectionPool;
#[allow(unused_imports)]
use crate::ConnPool;

#[cfg(test)]
pub fn new_conn_pool() -> ConnPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    ConnectionPool::new(&database_url)
}