// This sample app is so crazy because it stores passwords
// WITHOUT hashing. This should be improved in real apps.
// In addition, this app uses a literal DB URL for ease. Real apps
// should construt DB URL from environment variables or secret file
// or something.

use qjack::{q, Model};
type Result<T> = std::result::Result<T, qjack::Error>;

#[derive(Model, Debug)]
struct Friend {
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
            insert.push_str(&format!(" ('{first_name}', '{first_password}')"))
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

    let friends: Vec<(String, String)> = {
        println!();
        println!("Happy jacking! Could you enter data of some of your firends?");
        println!("(press q when you stop entering)");
        println!();

        let mut inputs = Vec::new(); loop {use std::io::{stdin, stdout, Write};
            print!("name: "); stdout().flush().ok();
            let mut name = String::new();
            stdin().read_line(&mut name).ok();
            name.pop(/* final '\n' */);
            if name == "q" {break}

            print!("password: "); stdout().flush().ok();
            let mut password = String::new();
            stdin().read_line(&mut password).ok();
            password.pop(/* final '\n' */);
            if password == "q" {break}

            inputs.push((name, password));
            println!()
        }

        println!("Ok, thanks!\n");
        inputs
    };
    if friends.is_empty() {
        println!("Oh No! You have no friends...");
        return Ok(())
    }
    Friend::create_many(friends).await?;

    let first_user = Friend::find_by_id(1).await?;
    println!("First user is `{}` (password: \"{}\").", first_user.name, first_user.password);

    match Friend::search_by_password("password").await? {
        None      => println!("No user has password \"password\"."),
        Some(one) => println!("{}th user `{}` has password \"password\"!.", one.id, one.name),
    }

    let friends_ending_with_a = Friend::find_all_with_limit_by_name_like("%a", 100).await?;
    println!("Fiends whose name ends with 'a': {friends_ending_with_a:#?}");

    Ok(())
}
