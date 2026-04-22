// SQL concepts: INNER JOIN, LEFT JOIN, joining multiple tables, aliasing.
// Run:  cargo run --bin 02_joins

use anyhow::Result;
use sql_and_rust::{connect, migrate};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
struct PostWithAuthor {
    post_id: i64,
    title: String,
    author: String,
}

#[derive(FromRow, Debug)]
struct UserPostCount {
    name: String,
    post_count: i64,
}

#[derive(FromRow, Debug)]
struct CommentContext {
    comment_id: i64,
    commenter: String,
    post_title: String,
    post_author: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;
    migrate(&pool).await?;

    // INNER JOIN — posts paired with their author's name.
    // Only rows where a matching user exists are returned.
    let rows: Vec<PostWithAuthor> = sqlx::query_as(
        "SELECT p.id AS post_id, p.title, u.name AS author
         FROM posts p
         INNER JOIN users u ON u.id = p.user_id
         ORDER BY p.id",
    )
    .fetch_all(&pool)
    .await?;

    println!("posts with authors (INNER JOIN):");
    for r in rows {
        println!("  [{}] {} — by {}", r.post_id, r.title, r.author);
    }

    // LEFT JOIN + COUNT — every user, plus their post count (including zero).
    // Change LEFT to INNER to see the difference (users with 0 posts disappear).
    let rows: Vec<UserPostCount> = sqlx::query_as(
        "SELECT u.name, COUNT(p.id)::BIGINT AS post_count
         FROM users u
         LEFT JOIN posts p ON p.user_id = u.id
         GROUP BY u.id, u.name
         ORDER BY post_count DESC, u.name",
    )
    .fetch_all(&pool)
    .await?;

    println!("\nposts per user (LEFT JOIN + COUNT):");
    for r in rows {
        println!("  {:<8} {} post(s)", r.name, r.post_count);
    }

    // Three-way JOIN — each comment with its commenter, the post, and the post's author.
    let rows: Vec<CommentContext> = sqlx::query_as(
        "SELECT
            c.id         AS comment_id,
            u.name       AS commenter,
            p.title      AS post_title,
            author.name  AS post_author,
            c.body
         FROM comments c
         JOIN users u      ON u.id = c.user_id
         JOIN posts p      ON p.id = c.post_id
         JOIN users author ON author.id = p.user_id
         ORDER BY c.id",
    )
    .fetch_all(&pool)
    .await?;

    println!("\ncomments with full context (three-way JOIN):");
    for r in rows {
        println!(
            "  [{}] {} commented on \"{}\" (by {}): {}",
            r.comment_id, r.commenter, r.post_title, r.post_author, r.body
        );
    }

    Ok(())
}
