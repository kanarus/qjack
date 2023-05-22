<h1 align="center">qjack</h1>

## ergonomic sqlx wrapper for *nightly* Rust
- nightly only
- macro free
- available DB：PostgreSQL, MySQL, SQLite
- available runtime：`tokio`, `async-std`
- status：<img align="bottom" alt="qjack build check status" src="https://github.com/kana-rus/qjack/actions/workflows/check.yaml/badge.svg"/>

<br/>

## Sample; How to use
```toml
[dependencies]
qjack = { version = "0.1", features = ["rt_tokio", "db_postgres"] }
tokio = { version = "1", features = ["macros"] }
```
<br/>

part of [`sample/src/bin/friends.rs`](https://github.com/kana-rus/qjack/tree/main/sample/src/bin/friends.rs)
```rust
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

    // ...
```
<br/>

part of [`sample/src/bin/transfer.rs`](https://github.com/kana-rus/qjack/tree/main/sample/src/bin/transfer.rs)
```rust
#[derive(Model, Debug)]
struct Account {
    id:      i64,
    name:    String,
    balance: i64,
}

impl Account {
    async fn create_table_if_not_exists() -> Result<()> {
        q("CREATE TABLE IF NOT EXISTS accounts (
            id BIGSERIAL PRIMARY KEY,
            name VARCHAR(32) NOT NULL,
            balance INT8 DEFAULT 0
        )").await?; Ok(())
    }

    async fn create_new(name: &str) -> Result<Self> {
        let created = q(Self::one("
            INSERT INTO accounts
            (name, balance) VALUES ($1, $2)
            RETURNING id, name, balance
        "), name, 0).await?;

        Ok(created)
    }

    async fn gets_income(&mut self, income: i64) -> Result<()> {
        q("UPDATE accounts
            SET balance = balance + $1
            WHERE id = $2
        ", income, &self.id).await?;

        self.balance += income;

        Ok(())
    }

    // transaction is unsafe in current version
    async unsafe fn transfer_to(
        &mut self,
        payee: &mut Account,
        ammount: i64
    ) -> Result<()> {
        q.transaction(|mut x| async {
            if let Err(e) = x("
                UPDATE accounts
                SET balance = balance - $1
                WHERE id = $2
            ", &ammount, &self.id).await {
                eprintln!("Failed to subtract balance: {e}");
                return x.rollback().await
            }

            if let Err(e) = x("
                UPDATE accounts
                SET balance = balance + $1
                WHERE id = $2
            ", &ammount, &payee.id).await {
                eprintln!("Failed to add balance: {e}");
                return x.rollback().await
            }

            self.balance  -= ammount;
            payee.balance += ammount;

            x.commit().await
        }).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    q.jack("postgres://qjack:password@postgres:5432/db").await?;

    // ...
```

<br/>

## `q` magic

```rust
q.jack("DB_URL") /* config */ .await?
```
creates connection pool in background. All queries must be performed after this.

<br/>

```rust
q.transaction(|mut x| async {
    /*
        returns
            x.rollback().await
        or
            x.commit().await
    */
}).await?
```
performs a transaction. This is **`unsafe`** in current version.

<br/>

```rust
q("query string" /* , params, ... */ ).await?
```
executes a non-fetch query. This returns `QueryResult`.

<br/>

```rust
q( M::all("query string") /* , params, ... */ ).await?
```
```rust
q( M::one("query string") /* , params, ... */ ).await?
```
```rust
q( M::optional("query string") /* , params, ... */ ).await?
```
executes a fetch query. Return type：

- `all` → `Vec<M>`
- `one` → `M`,
- `optional` → `Option<M>`.

( Here `M` means a struct that impls `Model` )

<br/>

## LICENSE
`qjack` is licensed under MIT LICENSE ([LICENSE](https://github.com/kana-rus/qjack/blob/main/LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))