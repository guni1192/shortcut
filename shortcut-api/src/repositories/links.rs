use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use sqlx::types::{chrono, Uuid};

#[derive(Debug, sqlx::FromRow)]
pub struct Link {
    /// id is UUID v4
    pub id: Uuid,
    /// name: URL short name
    pub name: String,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub user_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
}

#[async_trait]
pub trait Repository {
    async fn get_link_by_name(&self, name: &str) -> Result<Link, sqlx::Error>;
    async fn insert_link(&self, name: &str, url: &str) -> Result<Link, sqlx::Error>;
}

pub struct ShortcutRepository {
    pool: Pool<Postgres>,
}

impl ShortcutRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for ShortcutRepository {
    async fn get_link_by_name(&self, name: &str) -> Result<Link, sqlx::Error> {
        let link = sqlx::query_as!(Link, "SELECT * FROM links WHERE name = $1", name)
            .fetch_one(&self.pool)
            .await?;
        Ok(link)
    }

    async fn insert_link(&self, name: &str, url: &str) -> Result<Link, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let row = sqlx::query!(
            "INSERT INTO links (name, url) VALUES ($1, $2) RETURNING id",
            name,
            url
        )
        .fetch_one(&mut *tx)
        .await?;

        let link = sqlx::query_as!(Link, "SELECT * FROM links WHERE id = $1", row.id)
            .fetch_one(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(link)
    }
}
