#![allow(non_camel_case_types)]

use std::{pin::Pin, future::Future};
use sqlx::Executor;
use crate::{__feature__, Error, q, pool};
use super::{param::Param, transaction::X, QueryOutput};


impl<'q> FnOnce<(&'q str,)> for q {
    type Output = QueryOutput<'q>;
    extern "rust-call" fn call_once(self, (sql,): (&'q str,)) -> Self::Output {
        pool().execute(sql)
    }
}

impl<'q> FnOnce<(&'q str,)> for X {
    type Output = QueryOutput<'q>;
    extern "rust-call" fn call_once(mut self, (sql,): (&'q str,)) -> Self::Output {
        Box::pin(async move {self.0.execute(sql).await})
    }
}
impl<'q> FnMut<(&'q str,)> for X {
    extern "rust-call" fn call_mut(&mut self, (sql,): (&'q str,)) -> Self::Output {
        let output = self.0.execute(sql);
        unsafe {std::mem::transmute(output)}
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
