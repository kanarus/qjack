use sqlx::{Encode, Type};
use crate::__feature__;


#[allow(non_camel_case_types)]
type sqlxQuery<'q> = sqlx::query::Query<'q, __feature__::DB, __feature__::Params>;
#[allow(non_camel_case_types)]
type sqlxQueryAs<'q, Model> = sqlx::query::QueryAs<'q, __feature__::DB, Model, __feature__::Params>;

pub trait FromRow: Send + Unpin + for <'r> sqlx::FromRow<'r, __feature__::Row> {}
impl<Model: Send + Unpin + for <'r> sqlx::FromRow<'r, __feature__::Row>> FromRow for Model {}


pub trait IntoQueryParams<'q>: Sized {fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q>;}
const _: () = {
    trait Param<'q>: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}
    impl<'q, T> Param<'q> for T where T: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}

    impl<'q> IntoQueryParams<'q> for () {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query
        }
    }
    impl<'q, P0:Param<'q>> IntoQueryParams<'q> for (P0,) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>> IntoQueryParams<'q> for (P0,P1) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>> IntoQueryParams<'q> for (P0,P1,P2) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1).bind(self.2)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>> IntoQueryParams<'q> for (P0,P1,P2,P3) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>> IntoQueryParams<'q> for (P0,P1,P2,P3,P4) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>, P5:Param<'q>> IntoQueryParams<'q> for (P0,P1,P2,P3,P4,P5) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4).bind(self.5)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>, P5:Param<'q>, P6:Param<'q>> IntoQueryParams<'q> for (P0,P1,P2,P3,P4,P5,P6) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4).bind(self.5).bind(self.6)
        }
    }
    impl<'q, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>, P5:Param<'q>, P6:Param<'q>, P7:Param<'q>> IntoQueryParams<'q> for (P0,P1,P2,P3,P4,P5,P6,P7) {
        fn binded(self, query: sqlxQuery<'q>) -> sqlxQuery<'q> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4).bind(self.5).bind(self.6).bind(self.7)
        }
    }
};

pub trait IntoQueryAsParams<'q, Model:FromRow>: Sized {fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model>;}
const _: () = {
    trait Param<'q>: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}
    impl<'q, T> Param<'q> for T where T: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB> {}

    impl<'q, Model:FromRow> IntoQueryAsParams<'q, Model> for () {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1,P2) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1).bind(self.2)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1,P2,P3) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1,P2,P3,P4) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>, P5:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1,P2,P3,P4,P5) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4).bind(self.5)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>, P5:Param<'q>, P6:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1,P2,P3,P4,P5,P6) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4).bind(self.5).bind(self.6)
        }
    }
    impl<'q, Model:FromRow, P0:Param<'q>, P1:Param<'q>, P2:Param<'q>, P3:Param<'q>, P4:Param<'q>, P5:Param<'q>, P6:Param<'q>, P7:Param<'q>> IntoQueryAsParams<'q, Model> for (P0,P1,P2,P3,P4,P5,P6,P7) {
        fn binded(self, query: sqlxQueryAs<'q, Model>) -> sqlxQueryAs<'q, Model> {
            query.bind(self.0).bind(self.1).bind(self.2).bind(self.3).bind(self.4).bind(self.5).bind(self.6).bind(self.7)
        }
    }
};




#[cfg(test)]
fn __assert_impls__() {
    fn impl_params<'q, T: 'q + Send + Encode<'q, __feature__::DB> + Type<__feature__::DB>>() {}
    impl_params::<String>();
    impl_params::<&str>();
    impl_params::<&String>();

    impl_params::<i8>();
    impl_params::<&i8>();

    impl_params::<i64>();
    impl_params::<&i64>();


    fn impl_into_query_params<'q, T: IntoQueryParams<'q>>() {}
    impl_into_query_params::<(String,)>();
    // impl_into_query_params::<(&str, &i8)>();
}







