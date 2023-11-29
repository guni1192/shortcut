use async_trait::async_trait;
use sqlx::{MySql, Pool};

#[derive(Debug, sqlx::FromRow)]
pub struct Link {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[async_trait]
pub trait Repository {
    async fn get_link_by_name(&self, name: &str) -> Result<Link, sqlx::Error>;
    async fn insert_link(&self, name: &str, url: &str) -> Result<Link, sqlx::Error>;
}

pub struct ShortcutRepository {
    pool: Pool<MySql>,
}

impl ShortcutRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for ShortcutRepository {
    async fn get_link_by_name(&self, name: &str) -> Result<Link, sqlx::Error> {
        let link = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;
        Ok(link)
    }

    async fn insert_link(&self, name: &str, url: &str) -> Result<Link, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let id = sqlx::query("INSERT INTO links (name, url) VALUES (?, ?)")
            .bind(name)
            .bind(url)
            .execute(&mut *tx)
            .await?
            .last_insert_id();

        let link = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(link)
    }
}
