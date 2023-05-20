use crate::{pool::pool, Error, __feature__};


pub enum TransactionResult {
    Commit,
    Rollback,
}

pub struct X(
    pub(crate) sqlx::Transaction<'static, __feature__::DB>
);
impl X {
    #[inline(always)] pub async fn commit(self) -> Result<TransactionResult, Error> {
        self.0.commit().await?;
        Ok(TransactionResult::Commit)
    }
    #[inline(always)] pub async fn rollback(self) -> Result<TransactionResult, Error> {
        self.0.rollback().await?;
        Ok(TransactionResult::Rollback)
    }
}
impl X {
    #[inline(always)] pub(crate) async fn new() -> Result<Self, Error> {
        Ok(Self(pool().begin().await?))
    }
}
