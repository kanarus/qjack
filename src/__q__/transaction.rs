use crate::{__feature__, Error, pool};


pub struct X(pub(crate) sqlx::Transaction<'static, __feature__::DB>);
impl X {
    pub(crate) async fn new() -> Result<Self, Error> {
        Ok(Self(pool().begin().await?))
    }
}
impl X {
    pub async fn commit(self) -> Result<(), Error> {
        self.0.commit().await
    }
    pub async fn rollback(self) -> Result<(), Error> {
        self.0.rollback().await
    }
}
