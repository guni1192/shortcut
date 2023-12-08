use shortcut_api::repositories::links::{Link, LinkRepository, ScLinkRepository};
use sqlx::PgPool;

lazy_static::lazy_static! {
    static ref DATABASE_URL: String = std::env::var("DATABASE_URL").unwrap();
}

#[sqlx::test]
pub async fn test_link_create(pool: PgPool) -> Result<(), anyhow::Error> {
    let repository = ScLinkRepository::new(pool);
    let link = repository.create("guni", "https://guni1192.com").await?;

    dbg!(&link);

    assert_eq!(link.name, "guni".to_string());
    assert_eq!(link.url, "https://guni1192.com".to_string());

    Ok(())
}

#[sqlx::test]
pub async fn test_get_link_by_name(pool: PgPool) -> Result<(), anyhow::Error> {
    let repository = ScLinkRepository::new(pool);
    repository
        .create("guni", "https://guni1192.com")
        .await
        .expect("failed to create link");

    let link: Link = repository.find_by_name("guni").await?;

    assert_eq!(link.name, "guni".to_string());
    assert_eq!(link.url, "https://guni1192.com".to_string());
    Ok(())
}

#[sqlx::test]
pub async fn test_get_link_by_name_not_found(pool: PgPool) -> Result<(), anyhow::Error> {
    let repository = ScLinkRepository::new(pool);

    let err = repository
        .find_by_name("unknown_link_data")
        .await
        .unwrap_err();

    match err {
        sqlx::Error::RowNotFound => {}
        _ => panic!("unexpected error: {:?}", err),
    }
    Ok(())
}
