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




// #[cfg(test)]
// async fn __() -> Result<(), Error> {
//     q.transaction(|mut x| async {
//         let x2 = &mut x;
//         x2("").await?;
// 
//         Ok(())
//     }).await?;
// 
//     Ok(())
// }
// 
// #[cfg(test)]
// async fn __(x: &'static mut X) -> Result<(), Error> {
//     x("").await?;
//     Ok(())
// }
// 
// #[cfg(test)]
// async fn __(x: &'static X) -> Result<(), Error> {
//     // x("").await?;
//     let _ = x();
//     Ok(())
// }
