#![allow(non_snake_case)]
use std::{sync::OnceLock, future::Future, task::Poll, pin::pin};
use crate::__feature__;

pub static CONNECTION_POOL: OnceLock<__feature__::ConnectionPool> = OnceLock::new();

pub struct Config {
    DB_URL: String,
    max_connections: Option<u32>,
} impl Config {
    fn clone(&self) -> Self {
        Self { DB_URL: self.DB_URL.clone(),
            max_connections: self.max_connections.clone(),
        }
    }
} impl Config {
    pub(super) fn new(DB_URL: &str) -> Self {
        Self { DB_URL: DB_URL.to_string(),
            max_connections: None,
        }
    }
    fn into_sqlx_config(self) -> (/*DB URL*/String, __feature__::PoolConfig) {
        let mut config = __feature__::PoolConfig::new();
        if let Some(max_connections) = self.max_connections {
            config = config.max_connections(max_connections)
        }
        (self.DB_URL, config)
    }
} impl Config {
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections.replace(max_connections);
        self
    }
} impl Future for Config {
    type Output = Result<(), crate::Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let (db_url, config) = self.clone().into_sqlx_config();
        let connection_future = pin!(config.connect(&db_url));
        match connection_future.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Ready(Ok(pool)) => Poll::Ready(CONNECTION_POOL.set(pool).map_err(|_| crate::Error::WorkerCrashed)),
        }
    }
}

pub(crate) fn pool<'p>() -> &'p crate::__feature__::ConnectionPool {
    CONNECTION_POOL.get()
        .expect("Failed to use connection pool")
}