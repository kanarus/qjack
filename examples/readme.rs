use qjack::{q, FromRow};

// #[derive(FromRow)]
// struct User {
//     id: i64,
//     name: String,
// }

#[tokio::main]
async fn main() -> Result<(), qjack::Error> {
    qjack::spawn("My DB URL")
        .max_connections(1024)
        .await?;
    
    q("INSERT INTO users (id, name) VALUES ($1, $2)",
        (1, "Alice")
    ).await?;

    let all_users = q.all("SELECT id, name FROM users", ());

    Ok(())
}
