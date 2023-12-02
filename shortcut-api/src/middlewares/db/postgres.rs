use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

pub async fn pg_connect(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool_opts = PgPoolOptions::new();
    let pool = pool_opts.connect(database_url).await?;
    Ok(pool)
}
