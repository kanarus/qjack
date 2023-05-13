use std::{pin::Pin, future::Future};
use futures_util::{TryStreamExt, StreamExt, FutureExt};
use sqlx::{Executor, Either};
use crate::{__feature__, Error, Model, FetchAll, FetchOne, FetchOptional, q, pool};
use super::param::Param;

/*


    /// Execute the query and return the generated results as a stream.
    fn fetch<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxStream<'e, Result<<Self::Database as Database>::Row, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        self.fetch_many(query)
            .try_filter_map(|step| async move {
                Ok(match step {
                    Either::Left(_) => None,
                    Either::Right(row) => Some(row),
                })
            })
            .boxed()
    }

    /// Execute the query and return all the generated results, collected into a [`Vec`].
    fn fetch_all<'e, 'q: 'e, E: 'q>(
        self,
        query: E,
    ) -> BoxFuture<'e, Result<Vec<<Self::Database as Database>::Row>, Error>>
    where
        'c: 'e,
        E: Execute<'q, Self::Database>,
    {
        self.fetch(query).try_collect().boxed()
    }

*/

impl<'q, M:Model+'q> FnOnce<(FetchAll<'q, M>,)> for q {
    type Output = Pin<Box<dyn Future<Output = Result<Vec<M>, Error>> + 'q>>;
    extern "rust-call" fn call_once(self,
        (FetchAll{sql, __as__},): (FetchAll<'q, M>,)
    ) -> Self::Output {
        Box::pin(
            Box::pin(
                pool().fetch_many(sqlx::query(sql))
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
