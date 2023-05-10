// mod __ {
//     use std::{future::Future, task::Poll, pin::pin};
//     use sqlx::{Encode, Type};
//     use crate::{__feature__, pool, Error};
// 
//     trait Param<'q> = 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB>;
// 
//     pub struct Query<'q>(sqlx::query::Query<'q, __feature__::DB, __feature__::Arguments>);
//     impl<'q> Future for Query<'q> {
//         type Output = Result<(), Error>;
//         fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
//             let query_future = unsafe {self.map_unchecked_mut(|q| &mut q.0.execute(pool()))};
// 
//             todo!()
//         }
//     }
// 
//     pub struct Q;
//     impl<'q, P1: Param<'q>> FnOnce<(&'q str, (P1,))> for Q {
//         type Output = Query<'q>;
//         extern "rust-call" fn call_once(self,
//             (sql, (p1,)): (&'q str, (P1,))
//         ) -> Self::Output {
//             Query(sqlx::query(sql)
//                 .bind(p1)
//             )
//         }
//     }
//     // impl<'q, Args: Tuple> FnMut<(&'q str, Args)> for Q {}
//     // impl<'q, Args: Tuple> Fn<(&'q str, Args)> for Q {}
// }
// 

use crate::Error;

mod __ {
    use crate::__feature__;
    use sqlx::{Encode, Type};

    const PARAMS_LIMIT_NUM: usize = 16;
    pub(super) struct Params {
        buffer: __feature__::ParamsBuffer,
        types:  [Option<__feature__::TypeInfo>; PARAMS_LIMIT_NUM],
    } impl Params {
        pub(super) fn new() -> Self {
            Self {
                buffer: __feature__::ParamsBuffer::default(),
                types: [
                    None, None, None, None,
                    None, None, None, None,
                    None, None, None, None,
                    None, None, None, None,
                ]
            }
        }
        pub(super) fn add<'q, P: Param<'q>>(mut self, param: P) -> Self {
            self = Self {
                buffer: {self.buffer.encode(param); self.buffer.c},
                types:  ,
            }
        }
    }
    /*
    /// Implementation of [`Arguments`] for PostgreSQL.
#[derive(Default)]
pub struct PgArguments {
    // Types of each bind parameter
    pub(crate) types: Vec<PgTypeInfo>,

    // Buffer of encoded bind parameters
    pub(crate) buffer: PgArgumentBuffer,
}

impl PgArguments {
    pub(crate) fn add<'q, T>(&mut self, value: T)
    where
        T: Encode<'q, Postgres> + Type<Postgres>,
    {
        // remember the type information for this value
        self.types
            .push(value.produces().unwrap_or_else(T::type_info));

        // encode the value into our buffer
        self.buffer.encode(value);

        // increment the number of arguments we are tracking
        self.buffer.count += 1;
    }

    */

    trait Param<'q>: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}
    impl<'q, T> Param<'q> for T where T: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}

    pub trait IntoParams<'q>: Sized {
        fn into_params(self) -> __feature__::Params {
            let mut params = __feature__::Params::default();
            
            params
        }
    }
}

pub async fn q<'q, Params: __::IntoParams<'q>>(sql: &'q str, params: Params) -> Result<(), Error> {

}
