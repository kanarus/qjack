use qjack::{q, Error, model};

#[derive(Debug)]
#[model] struct User {
    id:       i64,
    name:     String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    q.jack("DB_URL")
        .max_connections(42)
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

    q(r#" UPDATE users SET password = $1 WHERE password = 'password' "#,
        "newpassword",
    ).await?;

    let users_ending_with_a: Vec<User> = q(User::all(r#"
        SELECT id, name, password FROM users
        WHERE name LIKE $1
        ORDER BY name
        LIMIT $2
    "#), "%a", 100).await?;

    println!("{users_ending_with_a:?}");
    Ok(())
}
