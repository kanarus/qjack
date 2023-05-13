use sqlx::{Encode, Type};
use crate::__feature__;

pub trait Param<'q>: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}
impl<'q, T> Param<'q> for T where T: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}
