mod param;
mod str_query;
mod fetch_query;
mod transaction;

use std::future::Future;
use crate::{Error, pool::Config};
use transaction::X;


#[allow(non_camel_case_types)]
pub struct q;

impl q {
    /// Establish connection pool with given configuration.
    /// 
    /// **ALL** queries **MUST** be executed **AFTER** this
    /// 
    /// <br/>
    /// 
    /// ```ignore
    /// async fn main() -> Result<(), qjack::Error> {
    ///     q.jack("DB_URL")
    ///         .max_connections(42)
    ///         .await?;
    /// 
    ///     /* do something with DB */
    /// }
    /// 
    /// async fn some_proc() {
    ///     /* called AFTER `q.jack() 〜 .await?` */
    /// }
    /// ```
    #[allow(non_snake_case)]
    pub fn jack<'url>(self, DB_URL: &'url str) -> Config {
        Config::new(DB_URL)
    }

    pub async fn transaction<F: Future<Output = Result<(), Error>>>(
        self,
        f: fn(X) -> F
    ) -> Result<(), Error> {
        f(X::new().await?).await
    }
}
