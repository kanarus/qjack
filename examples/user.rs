use qjack::{q, model, Error};

mod example_env {
    pub const POSTGRES_USER:     &str = "posgre";
    pub const POSTGRES_PASSWORD: &str = "password";
    pub const POSTGRES_HOST:     &str = "posgre";
    pub const POSTGRES_PORT:     &str = "5432";
    pub const POSTGRES_DATABASE: &str = "posgre";
}

#[derive(Debug)]
#[model] struct User {
    id:       i64,
    name:     String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_url = format!("postgres://{}:{}@{}:{}/{}",
        example_env::POSTGRES_USER,
        example_env::POSTGRES_PASSWORD,
        example_env::POSTGRES_HOST,
        example_env::POSTGRES_PORT,
        example_env::POSTGRES_DATABASE,
    );
    q.jack(&db_url)
        .max_connections(42)
        .await?;

    q("CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(32) NOT NULL,
        password VARCHAR(64) NOT NULL
    )").await?;

    q("INSERT INTO users (name, password) VALUES
        ('Alice', 'password'),
        ('Billy', 'password123'),
        ('Clara', 'wordpass'),
        ('David', 'passwordpassword'),
        ('Elena', 'password'),
        ('Fiona', 'password123456')
    ").await?;

    q("UPDATE users SET password = $1 WHERE password = 'password'",
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
