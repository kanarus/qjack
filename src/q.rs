use crate::{
    Error,
    pool::pool,
    query::{IntoQueryParams, IntoQueryAsParams, FromRow},
};

#[allow(non_upper_case_globals)]
pub const q: Q = Q;

pub struct Q;
impl Q {
    pub async fn jack<'q>(&self, sql: &'q str, params: impl IntoQueryParams<'q>) -> Result<(), Error> {
        params.binded(sqlx::query(sql)).execute(pool()).await.map(|_| ())
    }
}
impl Q {
    pub async fn optional<'q, Model:FromRow>(&self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Option<Model>, Error> {
        params.binded(sqlx::query_as(sql)).fetch_optional(pool()).await
    }
    pub async fn one<'q, Model:FromRow>(&self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Model, Error> {
        params.binded(sqlx::query_as(sql)).fetch_one(pool()).await
    }
    pub async fn all<'q, Model:FromRow>(&self, sql: &'q str, params: impl IntoQueryAsParams<'q, Model>) -> Result<Vec<Model>, Error> {
        params.binded(sqlx::query_as(sql)).fetch_all(pool()).await
    }
}
