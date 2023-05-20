#![doc(html_root_url = "https://docs.rs/ohkami/0.1.2")]


/*===== compiler features =====*/
#![feature(unboxed_closures, fn_traits)]


/*===== crate feature managements =====*/
#[cfg(not(any(
    feature="rt_tokio",
    feature="rt_async-std",
)))] compile_error!("
    You have to activate a `rt_*` feture to select runtime
");
#[cfg(any(
    all(feature="rt_tokio", feature="rt_async-std"),
))] compile_error!("
    Can't activate multiple `rt_*` features
");

#[cfg(not(any(
    feature="db_postgres",
    feature="db_mysql",
    feature="db_sqlite",
)))] compile_error!("
    You have to activate a `db_*` feature to select database
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
    pub(crate) use sqlx::postgres::PgQueryResult as QueryResult;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlQueryResult as QueryResult;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteQueryResult as QueryResult;
}


/*===== modules =====*/
mod pool;
mod model;
mod __q__;


/*===== visibility::pub =====*/
pub use __q__::{q};
pub use model::{Model};


/*===== visibility::pub(crate) =====*/
pub(crate) use pool::{pool};
pub(crate) use model::{FetchAll, FetchOne, FetchOptional};


/*===== reexports =====*/
pub use qjack_macros::Model;

pub use sqlx::Error;

pub mod __private__ {
    pub use crate::__feature__::{
        DB,
    };
    pub use ::sqlx::{
        FromRow, ColumnIndex, decode::Decode, types::Type, Row
    };
}
