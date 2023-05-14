use std::{pin::Pin, future::Future};
use futures_util::{TryStreamExt};
use sqlx::{Executor, Either};
use crate::{Error, Model, FetchAll, FetchOne, FetchOptional, q, pool};
use super::param::Param;

macro_rules! impl_q_fetch_all_with_params {
    ($( $param:ident )*) => {
        #[allow(non_camel_case_types)]
        impl<'q, M:Model+'q, $( $param:Param<'q> ),*> FnOnce<(FetchAll<'q, M>, $( $param ),*)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<Vec<M>, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (FetchAll{sql, __as__}, $( $param ),*): (FetchAll<'q, M>, $( $param ),*)
            ) -> Self::Output {
                Box::pin(
                    Box::pin(
                        pool().fetch_many(sqlx::query(sql) $( .bind($param) )*)
                            .try_filter_map(|step| async move {
                                Ok(match step {
                                    Either::Left(_) => None,
                                    Either::Right(row) => M::from_row(&row).ok(),
                                })
                            })
                    )
                    .try_collect()
                )
            }
        }
    };
} const _: () = {
    impl_q_fetch_all_with_params!();
    impl_q_fetch_all_with_params!(p1);
    impl_q_fetch_all_with_params!(p1 p2);
    impl_q_fetch_all_with_params!(p1 p2 p3);
    impl_q_fetch_all_with_params!(p1 p2 p3 p4);
    impl_q_fetch_all_with_params!(p1 p2 p3 p4 p5);
    impl_q_fetch_all_with_params!(p1 p2 p3 p4 p5 p6);
    impl_q_fetch_all_with_params!(p1 p2 p3 p4 p5 p6 p7);
};

macro_rules! impl_q_fetch_one_with_params {
    ($( $param:ident )*) => {
        #[allow(non_camel_case_types)]
        impl<'q, M:Model+'q, $( $param:Param<'q> ),*> FnOnce<(FetchOne<'q, M>, $( $param ),*)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<Vec<M>, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (FetchOne{sql, __as__}, $( $param ),*): (FetchOne<'q, M>, $( $param ),*)
            ) -> Self::Output {
                Box::pin(
                    Box::pin(
                        pool().fetch_many(sqlx::query(sql) $( .bind($param) )*)
                            .try_filter_map(|step| async move {
                                Ok(match step {
                                    Either::Left(_) => None,
                                    Either::Right(row) => M::from_row(&row).ok(),
                                })
                            })
                    )
                    .try_collect()
                )
            }
        }
    };
} const _: () = {
    impl_q_fetch_one_with_params!();
    impl_q_fetch_one_with_params!(p1);
    impl_q_fetch_one_with_params!(p1 p2);
    impl_q_fetch_one_with_params!(p1 p2 p3);
    impl_q_fetch_one_with_params!(p1 p2 p3 p4);
    impl_q_fetch_one_with_params!(p1 p2 p3 p4 p5);
    impl_q_fetch_one_with_params!(p1 p2 p3 p4 p5 p6);
    impl_q_fetch_one_with_params!(p1 p2 p3 p4 p5 p6 p7);
};

macro_rules! impl_q_fetch_optional_with_params {
    ($( $param:ident )*) => {
        #[allow(non_camel_case_types)]
        impl<'q, M:Model+'q, $( $param:Param<'q> ),*> FnOnce<(FetchOptional<'q, M>, $( $param ),*)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<Vec<M>, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (FetchOptional{sql, __as__}, $( $param ),*): (FetchOptional<'q, M>, $( $param ),*)
            ) -> Self::Output {
                Box::pin(
                    Box::pin(
                        pool().fetch_many(sqlx::query(sql) $( .bind($param) )*)
                            .try_filter_map(|step| async move {
                                Ok(match step {
                                    Either::Left(_) => None,
                                    Either::Right(row) => M::from_row(&row).ok(),
                                })
                            })
                    )
                    .try_collect()
                )
            }
        }
    };
} const _: () = {
    impl_q_fetch_optional_with_params!();
    impl_q_fetch_optional_with_params!(p1);
    impl_q_fetch_optional_with_params!(p1 p2);
    impl_q_fetch_optional_with_params!(p1 p2 p3);
    impl_q_fetch_optional_with_params!(p1 p2 p3 p4);
    impl_q_fetch_optional_with_params!(p1 p2 p3 p4 p5);
    impl_q_fetch_optional_with_params!(p1 p2 p3 p4 p5 p6);
    impl_q_fetch_optional_with_params!(p1 p2 p3 p4 p5 p6 p7);
};
