pub struct X(
    pub(crate) sqlx::Transaction<'static, crate::__feature__::DB>
); impl X {
    pub fn commit(&mut self) -> Result<TransactionResult, crate::Error> {
        Ok(TransactionResult::Commit)
    }
    pub fn rollback(&mut self) -> Result<TransactionResult, crate::Error> {
        Ok(TransactionResult::Rollback)
    }
}

pub enum TransactionResult {
    Commit,
    Rollback,
}


// #[cfg(test)]
// mod __ {
//     struct T(Transaction);
//     impl<'q> FnOnce<(&'q str,)> for T {
//         type Output = Pin<Box<dyn 'q + Future<Output = Result<__feature__::QueryResult, Error>>>>;
//         extern "rust-call" fn call_once(mut self, (sql,): (&'q str,)) -> Self::Output {
//             Box::pin(async move {self.0.execute(sql).await})
//         }
//     }
//     impl<'q> FnMut<(&'q str,)> for T {
//         extern "rust-call" fn call_mut(&mut self, (sql,): (&'q str,)) -> Self::Output {
//             unsafe {std::mem::transmute(
//                 self.0.execute(sql)
//             )}
//         }
//     }
// }
// 