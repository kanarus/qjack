/*===== compiler features =====*/
#![feature(unboxed_closures, fn_traits, trait_alias)]


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

    // #[cfg(feature="db_postgres")]
    // pub(crate) use sqlx::postgres::PgArguments as Params;
    // #[cfg(feature="db_mysql")]
    // pub(crate) use sqlx::mysql::MySqlArguments as Params;
    // #[cfg(feature="db_sqlite")]
    // pub(crate) use sqlx::sqlite::SqliteArguments as Params;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgArgumentBuffer as ParamsBuffer;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlArgumentBuffer as ParamsBuffer;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteArgumentBuffer as ParamsBuffer;

    #[cfg(feature="db_postgres")]
    pub(crate) use sqlx::postgres::PgTypeInfo as TypeInfo;
    #[cfg(feature="db_mysql")]
    pub(crate) use sqlx::mysql::MySqlTypeInfo as TypeInfo;
    #[cfg(feature="db_sqlite")]
    pub(crate) use sqlx::sqlite::SqliteTypeInfo as TypeInfo;
}


/*===== modules =====*/
mod q;
mod pool;
mod params;


/*===== visibility::pub =====*/
pub use pool::spawn;


/*===== visibility::in-crate =====*/
pub(crate) use pool::pool;


/*===== reexports =====*/
pub use sqlx::{FromRow, Error};
