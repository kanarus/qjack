#![allow(non_upper_case_globals)]
use std::marker::PhantomData;
use sqlx::FromRow;
use crate::__feature__;

pub struct FetchAll<'q, M: Model> {
    pub(crate) __as__:PhantomData<fn()->M>, pub(crate) sql: &'q str
}
pub struct FetchOne<'q, M: Model> {
    pub(crate) __as__:PhantomData<fn()->M>, pub(crate) sql: &'q str
}
pub struct FetchOptional<'q, M: Model> {
    pub(crate) __as__:PhantomData<fn()->M>, pub(crate) sql: &'q str
}

#[allow(non_camel_case_types)]
pub trait Model: for<'r> FromRow<'r, __feature__::Row> {
    #[inline(always)] fn all<'q>(sql: &'q str) -> FetchAll<'q, Self> {
        FetchAll { sql, __as__:PhantomData }
    }
    #[inline(always)] fn one<'q>(sql: &'q str) -> FetchOne<'q, Self> {
        FetchOne { sql, __as__:PhantomData }
    }
    #[inline(always)] fn optional<'q>(sql: &'q str) -> FetchOptional<'q, Self> {
        FetchOptional { sql, __as__:PhantomData }
    }
}

/*
macro_rules! impl_from_row_for_tuple {
    ($( ($idx:tt) -> $T:ident );+;) => {
        impl<'r, R, $($T,)+> FromRow<'r, R> for ($($T,)+)
        where
            R: Row,
            usize: crate::column::ColumnIndex<R>,
            $($T: crate::decode::Decode<'r, R::Database> + crate::types::Type<R::Database>,)+
        {
            #[inline]
            fn from_row(row: &'r R) -> Result<Self, Error> {
                Ok(($(row.try_get($idx as usize)?,)+))
            }
        }
    };
}
*/

macro_rules! impl_model_for_tuple {
    ( $( $index:literal -> $T:ident; )+ ) => {
        impl<$( $T ),+> Model for ( $( $T, )+ )
        where
            $($T: for<'r> crate::__private__::Decode<'r, __feature__::DB> + crate::__private__::Type<__feature__::DB>,)+
        {}
    };
} const _: () = {
    impl_model_for_tuple!(
        0 -> T0;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
        2 -> T2;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
        2 -> T2;
        3 -> T3;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
        2 -> T2;
        3 -> T3;
        4 -> T4;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
        2 -> T2;
        3 -> T3;
        4 -> T4;
        5 -> T5;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
        2 -> T2;
        3 -> T3;
        4 -> T4;
        5 -> T5;
        6 -> T6;
    );
    impl_model_for_tuple!(
        0 -> T0;
        1 -> T1;
        2 -> T2;
        3 -> T3;
        4 -> T4;
        5 -> T5;
        6 -> T6;
        7 -> T7;
    );
};
