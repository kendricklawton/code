# sql_and_rust

A playground for mastering PostgreSQL through Rust. Real schema, real queries, real Postgres in Docker — no toy abstractions.

## Stack

- **Database:** PostgreSQL 17 (latest stable) in Docker
- **Driver:** [`sqlx`](https://github.com/launchbadge/sqlx) — async, type-safe, raw SQL (not an ORM)
- **Runtime:** `tokio`
- **Migrations:** `sqlx::migrate!` — embedded at compile time, tracked in `_sqlx_migrations`

## Prerequisites

- Docker + Docker Compose
- Rust 1.80+ (for edition 2024)

## Setup

```bash
# 1. Start Postgres (waits until healthy)
docker compose up -d --wait

# 2. Run any example — migrations auto-apply on first call
cargo run --bin 01_basics
cargo run --bin 02_joins
cargo run --bin 03_aggregations
cargo run --bin 04_transactions
```

The `.env` file points at `localhost:5432`. First `cargo build` pulls ~40 crates, so allow a minute.

## Database access

```bash
# Drop into psql inside the container
docker compose exec postgres psql -U postgres -d playground

# Or from the host (if psql is installed)
psql postgres://postgres:postgres@localhost:5432/playground
```

### Handy psql commands

| Command | What it does |
|---|---|
| `\dt` | list tables |
| `\d users` | describe the `users` table |
| `\di` | list indexes |
| `\df` | list functions |
| `\x` | toggle expanded row display (great for wide rows) |
| `EXPLAIN ANALYZE <query>` | show execution plan with real timings |

## Schema

```
users ───< posts ───< comments
   │                      │
   └──────────────────────┘
         (comment author)
```

See `migrations/0001_init.sql` for the exact DDL and `migrations/0002_seed.sql` for sample data.

## Example binaries

| Bin | SQL concepts covered |
|---|---|
| `01_basics` | `INSERT ... RETURNING`, `SELECT`, `UPDATE`, `DELETE`, parameter binding |
| `02_joins` | `INNER JOIN`, `LEFT JOIN`, three-way joins, aliases |
| `03_aggregations` | `GROUP BY`, `HAVING`, window functions (`ROW_NUMBER`, `RANK`), `PARTITION BY`, CTEs |
| `04_transactions` | `BEGIN` / `COMMIT` / `ROLLBACK`, atomicity, constraint violations |

Add your own in `src/bin/`. Each file becomes a separate binary automatically.

## Exercises (in rough order of difficulty)

### Beginner
1. Add a `tags` table and a `post_tags` join table (many-to-many). Write a query listing posts with their comma-separated tags using `STRING_AGG`.
2. Write a `SELECT` that returns each user's latest post (the one with the newest `created_at`).
3. Add a `likes` column to `posts`, default 0. Write an `UPDATE` that increments likes for a post by id.

### Intermediate
4. Use a CTE to find "super commenters" — users whose comment count is above the average comment count per user.
5. Write a query with `LATERAL JOIN` that returns each user's two most-recent posts.
6. Add a partial index on `posts` for only rows where `likes > 100`. Benchmark with `EXPLAIN ANALYZE`.

### Advanced
7. Create a Postgres `FUNCTION` that returns a user's activity summary (post count, comment count, last active timestamp) as a composite type. Call it from Rust.
8. Use `LISTEN` / `NOTIFY` to have a Rust async task react to inserts on `comments` in real time.
9. Add a `tsvector` column on `posts` with a `GENERATED ALWAYS AS` expression; create a GIN index and implement full-text search from Rust.
10. Implement optimistic locking: add a `version` column to `users`, update via `WHERE id = $1 AND version = $2 RETURNING version`. Demonstrate a lost-update prevention.

### Performance / ops
11. Load 100k fake users with `generate_series` + a Python/Rust fake-data generator. Profile a query before/after adding an index.
12. Measure the round-trip latency of a single query vs. 100 queries in a single transaction vs. `UNNEST`-based bulk insert.
13. Kill the Postgres container mid-transaction and observe Rust's error handling. Add a reconnection strategy to `connect()`.

## Useful commands

```bash
# Reset everything (deletes data!)
docker compose down -v

# View Postgres logs
docker compose logs -f postgres

# Run migrations manually (or just run any bin)
cargo run --bin 01_basics  # migrations run on startup

# Explore schema
docker compose exec postgres psql -U postgres -d playground -c '\d+ posts'
```

## Upgrading to compile-time checked queries

The examples use runtime queries (`sqlx::query_as("SELECT ...")`) — typos are caught at runtime. To graduate to compile-time checked queries (`sqlx::query_as!` macro), install `sqlx-cli`:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Then replace `query_as("SELECT *...")` with `query_as!(User, "SELECT *...")`. The macro verifies the query against the live DB at build time, and infers types from the schema. Run `cargo sqlx prepare` to cache the metadata for offline/CI builds.

## Why sqlx (not diesel, not sea-orm)?

- You write **actual SQL**, not a builder DSL. Transfers directly to other languages/tools.
- Compile-time verification when you want it, runtime strings when you don't.
- No ORM magic — you always know exactly what query hits the DB.

ORMs are great for CRUD apps where you never touch the SQL. For *mastering* SQL, raw queries are the point.
