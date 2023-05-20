mod param;
mod str_query;
mod fetch_query;
mod transaction;

use std::{future::Future, pin::Pin};
use transaction::{X, TransactionResult};
use crate::{Error, pool::Config, __feature__};

pub(crate) type QueryOutput<'q> = Pin<Box<dyn 'q + Future<Output = Result<__feature__::QueryResult, Error>>>>;
pub(crate) type FetchQueryOutput<'q, Fetched> = Pin<Box<dyn 'q + Future<Output = Result<Fetched, Error>>>>;


#[allow(non_camel_case_types)]
pub struct q;

impl q {
    /// Create connection pool with given configuration.
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
    #[inline(always)] pub fn jack(self, DB_URL: impl ToString) -> Config {
        Config::new(DB_URL.to_string())
    }

    /// Perform a transaction. In current version, this is `unsafe` function for a technical reason.
    /// 
    /// <br/>
    /// 
    /// ```ignore
    /// #[Payload(JSON)]
    /// struct TransferRequest {
    ///     from: Account,
    ///     to:   Account,
    ///     ammount: u128,
    /// }
    /// 
    /// async fn transfer(c: Context, req: TransferRequest) -> Result<()> {
    ///     let TransferRequest {from, to, ammount} = req;
    /// 
    ///     unsafe {q.transaction(|mut x| async {
    ///         if let Err(e) = x("
    ///             UPDATE accounts
    ///             SET balance = balance - $1
    ///             WHERE id = $2 AND name = $3
    ///         ", ammount, from.id, from.name).await {
    ///             tracing::error!("Error in subtracting balance: {e}");
    ///             return x.rollback().await
    ///         }
    /// 
    ///         if let Err(e) = x("
    ///             UPDATE accounts
    ///             SET balance = balance + $1
    ///             WHERE id = $2 AND name = $3   
    ///         ", ammount, to.id, to.name).await {
    ///             tracing::error!("Error in add balance: {e}");
    ///             return x.rollback().await
    ///         }
    /// 
    ///         x.commit().await
    ///     })}.await
    /// }
    /// ```
    pub async unsafe fn transaction<'f,
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
