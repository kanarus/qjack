use qjack::q;
type Result<T> = std::result::Result<T, qjack::Error>;

const AMMOUNT:  i32 = 500;
const PAYER_ID: i32 = 42;
const PAYEE_ID: i32 = 24;
const PAYER_NAME: &str = "Alice";
const PAYEE_NAME: &str = "Clara";

#[tokio::main]
async fn main() -> Result<()> {
    q.jack("DB_URL").await?;

    unsafe {q.transaction(|mut x| async {
        if let Err(e) = x("
            UPDATE accounts
            SET balance = balance - $1
            WHERE id = $2 AND name = $3
        ", AMMOUNT, PAYER_ID, PAYER_NAME).await {
            eprintln!("Failed to subtract balance: {e}");
            return x.rollback().await
        }

        if let Err(e) = x("
            UPDATE accounts
            SET balance = balance + $1
            WHERE id = $2 AND name = $3
        ", AMMOUNT, PAYEE_ID, PAYEE_NAME).await {
            eprintln!("Failed to add balance: {e}");
            return x.rollback().await
        }

        x.commit().await
    })}.await?;

    Ok(())
}
