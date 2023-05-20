mod param;
mod str_query;
mod fetch_query;
mod transaction;

use std::{future::Future, pin::Pin};
use crate::{Error, pool::Config, __feature__};
use transaction::X;

use self::transaction::TransactionResult;

pub(crate) type QueryOutput<'q> = Pin<Box<dyn 'q + Future<Output = Result<__feature__::QueryResult, Error>>>>;
pub(crate) type FetchQueryOutput<'q, Fetched> = Pin<Box<dyn 'q + Future<Output = Result<Fetched, Error>>>>;


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
    pub fn jack(self, DB_URL: impl ToString) -> Config {
        Config::new(DB_URL.to_string())
    }

    pub async fn transaction<'f, 'x,
        Proc: FnOnce(X) -> Fut,
        Fut: 'f + Future<Output = Result<TransactionResult, Error>>
    >(self, proc: Proc) -> Result<(), Error> {
        let x = X::new().await?;
        let _: TransactionResult = proc(x).await?;
        Ok(())
    }
}




#[cfg(test)]
mod __ {
    use crate::Error;
    use super::transaction::{X, TransactionResult};

    async fn __(mut x: X) -> Result<TransactionResult, Error> {
        x("").await?;
        x("").await?;

        x.commit().await
    }
}
