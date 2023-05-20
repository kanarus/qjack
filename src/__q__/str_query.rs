#![allow(non_camel_case_types)]
use sqlx::Executor;
use crate::{q, pool};
use super::{param::Param, transaction::X, QueryOutput};


impl<'q> FnOnce<(&'q str,)> for q {
    type Output = QueryOutput<'q>;
    #[inline(always)] extern "rust-call" fn call_once(self, (sql,): (&'q str,)) -> Self::Output {
        pool().execute(sql)
    }
}

impl<'q> FnOnce<(&'q str,)> for X {
    type Output = QueryOutput<'q>;
    #[inline(always)] extern "rust-call" fn call_once(mut self, (sql,): (&'q str,)) -> Self::Output {
        Box::pin(async move {self.0.execute(sql).await})
    }
}
impl<'q> FnMut<(&'q str,)> for X {
    #[inline(always)] extern "rust-call" fn call_mut(&mut self, (sql,): (&'q str,)) -> Self::Output {
        let output = self.0.execute(sql);
        unsafe {std::mem::transmute(output)}
    }
}

macro_rules! str_query_with_params {
    ($( $param:ident )+) => {
        impl<'q, $( $param:Param<'q> ),+> FnOnce<(&'q str, $( $param ),+)> for q {
            type Output = QueryOutput<'q>;
            #[inline(always)] extern "rust-call" fn call_once(
                self,
                (sql, $( $param ),+): (&'q str, $( $param ),+)
            ) -> Self::Output {
                pool().execute(sqlx::query(sql)
                    $( .bind($param) )+
                )
            }
        }

        impl<'q, $( $param:Param<'q> ),+> FnOnce<(&'q str, $( $param ),+)> for X {
            type Output = QueryOutput<'q>;
            #[inline(always)] extern "rust-call" fn call_once(
                mut self,
                (sql, $( $param ),+): (&'q str, $( $param ),+)
            ) -> Self::Output {
                Box::pin(async move {
                    self.0.execute(
                        sqlx::query(sql)
                            $( .bind($param) )+
                    ).await
                })
            }
        }
        impl<'q, $( $param:Param<'q> ),+> FnMut<(&'q str, $( $param ),+ )> for X {
            #[inline(always)] extern "rust-call" fn call_mut(
                &mut self,
                (sql, $( $param ),+): (&'q str, $( $param ),+)
            ) -> Self::Output {
                let output = self.0.execute(
                    sqlx::query(sql)
                        $( .bind($param) )+
                );
                unsafe {std::mem::transmute(output)}
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
