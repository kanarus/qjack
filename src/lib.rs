/*===== compiler features =====*/
#![feature(unboxed_closures, fn_traits)]


/*===== crate feature managements =====*/
#[cfg(any(
    all(feature="rt_tokio", feature="rt_async-std"),
))] compile_error!("
    Can't activate multiple `rt_*` features
");

#[cfg(any(
    all(feature="db_postgres", feature="db_mysql"),
    all(feature="db_mysql", feature="db_sqlite"),
    all(feature="db_sqlite", feature="db_postgres"),
))] compile_error!("
    Can't activate multiple `db_*` features
");


/*===== feature-abstraction layer =====*/
mod __feature__ {
    #[cfg(feature="db_postgres")]
    pub use sqlx::postgres::PgRow as Row;
    #[cfg(feature="db_mysql")]
    pub use sqlx::mysql::MySqlRow as Row;
    #[cfg(feature="db_sqlite")]
    pub use sqlx::sqlite::SqliteRow as Row;

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
    pub use sqlx::Postgres as DB;
    #[cfg(feature="db_mysql")]
    pub use sqlx::MySql as DB;
    #[cfg(feature="db_sqlite")]
    pub use sqlx::Sqlite as DB;

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
mod __model__;
mod __q__;


/*===== visibility::pub =====*/
pub use __q__::{q};
pub use __model__::{model};


/*===== visibility::pub(crate) =====*/
pub(crate) use pool::{pool};
pub(crate) use __model__::{FetchAll, FetchOne, FetchOptional};


/*===== reexports =====*/
pub use qjack_macros::model;

pub use sqlx::Error;

pub mod __private__ {
    pub use crate::__feature__::{
        DB,
    };
    pub use ::sqlx::{
        FromRow, ColumnIndex, decode::Decode, types::Type, Row
    };
}


/*===== q =====*/
// use query::{IntoQueryParams, IntoQueryAsParams};
// use std::{future::Future, task::Poll, pin::Pin};
// 
// #[allow(non_camel_case_types)]
// pub struct q;
// 
// impl q {
//     pub async fn optional<'q, Model: query::FromRow>(self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Option<Model>, Error> {
//         params.binded(sqlx::query_as(sql)).fetch_optional(pool()).await
//     }
//     pub async fn one<'q, Model: query::FromRow>(self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Model, Error> {
//         params.binded(sqlx::query_as(sql)).fetch_one(pool()).await
//     }
//     pub async fn all<'q, Model: query::FromRow>(self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Vec<Model>, Error> {
//         params.binded(sqlx::query_as(sql)).fetch_all(pool()).await
//     }
// }
// 
// const _: () = {
//     impl<'q, Params:IntoQueryParams<'q>> FnOnce<(&'q str, Params)> for q {
//         type Output = Query<'q>;
//         extern "rust-call" fn call_once(self, (sql, params): (&'q str, Params)) -> Self::Output {
//             use sqlx::Executor;
//             Query( pool().execute(params.binded(sqlx::query(sql))) )
//         }
//     }
//     impl<'q> FnOnce<(&'q str,)> for q {
//         type Output = Query<'q>;
//         extern "rust-call" fn call_once(self, (sql,): (&'q str,)) -> Self::Output {
//             use sqlx::Executor;
//             Query( pool().execute(sqlx::query(sql)) )
//         }
//     }
// 
//     pub struct Query<'q>(
//         Pin<Box<dyn
//             Future<Output = Result<__feature__::QueryResult, Error>>
//             + Send
//             + 'q
//         >>
//     );
//     impl<'q> Future for Query<'q> {
//         type Output = Result<__feature__::QueryResult, Error>;
//         fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//             (&mut self
//                 .get_mut()
//                 .0)
//                 .as_mut()
//                 .poll(cx)
//         }
//     }
// };
// 