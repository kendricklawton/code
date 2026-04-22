// SQL concepts: GROUP BY, HAVING, window functions (ROW_NUMBER, RANK, PARTITION BY),
// Common Table Expressions (WITH clauses).
// Run:  cargo run --bin 03_aggregations

use anyhow::Result;
use sql_and_rust::{connect, migrate};
use sqlx::FromRow;

#[derive(FromRow, Debug)]
struct HotPost {
    title: String,
    author: String,
    comment_count: i64,
}

#[derive(FromRow, Debug)]
struct PostRanked {
    user_name: String,
    title: String,
    comment_count: i64,
    rank_in_user: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect().await?;
    migrate(&pool).await?;

    // GROUP BY + HAVING — posts with at least 2 comments.
    // HAVING filters groups after aggregation (WHERE filters rows before).
    let rows: Vec<HotPost> = sqlx::query_as(
        "SELECT p.title, u.name AS author, COUNT(c.id)::BIGINT AS comment_count
         FROM posts p
         JOIN users u ON u.id = p.user_id
         LEFT JOIN comments c ON c.post_id = p.id
         GROUP BY p.id, p.title, u.name
         HAVING COUNT(c.id) >= 2
         ORDER BY comment_count DESC",
    )
    .fetch_all(&pool)
    .await?;

    println!("hot posts (>= 2 comments):");
    for r in rows {
        println!("  {:<22} by {:<6} — {} comments", r.title, r.author, r.comment_count);
    }

    // Window function — for each user, rank their posts by comment count.
    // PARTITION BY resets ranking for each user; ORDER BY inside OVER defines rank order.
    let rows: Vec<PostRanked> = sqlx::query_as(
        "SELECT
            user_name,
            title,
            comment_count,
            ROW_NUMBER() OVER (PARTITION BY user_id ORDER BY comment_count DESC)::BIGINT
                AS rank_in_user
         FROM (
            SELECT u.id AS user_id, u.name AS user_name, p.id AS post_id, p.title,
                   COUNT(c.id) AS comment_count
            FROM users u
            JOIN posts p ON p.user_id = u.id
            LEFT JOIN comments c ON c.post_id = p.id
            GROUP BY u.id, u.name, p.id, p.title
         ) sub
         ORDER BY user_name, rank_in_user",
    )
    .fetch_all(&pool)
    .await?;

    println!("\nposts ranked per user by comment count (window function):");
    let mut current = String::new();
    for r in rows {
        if r.user_name != current {
            println!("\n  {}:", r.user_name);
            current = r.user_name;
        }
        println!(
            "    #{}  {:<22} {} comment(s)",
            r.rank_in_user, r.title, r.comment_count
        );
    }

    // CTE (Common Table Expression) — same result as above with cleaner syntax.
    // CTEs make complex queries readable by naming intermediate results.
    let rows: Vec<PostRanked> = sqlx::query_as(
        "WITH post_stats AS (
            SELECT u.id AS user_id, u.name AS user_name, p.id AS post_id, p.title,
                   COUNT(c.id) AS comment_count
            FROM users u
            JOIN posts p ON p.user_id = u.id
            LEFT JOIN comments c ON c.post_id = p.id
            GROUP BY u.id, u.name, p.id, p.title
         )
         SELECT user_name, title, comment_count::BIGINT,
                RANK() OVER (PARTITION BY user_id ORDER BY comment_count DESC)::BIGINT
                    AS rank_in_user
         FROM post_stats
         ORDER BY user_name, rank_in_user",
    )
    .fetch_all(&pool)
    .await?;

    println!("\nsame query via CTE + RANK (note: RANK gives ties the same number):");
    let mut current = String::new();
    for r in rows {
        if r.user_name != current {
            println!("\n  {}:", r.user_name);
            current = r.user_name;
        }
        println!(
            "    #{}  {:<22} {} comment(s)",
            r.rank_in_user, r.title, r.comment_count
        );
    }

    Ok(())
}
