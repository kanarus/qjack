<h1 align="center">
    qjack
</h1>

## ergonomic sqlx wrapper for *nightly* Rust
- nightly only
- available DB：PostgreSQL, MySQL, SQLite
- available runtime：`tokio`, `async-std`

## Sample; How to use
part of `Cargo.toml`
```toml
[dependencies]
qjack = { version = "0.1", features = ["rt_tokio", "db_postgres"] }
```
`src/main.rs` (copied from `qjack/examples/user.rs`)
```rust
use qjack::{q, model, Error};

mod example_env {
    pub const POSTGRES_USER:     &str = "posgre";
    pub const POSTGRES_PASSWORD: &str = "password";
    pub const POSTGRES_HOST:     &str = "posgre";
    pub const POSTGRES_PORT:     &str = "5432";
}

#[derive(Debug)]
#[model] struct User {
    id:       i64,
    name:     String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_url = format!("postgres://{}:{}@{}:{}",
        example_env::POSTGRES_USER,
        example_env::POSTGRES_PASSWORD,
        example_env::POSTGRES_HOST,
        example_env::POSTGRES_PORT,
    );
    q.jack(&db_url)
        .max_connections(42)
        .await?;

    q("CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(32) NOT NULL,
        password VARCHAR(64) NOT NULL
    ) ").await?;

    q("INSERT INTO users (name, password) VALUES
        ('Alice', 'password'),
        ('Billy', 'password123'),
        ('Clara', 'wordpass'),
        ('David', 'passwordpassword'),
        ('Elena', 'password'),
        ('Fiona', 'password123456')
    ").await?;

    q("UPDATE users SET password = $1 WHERE password = 'password' ",
        "newpassword",
    ).await?;

    let users_ending_with_a = q(User::all("
        SELECT id, name, password FROM users
        WHERE name LIKE $1
        ORDER BY name
        LIMIT $2
    "), "%a", 100).await?;

    println!("{users_ending_with_a:?}");
    Ok(())
}
```

## `q` magic

- `q.jack("DB_URL") /* config */ .await?` creates connection pool in background. All queries must be performed after this.
- `q("query string" /* , param1, param2, ... */ ).await?` executes a non-fetch query. This returns `QueryResult`.
- `q( Model::all("query string") /* , param1, param2, ... */ ).await?` executes a fetch-all query. This returns `Vec<Model>`.
- `q( Model::one("query string") /* , param1, param2, ... */ ).await?` executes a fetch-one query. This returns `Model`.
- `q( Model::optional("query string") /* , param1, param2, ... */ ).await?` executes a fetch-optional query. This returns `Option<Model>`.

Here `Model` means a `#[model] struct`.
