// SQL concepts: explicit transactions, ACID, rollback on error.
// The UNIQUE constraint on users.email triggers the failure path.
// Run:  cargo run --bin 04_transactions

use anyhow::Result;
use sql_and_rust::{connect, migrate};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;
    migrate(&pool).await?;

    // --- Happy path: atomic insert of a user and their first post.
    let mut tx = pool.begin().await?;

    let user_id: i64 = sqlx::query_scalar(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
    )
    .bind("Eve")
    .bind("eve@example.com")
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3)")
        .bind(user_id)
        .bind("Transactional post")
        .bind("This user + post were created atomically.")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    println!("committed: user id={user_id} with post");

    // --- Failure path: try to insert a duplicate email.
    // Everything done inside `tx` is discarded when we call rollback.
    let mut tx = pool.begin().await?;

    // First op would succeed in isolation...
    let _ = sqlx::query(
        "INSERT INTO posts (user_id, title, body)
         VALUES ((SELECT id FROM users WHERE email = 'alice@example.com'), 'would be rolled back', 'nope')",
    )
    .execute(&mut *tx)
    .await?;

    // ...but this one violates UNIQUE(email), so the whole thing rolls back.
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind("Dupe")
        .bind("eve@example.com")
        .execute(&mut *tx)
        .await;

    match result {
        Ok(_) => {
            tx.commit().await?;
            println!("(unexpected success)");
        }
        Err(e) => {
            tx.rollback().await?;
            println!("rolled back — constraint violation: {}", e);
        }
    }

    // Prove the rolled-back post insert did NOT land.
    let stranded: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM posts WHERE title = 'would be rolled back'",
    )
    .fetch_one(&pool)
    .await?;
    println!("stranded rollback rows in posts: {stranded} (should be 0)");

    // Clean up to keep the script rerunnable.
    sqlx::query("DELETE FROM users WHERE email = 'eve@example.com'")
        .execute(&pool)
        .await?;

    Ok(())
}
