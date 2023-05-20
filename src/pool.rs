#![allow(non_snake_case)]
use std::{sync::OnceLock, future::{Future, IntoFuture}, task::Poll, time::Duration};
use crate::__feature__;


pub static CONNECTION_POOL: OnceLock<__feature__::ConnectionPool> = OnceLock::new();
pub(crate) fn pool<'p>() -> &'p crate::__feature__::ConnectionPool {
    CONNECTION_POOL.get()
        .expect("Failed to use connection pool")
}

#[derive(Clone)]
pub struct Config {DB_URL: String,
    test_before_acquire: Option<bool>,
    max_connections:     Option<u32>,
    min_connections:     Option<u32>,
    acquire_timeout:     Option<Duration>,
    max_lifetime:        Option<Duration>,
    idle_timeout:        Option<Duration>,
} impl IntoFuture for Config {
    type Output = Result<(), crate::Error>;
    type IntoFuture = ConnectingFuture;
    fn into_future(self) -> Self::IntoFuture {
        ConnectingFuture({
            let (db_url, pool_config) = self.into_pool_config();
            let connecting_future = pool_config.connect(db_url);
            Box::new(connecting_future)
        })
    }
}

pub struct ConnectingFuture(Box<dyn Future<Output = Result<__feature__::ConnectionPool, crate::Error>>>);
impl Future for ConnectingFuture {
    type Output = Result<(), crate::Error>;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        match unsafe{self.map_unchecked_mut(|cf| cf.0.as_mut())}.poll(cx) {
            Poll::Pending         => Poll::Pending,
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Ready(Ok(pool)) => Poll::Ready(CONNECTION_POOL.set(pool).map_err(|_| crate::Error::WorkerCrashed)),
        }
    }
}

impl Config {
    pub(crate) fn new(DB_URL: String) -> Self {
        Self { DB_URL,
            test_before_acquire: None,
            max_connections:     None,
            min_connections:     None,
            acquire_timeout:     None,
            idle_timeout:        None,
            max_lifetime:        None,
        }
    }
    fn into_pool_config(self) -> (/*DB_URL*/&'static str, __feature__::PoolConfig) {
        let db_url = Box::leak(Box::new(self.DB_URL).into_boxed_str());

        let mut config = __feature__::PoolConfig::new();
        if let Some(test_before_aquire) = self.test_before_acquire {
            config = config.test_before_acquire(test_before_aquire)
        }
        if let Some(max_connections) = self.max_connections {
            config = config.max_connections(max_connections)
        }
        if let Some(min_connections) = self.min_connections {
            config = config.min_connections(min_connections)
        }
        if let Some(acquire_timeout) = self.acquire_timeout {
            config = config.acquire_timeout(acquire_timeout)
        }
        config = config
            .idle_timeout(self.idle_timeout)
            .max_lifetime(self.max_lifetime);

        (db_url, config)
    }
}
impl Config {
    pub fn test_before_acquire(mut self, test: bool) -> Self {
        self.test_before_acquire.replace(test);
        self
    }
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections.replace(max);
        self
    }
    pub fn min_connections(mut self, min: u32) -> Self {
        self.min_connections.replace(min);
        self
    }
    pub fn acquire_timeout(mut self, timeout: Duration) -> Self {
        self.acquire_timeout.replace(timeout);
        self
    }
    pub fn idle_timeout(mut self, timeout: impl Into<Option<Duration>>) -> Self {
        self.idle_timeout = timeout.into();
        self
    }
    pub fn max_lifetime(mut self, lifetime: impl Into<Option<Duration>>) -> Self {
        self.max_lifetime = lifetime.into();
        self
    }
}


/*
    pub(crate) after_connect: Option<
        Arc<
            dyn Fn(&mut DB::Connection, PoolConnectionMetadata) -> BoxFuture<'_, Result<(), Error>>
                + 'static
                + Send
                + Sync,
        >,
    >,
    pub(crate) before_acquire: Option<
        Arc<
            dyn Fn(
                    &mut DB::Connection,
                    PoolConnectionMetadata,
                ) -> BoxFuture<'_, Result<bool, Error>>
                + 'static
                + Send
                + Sync,
        >,
    >,
    pub(crate) after_release: Option<
        Arc<
            dyn Fn(
                    &mut DB::Connection,
                    PoolConnectionMetadata,
                ) -> BoxFuture<'_, Result<bool, Error>>
                + 'static
                + Send
                + Sync,
        >,
    >,
    pub(crate) parent_pool: Option<Pool<DB>>,
*/
