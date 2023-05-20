use crate::{pool::pool, Error, __feature__};


pub enum TransactionResult {
    Commit,
    Rollback,
}

pub struct X(
    pub(crate) sqlx::Transaction<'static, __feature__::DB>
);
impl X {
    pub fn commit(&mut self) -> Result<TransactionResult, Error> {
        Ok(TransactionResult::Commit)
    }
    pub fn rollback(&mut self) -> Result<TransactionResult, Error> {
        Ok(TransactionResult::Rollback)
    }
}
impl X {
    pub(crate) async fn new() -> Result<Self, Error> {
        Ok(Self(pool().begin().await?))
    }
}
