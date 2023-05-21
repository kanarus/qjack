#![allow(unused, non_snake_case/* for account names */)]

use qjack::{q, Model};
type Result<T> = std::result::Result<T, qjack::Error>;

#[derive(Model, Debug)]
struct Account {
    id: i64,
    name: String,
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
            ", ammount, &self.id).await {
                eprintln!("Failed to subtract balance: {e}");
                return x.rollback().await
            }

            if let Err(e) = x("
                UPDATE accounts
                SET balance = balance + $1
                WHERE id = $2
            ", ammount, &payee.id).await {
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
    Account::create_table_if_not_exists().await?;

    let mut Alice = Account::create_new("Alice").await?;
    Alice.gets_income(1024).await?;

    let mut Clara = Account::create_new("Clara").await?;
    Clara.gets_income(2048).await?;

    println!("Created 2 accounts: {Alice:#?}, {Clara:#?}");

    unsafe {Alice.transfer_to(&mut Clara, 512).await?;}
    println!("\nSucceeded to transfer: {Alice:#?}, {Clara:#?}");

    Ok(())
}
