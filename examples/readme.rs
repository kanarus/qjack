use qjack::{q, FromRow};

// #[derive(FromRow)]
// struct User {
//     id: i64,
//     name: String,
// }

const ADMIN_NAME: &str = "adminmin";

#[tokio::main]
async fn main() -> Result<(), qjack::Error> {
    qjack::spawn("My DB URL")
        .max_connections(1024)
        .await?;
    
    q("CREATE TABLE IF NOT EXISTS (
        id SERIAL PRIMARY KEY,
        name VARCHAR(32) NOT NULL
    )").await?;

    q("INSERT INTO users (name) VALUES
        ($1), ($2), ($3), ($4), ($5), ($6), ($7), ($8)
    ", (
        ADMIN_NAME,
        "Clara",
        "Elena",
        ADMIN_NAME,
        "Alice",
        "Billy",
        "Fiona",
        "David",
    )).await?;

    let (people_number,) = q.one::<(i64,)>("SELECT COUNT(*) FROM users", ()).await?;
    assert_eq!(people_number, 8);

    let all_users = q.all::<(i64, String)>("
        SELECT id, name FROM users
        WHERE name != $1
        ORDER BY name
    ", (ADMIN_NAME,)).await?;

    println!("{all_users:?}");
    Ok(())
}
