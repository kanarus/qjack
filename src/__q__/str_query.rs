use std::{pin::Pin, future::Future};
use sqlx::Executor;
use crate::{__feature__, Error, q, pool};
use super::param::Param;


#[allow(non_camel_case_types)]
impl<'q> FnOnce<(&'q str,)> for q {
    type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>> + 'q>>;
    extern "rust-call" fn call_once(self, (sql,): (&'q str,)) -> Self::Output {
        pool().execute(sql)
    }
}

macro_rules! impl_q_str_with_params {
    ($( $param:ident )+) => {
        #[allow(non_camel_case_types)]
        impl<'q, $( $param:Param<'q> ),+> FnOnce<(&'q str, $( $param ),+)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>> + 'q>>;
            extern "rust-call" fn call_once(self,
                (sql, $( $param ),+): (&'q str, $( $param ),+)
            ) -> Self::Output {
                pool().execute(sqlx::query(sql)
                    $( .bind($param) )+
                )
            }
        }
    };
} const _: () = {
    impl_q_str_with_params!(p1);
    impl_q_str_with_params!(p1 p2);
    impl_q_str_with_params!(p1 p2 p3);
    impl_q_str_with_params!(p1 p2 p3 p4);
    impl_q_str_with_params!(p1 p2 p3 p4 p5);
    impl_q_str_with_params!(p1 p2 p3 p4 p5 p6);
    impl_q_str_with_params!(p1 p2 p3 p4 p5 p6 p7);
};
