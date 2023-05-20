mod param;
mod str_query;
mod fetch_query;
mod transaction;

use std::{future::Future, pin::Pin};
use crate::{Error, pool::Config, __feature__};
use transaction::X;

pub(crate) type QueryOutput<'q> = Pin<Box<dyn 'q + Future<Output = Result<__feature__::QueryResult, Error>>>>;
pub(crate) type FetchQueryResult<'q, Fetched> = Pin<Box<dyn 'q + Future<Output = Result<Fetched, Error>>>>;


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

    pub async fn transaction<F: Future<Output = Result<(), Error>>>(
        self,
        f: fn(&mut X) -> F
    ) -> Result<(), Error> {
        /*
        
        let mut tx = pool().begin().await?;
        let transaction_result = f( X(&mut tx) ).await?;
        match transaction_result {
            Commit   => tx.commit(),
            Rollback => tx.rollback(),
        }.await?
        
        */

        todo!()
    }
}




#[cfg(test)]
mod __ {
    use crate::Error;
    use super::transaction::{X, TransactionResult};

    async fn __(x: &mut X) -> Result<TransactionResult, Error> {
        x("").await?;

        x.commit()
    }

}
