<h1 align="center">
    qjack
</h1>

## ergonomic sqlx wrapper for *nightly* Rust
- nightly only
- available DB：PostgreSQL, MySQL, SQLite
- available runtime：`tokio`, `async-std`

## Sample; How to use
```toml
[dependencies]
qjack = { version = "0.1", features = ["rt_tokio", "db_postgres"] }
```
part of `qjack/examples/friends.rs`
```rust
use qjack::{q, model};
type Result<T> = std::result::Result<T, qjack::Error>;

#[derive(Debug)]
#[model] struct Friend {
    id:       i32,
    name:     String,
    password: String,
}

impl Friend {
    async fn create_table_if_not_exists() -> Result<()> {
        q("CREATE TABLE IF NOT EXISTS friends (
            id SERIAL PRIMARY KEY,
            name VARCHAR(32) NOT NULL,
            password VARCHAR(64) NOT NULL
        )").await?; Ok(())
    }

    async fn find_by_id(id: i32) -> Result<Self> {
        q(Self::one("
            SELECT id, name, password FROM friends
            WHERE id = $1
        "), id).await
    }

    async fn search_by_password(password: &str) -> Result<Option<Self>> {
        q(Self::optional("
            SELECT id, name, password FROM friends
            WHERE password = $1
        "), password).await
    }

    async fn find_all_with_limit_by_name_like(like: &str, limit: i32) -> Result<Vec<Friend>> {
        q(Self::all("
            SELECT id, name, password FROM friends
            WHERE name LIKE $1
            LIMIT $2
        "), like, limit).await
    }

    async fn create_many(name_passwords: impl IntoIterator<Item = (String, String)>) -> Result<()> {
        let mut name_passwords = name_passwords.into_iter();

        let mut insert = String::from("INSERT INTO friends (name, password) VALUES");
        if let Some((first_name, first_password)) = name_passwords.next() {
            insert.push_str(&format!(" ('{}', '{}')", first_name, first_password))
        } else {return Ok(())}
        for (name, password) in name_passwords {
            insert.push_str(&format!(", ('{name}', '{password}')"))
        }

        q(&*insert).await?; Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    q.jack("postgres://qjack:password@postgres:5432/db")
        .max_connections(42)
        .await?;
    println!("Hi, jacked!");

    Friend::create_table_if_not_exists().await?;

```

## `q` magic

- `q.jack("DB_URL") /* config */ .await?` creates connection pool in background. All queries must be performed after this.
- `q("query string" /* , param1, param2, ... */ ).await?` executes a non-fetch query. This returns `QueryResult`.
- `q( Model::all("query string") /* , param1, param2, ... */ ).await?` executes a fetch-all query. This returns `Vec<Model>`.
- `q( Model::one("query string") /* , param1, param2, ... */ ).await?` executes a fetch-one query. This returns `Model`.
- `q( Model::optional("query string") /* , param1, param2, ... */ ).await?` executes a fetch-optional query. This returns `Option<Model>`.

Here `Model` means a `#[model] struct`.
