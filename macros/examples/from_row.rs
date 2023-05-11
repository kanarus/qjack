fn main() {}

#[derive(sqlx::FromRow)]
struct User {
    id: usize,
    name: String,
}

/* expanded:

#[automatically_derived]
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for User
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    i64: ::sqlx::decode::Decode<'a, R::Database>,
    i64: ::sqlx::types::Type<R::Database>,
    String: ::sqlx::decode::Decode<'a, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
        let id: i64 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        ::std::result::Result::Ok(User { id, name })
    }
}

*/

struct T {
    id: usize,
}
impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for T
where
    &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    usize: ::sqlx::decode::Decode<'a, R::Database>,
    usize: ::sqlx::types::Type<R::Database>,
    // String: ::sqlx::decode::Decode<'a, R::Database>,
    // String: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
        let id: usize = row.try_get("id")?;
        // let name: String = row.try_get("name")?;
        ::std::result::Result::Ok(T { id })
    }
}

