use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use r2d2::{Pool, PooledConnection};
use std::path::{Path, PathBuf};
use tide_naive_static_files::{serve_static_files, StaticRootDir};
/// Async connection pool based on
/// r2d2::Pool.
#[derive(Clone)]
pub struct ConnectionPool<T>
where
    T: Connection + 'static,
{
    connection_pool: Pool<ConnectionManager<T>>,
}

impl<T> StaticRootDir for ConnectionPool<T> 
where
    T: Connection + 'static, {
    fn root_dir(&self) -> &Path {
        &Path::new("./")
    }
}

#[allow(dead_code)]
impl<T> ConnectionPool<T>
where
    T: Connection + 'static,
{
    pub fn new(db_url: &str) -> Self {
        Self::from_pool_builder(db_url, r2d2::Builder::default())
    }

    pub fn from_pool_builder(db_url: &str, builder: r2d2::Builder<ConnectionManager<T>>) -> Self {
        let manager = ConnectionManager::new(db_url);
        let connection_pool = builder.build(manager).expect("can not init db pool");
        ConnectionPool { connection_pool }
    }

    pub async fn run<F, R>(&self, f: F) -> R
    where
        F: FnOnce(PooledConnection<ConnectionManager<T>>) -> R
            + Send
            + std::marker::Unpin
            + 'static,
        T: Send + 'static,
    {
        let pool = self.connection_pool.clone();
        let mut f = Some(f);
        (f.take().unwrap())(pool.get().unwrap())
    }
}
