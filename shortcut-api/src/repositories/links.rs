use async_trait::async_trait;
use sqlx::types::{chrono, Uuid};
use sqlx::{Acquire, PgPool, Pool, Postgres};
use tracing::debug;

#[derive(Debug, sqlx::FromRow, Default)]
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
pub trait LinkRepository {
    async fn find_by_name(&self, name: &str) -> Result<Link, sqlx::Error>;
    async fn list(&self) -> Result<Vec<Link>, sqlx::Error>;
    async fn create(&self, name: &str, url: &str) -> Result<Link, sqlx::Error>;
    async fn delete(&self, name: &str) -> Result<(), sqlx::Error>;
}

#[derive(Debug)]
pub struct ScLinkRepository {
    pool: PgPool,
}

impl ScLinkRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LinkRepository for ScLinkRepository {
    async fn find_by_name(&self, name: &str) -> Result<Link, sqlx::Error> {
        let link = InternalLinkRepository::find_by_name(name, &self.pool).await?;
        Ok(link)
    }

    async fn list(&self) -> Result<Vec<Link>, sqlx::Error> {
        let links = InternalLinkRepository::list(&self.pool).await?;
        Ok(links)
    }

    async fn create(&self, name: &str, url: &str) -> Result<Link, sqlx::Error> {
        let link = InternalLinkRepository::create(name, url, &self.pool).await?;
        Ok(link)
    }

    async fn delete(&self, name: &str) -> Result<(), sqlx::Error> {
        InternalLinkRepository::delete(name, &self.pool).await?;
        Ok(())
    }
}

pub struct InternalLinkRepository;

impl InternalLinkRepository {
    pub async fn find_by_name<'a, A>(name: &str, conn: A) -> Result<Link, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + 'a,
    {
        let mut conn = conn.acquire().await?;
        let link = sqlx::query_as!(Link, "SELECT * FROM links WHERE name = $1", name)
            .fetch_one(&mut *conn)
            .await?;
        Ok(link)
    }

    pub async fn list<'a, A>(conn: A) -> Result<Vec<Link>, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + 'a,
    {
        let mut conn = conn.acquire().await?;
        let links = sqlx::query_as!(Link, "SELECT * FROM links")
            .fetch_all(&mut *conn)
            .await?;
        Ok(links)
    }

    pub async fn create<'a, A>(name: &str, url: &str, conn: A) -> Result<Link, sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + 'a,
    {
        let mut conn = conn.acquire().await?;
        let row = sqlx::query!(
            "INSERT INTO links (name, url) VALUES ($1, $2) RETURNING id",
            name,
            url
        )
        .fetch_one(&mut *conn)
        .await?;

        debug!("created link: {{ name: {name}, url: {url} }}");

        let link = sqlx::query_as!(Link, "SELECT * FROM links WHERE id = $1", row.id)
            .fetch_one(&mut *conn)
            .await?;

        Ok(link)
    }

    pub async fn delete<'a, A>(name: &str, conn: A) -> Result<(), sqlx::Error>
    where
        A: Acquire<'a, Database = Postgres> + 'a,
    {
        let mut conn = conn.acquire().await?;
        sqlx::query_as!(Link, "DELETE FROM links WHERE name = $1", name)
            .execute(&mut *conn)
            .await?;
        Ok(())
    }
}
