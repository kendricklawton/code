// SQL concepts: INSERT ... RETURNING, SELECT, UPDATE, DELETE (full CRUD cycle).
// Run:  cargo run --bin 01_basics

use anyhow::Result;
use chrono::{DateTime, Utc};
use sql_and_rust::{connect, migrate};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
#[allow(dead_code)]
struct User {
    id: i64,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;
    migrate(&pool).await?;

    // INSERT with RETURNING — get the created row back in one round-trip.
    let user: User = sqlx::query_as(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *",
    )
    .bind("Dave")
    .bind(format!("dave+{}@example.com", Utc::now().timestamp()))
    .fetch_one(&pool)
    .await?;
    println!("inserted: id={}, name={}, email={}", user.id, user.name, user.email);

    // SELECT — list all users.
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users ORDER BY id")
        .fetch_all(&pool)
        .await?;
    println!("\nall users:");
    for u in &users {
        println!("  {:>3}  {:<8}  {}", u.id, u.name, u.email);
    }

    // UPDATE — rename Dave, capture affected rows.
    let affected = sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
        .bind("David")
        .bind(user.id)
        .execute(&pool)
        .await?
        .rows_affected();
    println!("\nupdated {} row(s)", affected);

    // DELETE — clean up Dave so the script is rerunnable.
    let affected = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user.id)
        .execute(&pool)
        .await?
        .rows_affected();
    println!("deleted {} row(s)", affected);

    Ok(())
}
