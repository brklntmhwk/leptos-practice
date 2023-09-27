use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn run_database(db_url: &str) -> Result<Pool<Postgres>> {
    let pool_options = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .with_context(|| "Unable to connect to database..")?;

    Ok(pool_options)
}
