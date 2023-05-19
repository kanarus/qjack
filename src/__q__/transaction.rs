use crate::{__feature__, Error, pool, q};
type Transaction = sqlx::Transaction<'static, __feature__::DB>;


// pub struct X(
//     pub(crate) sqlx::Transaction<'static, __feature__::DB>
// );

// #[allow(non_camel_case_types)]
// pub struct qX<'x>(
//     &'x mut X
// );
// impl<'x> std::ops::Index<&'x mut X> for q {
//     type Output = &'x mut X;
//     fn index(&self, index: &'x mut X) -> &Self::Output {
//         todo!()        
//     }
// }
// impl<'x> std::ops::IndexMut<&'x mut X> for q {
//     fn index_mut(&mut self, index: &'x mut X) -> &mut Self::Output {
//         
//     }
// }

pub enum TransactionResult {
    Commit,
    Rollback,
}

// 実際には tokio / async-std の Arc, Mutex を使う
use std::sync::{Arc, Mutex};

pub struct X(
    pub(crate) Arc<Mutex<Transaction>>,
);

#[cfg(test)]
mod __ {
    use std::{future::Future, sync::{Mutex, Arc}};
    use crate::{Error};
    use super::{TransactionResult, TransactionResult::*, X, Transaction};

    async fn __<
        F:   FnOnce(&mut Mutex<Transaction>) -> Fut,
        Fut: Future<Output = Result<TransactionResult, Error>>,
    >(tx: Transaction, f: F) -> Result<(), Error> {
        let mut mutex = Mutex::new(tx);
        let transaction_result = f(&mut mutex).await?;
        // Arc::into_inner() // -> Option<T>

        let transaction = mutex.into_inner().unwrap();
        match transaction_result {
            Commit   => transaction.commit().await?,
            Rollback => transaction.rollback().await?,
        }

        Ok(())
    }
}


// #[cfg(test)]
// mod __ {
//     use super::X;
//     use crate::{Error, __feature__};
//     use sqlx::Executor;
//     use std::{pin::Pin, future::Future};
// 
//     const _: () = {
//         impl FnOnce<(&'static str,)> for X {
//             type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>>>>;
//             extern "rust-call" fn call_once(self, (sql,): (&'static str,)) -> Self::Output {
//                 // let query = sqlx::query(sql);
//                 // let mut tx    = self.0;
//                 // let fut   = tx.execute(query);
//                 // fut
//                 todo!()
//             }
//         }
//         impl FnMut<(&'static str,)> for X {
//             extern "rust-call" fn call_mut(&mut self, (sql,): (&'static str,)) -> Self::Output {
//                 self.0.execute(sql)
//             }
//         }
//     };
// }

// #[cfg(test)]
// mod __ {
//     use super::X;
// 
//     const _: () = {
//         impl<'x> FnOnce<()> for X<'x> {
//             type Output = i32;
//             extern "rust-call" fn call_once(self, _: ()) -> Self::Output {
//                 42
//             }
//         }
//         impl<'x> FnMut<()> for X<'x> {
//             extern "rust-call" fn call_mut(&mut self, _: ()) -> Self::Output {
//                 42
//             }
//         }
//         impl<'x> Fn<()> for X<'x> {
//             extern "rust-call" fn call(&self, _: ()) -> Self::Output {
//                 42
//             }
//         }
//     };
// 
//     fn __(x: X) {
//         let _ = x();
//         let _ = x();
//         let _ = x();
//     }
// }
// 
// 