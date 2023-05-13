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
```rust
use qjack::{q, Model};

#[derive(Debug)]
#[Model] struct User {
    id:       usize,
    name:     String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), qjack::Error> {
    q.jack("DB_URL")
        .max_connections(5)
        .await?;

    q(r#" CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(32) NOT NULL,
        password VARCHAR(64) NOT NULL
    ) "#).await?;

    q(r#" INSERT INTO users (name, password) VALUES
        ('Alice', 'password'),
        ('Billy', 'password123'),
        ('Clara', 'wordpass'),
        ('David', 'passwordpassword'),
        ('Elena', 'password'),
        ('Fiona', 'password123456')
    "#).await?;

    q(r#" UPDATE user SET password = $1 WHERE name LIKE $2 "#,
        "newpassword",
        "%a",
    ).await?;

    let users_ending_with_a = q(User::all(r#"
        SELECT id, name, password FROM users
        WHERE name LIKE $1
        ORDER BY name
        LIMIT $2
    "#), "%a", 100).await?;

    println!("{users_ending_with_a:?}");
    Ok(())
}
```
cf ) sqlx
```toml
[dependencies]
sqlx = { version = "0.6", features = ["runtime-tokio-native-tls", "postgres"] }
```
```rust
#[derive(sqlx::FromRow, Debug)]
struct User {
    id:       i64,
    name:     String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("DB_URL").await?;

    pool.execute(r#" CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(32) NOT NULL,
        password VARCHAR(64) NOT NULL
    ) "#).await?;

    pool.execute(r#" INSERT INTO users (name, password) VALUES
        ('Alice', 'password'),
        ('Billy', 'password123'),
        ('Clara', 'wordpass'),
        ('David', 'passwordpassword'),
        ('Elena', 'password'),
        ('Fiona', 'password123456')
    "#).await?;

    sqlx::query(r#" UPDATE user SET password = $1 WHERE name LIKE $2 "#)
        .bind("newpassword")
        .bind("%a")
        .execute(&pool)
        .await?;

    let users_ending_with_a = sqlx::query_as::<_, User>(r#"
        SELECT id, name, password FROM users
        WHERE name LIKE $1
        ORDER BY name
        LIMIT $2
    "#).bind("%a").bind("%a").fetch_all(&pool).await?;

    println!("{users_ending_with_a:?}");
    Ok(())
}
```
