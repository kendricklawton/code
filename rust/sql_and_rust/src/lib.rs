use anyhow::Result;
use sqlx::PgPool;

pub async fn connect() -> Result<PgPool> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (see .env)");
    let pool = PgPool::connect(&url).await?;
    Ok(pool)
}

pub async fn migrate(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
