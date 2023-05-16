#![allow(non_snake_case)]
use std::{sync::OnceLock, future::{Future, IntoFuture}, task::Poll, pin::pin};
use crate::__feature__;

pub static CONNECTION_POOL: OnceLock<__feature__::ConnectionPool> = OnceLock::new();

#[derive(Clone)]
pub struct Config {
    DB_URL: String,
    max_connections: Option<u32>,
} impl IntoFuture for Config {
    type Output = Result<(), crate::Error>;
    type IntoFuture = ConnectingFuture;
    fn into_future(self) -> Self::IntoFuture {
        ConnectingFuture({
            let (db_url, pool_config) = self.into_pool_config();
            let connecting_future = pool_config.connect(db_url);

            // ===
            println!("called `into_future`");
            // ===

            Box::new(connecting_future)
        })
    }
} impl Config {
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections.replace(max_connections);
        self
    }
} impl Config {
    pub(crate) fn new(DB_URL: &str) -> Self {
        Self { DB_URL: DB_URL.to_string(),
            max_connections: None,
        }
    }
    fn into_pool_config(self) -> (/*DB_URL*/&'static str, __feature__::PoolConfig) {
        let mut config = __feature__::PoolConfig::new();
        if let Some(max_connections) = self.max_connections {
            config = config.max_connections(max_connections)
        }

        let db_url = Box::leak(Box::new(self.DB_URL).into_boxed_str());

        (db_url, config)
    }
}

pub struct ConnectingFuture(Box<dyn Future<Output = Result<__feature__::ConnectionPool, crate::Error>>>);
impl Future for ConnectingFuture {
    type Output = Result<(), crate::Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match unsafe{self.map_unchecked_mut(|cf| cf.0.as_mut())}.poll(cx) {
            Poll::Pending => {println!("Pending"); Poll::Pending},
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Ready(Ok(pool)) => Poll::Ready(CONNECTION_POOL.set(pool).map_err(|_| crate::Error::WorkerCrashed)),
        }
    }
}

pub(crate) fn pool<'p>() -> &'p crate::__feature__::ConnectionPool {
    CONNECTION_POOL.get()
        .expect("Failed to use connection pool")
}
