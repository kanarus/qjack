#![allow(non_camel_case_types)]

use std::{pin::Pin, future::Future};
use sqlx::Executor;
use crate::{__feature__, Error, q, pool};
use super::{param::Param, transaction::X};


impl<'q> FnOnce<(&'q str,)> for q {
    type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>> + 'q>>;
    extern "rust-call" fn call_once(self, (sql,): (&'q str,)) -> Self::Output {
        pool().execute(sql)
    }
}
impl<'q, 'x:'q> FnOnce<(&'q str,)> for &'x mut X {
    type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>> + 'q>>;
    extern "rust-call" fn call_once(self, (sql,): (&'q str,)) -> Self::Output {
        self.0.execute(sql)
    }
}

macro_rules! str_query_with_params {
    ($( $param:ident )+) => {
        impl<'q, $( $param:Param<'q> ),+> FnOnce<(&'q str, $( $param ),+)> for q {
            type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>> + 'q>>;
            extern "rust-call" fn call_once(
                self,
                (sql, $( $param ),+): (&'q str, $( $param ),+)
            ) -> Self::Output {
                pool().execute(sqlx::query(sql)
                    $( .bind($param) )+
                )
            }
        }
        impl<'q, 'x:'q, $( $param:Param<'q> ),+> FnOnce<(&'q str, $( $param ),+)> for &'x mut X {
            type Output = Pin<Box<dyn Future<Output = Result<__feature__::QueryResult, Error>> + 'q>>;
            extern "rust-call" fn call_once(
                self,
                (sql, $( $param ),+): (&'q str, $( $param ),+)
            ) -> Self::Output {
                self.0.execute(sqlx::query(sql)
                    $( .bind($param) )+
                )
            }
        }
    };
} const _: () = {
    str_query_with_params!(p1);
    str_query_with_params!(p1 p2);
    str_query_with_params!(p1 p2 p3);
    str_query_with_params!(p1 p2 p3 p4);
    str_query_with_params!(p1 p2 p3 p4 p5);
    str_query_with_params!(p1 p2 p3 p4 p5 p6);
    str_query_with_params!(p1 p2 p3 p4 p5 p6 p7);
};
