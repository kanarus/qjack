use qjack::q;
type Result<T> = std::result::Result<T, qjack::Error>;

#[tokio::main]
async fn main() -> Result<()> {
    q.jack("DB_URL").await?;

    unsafe {q.transaction(|mut x| async {
        if let Err(e) = x("
            UPDATE accounts
            SET balance = balance - 500
            WHERE id = 42 AND name = Alice
        ").await {
            eprintln!("Failed to subtract balance: {e}");
            return x.rollback().await
        }

        if let Err(e) = x("
            UPDATE accounts
            SET balance = balance + 500
            WHERE id = 24 AND name = Clara
        ").await {
            eprintln!("Failed to add balance: {e}");
            return x.rollback().await
        }

        x.commit().await
    })}.await?;

    Ok(())
}
