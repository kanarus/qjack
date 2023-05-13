#![allow(non_upper_case_globals)]
use std::marker::PhantomData;
use sqlx::FromRow;
use crate::__feature__;

pub struct FetchAll<'q, M: Model> {
    pub(crate) sql: &'q str, pub(crate) __as__:PhantomData<fn()->M>
} impl<'q, M: Model> FetchAll<'q, M> {
    pub(crate) fn new(sql: &'q str) -> Self {
        Self { sql, __as__:PhantomData }
    }
    pub(crate) fn sql(&self) -> &str {
        self.sql
    }
}
pub struct FetchOne<'q, M: Model> {
    pub(crate) sql: &'q str, pub(crate) __as__:PhantomData<fn()->M>,
} impl<'q, M: Model> FetchOne<'q, M> {
    pub(crate) fn new(sql: &'q str) -> Self {
        Self { sql, __as__:PhantomData }
    }
    pub(crate) fn sql(&self) -> &str {
        self.sql
    }
}
pub struct FetchOptional<'q, M: Model> {
    pub(crate) sql: &'q str, pub(crate) __as__:PhantomData<fn()->M>,
} impl<'q, M: Model> FetchOptional<'q, M> {
    pub(crate) fn new(sql: &'q str) -> Self {
        Self { sql, __as__:PhantomData }
    }
    pub(crate) fn sql(&self) -> &str {
        self.sql
    }
}

pub trait Model: for<'r> FromRow<'r, __feature__::Row> {
    fn all<'q>(sql: &'q str) -> FetchAll<'q, Self> {
        FetchAll { sql, __as__:PhantomData }
    }
    fn one<'q>(sql: &'q str) -> FetchOne<'q, Self> {
        FetchOne { sql, __as__:PhantomData }
    }
    fn optional<'q>(sql: &'q str) -> FetchOptional<'q, Self> {
        FetchOptional { sql, __as__:PhantomData }
    }
}
