<h1 align="center">
    qjack
</h1>

# This is just working draft (any codes here doesn't run now)

## ergonomic sqlx wrapper
- available DB：PostgreSQL, MySQL, SQLite
- available runtime：`tokio`, `async-std`

## Sample; How to use
```rust
use qjack::{q, FromRow};

#[FromRow]
struct User {
    id:       usize,
    name:     String,
    password: String,
}

const dangerous_passwords: &[&str] = &[
    "password",
    "password123",
    "passwordpassword",
];

async fn sample() -> Result<(), qjack::Error> {
    qjack::spawn("MY DB URL")
        .max_connections(1024)
        .await?;

    let dangerous_users = q.all::<User>(r#"
        SELECT id, name, password FROM users
        WHERE
            (id > $1) AND
            (password IN $2)
        ORDER BY name
    "#, (42, dangerous_passwords)).await?;

    println!("{dangerous_users}");
    Ok(())
}
```
