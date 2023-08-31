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

<<<<<<< Updated upstream
        println!("{:?}", link);
=======
>>>>>>> Stashed changes
        Ok(link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::repositories::MySqlClient;

    async fn setup_link_data(repository: &ShortcutRepository) -> Result<(), sqlx::Error> {
        repository
            .insert_link("guni", "https://guni1192.com")
            .await?;
        Ok(())
    }

    #[tokio::test]
    pub async fn test_get_link_by_name() -> Result<(), anyhow::Error> {
        let pool = MySqlClient::from_env().connect().await?;
        let repository = ShortcutRepository::new(pool);
        setup_link_data(&repository).await?;

        let link = repository.get_link_by_name("guni").await.unwrap();

        assert_eq!(link.name, "guni".to_string());
        assert_eq!(link.url, "https://guni1192.com".to_string());
        Ok(())
    }

    #[tokio::test]
    pub async fn test_get_link_by_name_not_found() -> Result<(), anyhow::Error> {
        let pool = MySqlClient::from_env().connect().await?;
        let repository = ShortcutRepository::new(pool);
        setup_link_data(&repository).await?;

        let err = repository.get_link_by_name("foo").await.unwrap_err();

        match err {
            sqlx::Error::RowNotFound => assert!(true),
            _ => assert!(false, "unexpected error: {:?}", err),
        }
        Ok(())
    }
}
