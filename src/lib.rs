/*===== compiler features =====*/
#![feature(unboxed_closures, fn_traits)]


/*===== crate feature managements =====*/
#[cfg(any(
    all(feature="rt_tokio", feature="rt_async-std"),
))] compile_error!("
    Can't enable multiple `rt_*` features
");

#[cfg(any(
    all(feature="db_postgres", feature="db_mysql"),
    all(feature="db_mysql", feature="db_sqlite"),
    all(feature="db_sqlite", feature="db_postgres"),
))] compile_error!("
    Can't enable multiple `db_*` features
");


/*===== feature-abstraction layer =====*/
mod __feature__ {
    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgRow as Row;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlRow as Row;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteRow as Row;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::PgPool as ConnectionPool;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::MySqlPool as ConnectionPool;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::SqlitePool as ConnectionPool;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgPoolOptions as PoolConfig;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlPoolOptions as PoolConfig;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqlitePoolOptions as PoolConfig;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::Postgres as DB;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::MySql as DB;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::Sqlite as DB;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgArguments as Params;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlArguments as Params;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteArguments as Params;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgQueryResult as QueryResult;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlQueryResult as QueryResult;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteQueryResult as QueryResult;
}


/*===== modules =====*/
mod pool;
mod query;


/*===== visibility::pub =====*/
pub use pool::spawn;


/*===== visibility::pub(crate) =====*/
pub(crate) use pool::pool;


/*===== reexports =====*/
pub use sqlx::{FromRow, Error};


/*===== q =====*/
use query::{IntoQueryParams, IntoQueryAsParams};
use std::{future::Future, task::Poll, ops::DerefMut};

#[allow(non_camel_case_types)]
pub struct q;

impl q {
    pub async fn optional<'q, Model: query::FromRow>(self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Option<Model>, Error> {
        params.binded(sqlx::query_as(sql)).fetch_optional(pool()).await
    }
    pub async fn one<'q, Model: query::FromRow>(self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Model, Error> {
        params.binded(sqlx::query_as(sql)).fetch_one(pool()).await
    }
    pub async fn all<'q, Model: query::FromRow>(self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Vec<Model>, Error> {
        params.binded(sqlx::query_as(sql)).fetch_all(pool()).await
    }
}

const _: () = {
    pub struct Query<'q>(Box<dyn 'q + Future<Output = Result<__feature__::QueryResult, Error>>>);
    impl<'q, Params:IntoQueryParams<'q>> FnOnce<(&'q str, Params)> for q {
        type Output = Query<'q>;
        extern "rust-call" fn call_once(self, (sql, params): (&'q str, Params)) -> Self::Output {
            Query(Box::new(params.binded(sqlx::query(sql)).execute(pool())))
        }
    }
    impl<'q> Future for Query<'q> {
        type Output = Result<(), Error>;
        fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            unsafe {self.map_unchecked_mut(|this| this.0.deref_mut())}.poll(cx).map(|result| result.map(|_| ()))
        }
    }
};
