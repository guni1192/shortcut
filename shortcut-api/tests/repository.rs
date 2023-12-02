use shortcut_api::middlewares::db::postgres::pg_connect;
use shortcut_api::repositories::links::Link;
use shortcut_api::repositories::links::Repository;
use shortcut_api::repositories::links::ShortcutRepository;

async fn setup_link_data(repository: &ShortcutRepository) -> Result<(), sqlx::Error> {
    repository
        .insert_link("guni", "https://guni1192.com")
        .await?;
    Ok(())
}

lazy_static::lazy_static! {
    static ref DATABASE_URL: String = std::env::var("DATABASE_URL").unwrap();
}


#[tokio::test]
pub async fn test_insert_link() -> Result<(), anyhow::Error> {
    let pool = pg_connect(&DATABASE_URL).await?;
    let repository = ShortcutRepository::new(pool);

    let link: Link = repository
        .insert_link("guni", "https://guni1192.com")
        .await?;

    dbg!(&link);

    assert_eq!(link.name, "guni".to_string());
    assert_eq!(link.url, "https://guni1192.com".to_string());

    Ok(())
}

#[tokio::test]
pub async fn test_get_link_by_name() -> Result<(), anyhow::Error> {
    let pool = pg_connect(&DATABASE_URL).await?;
    let repository = ShortcutRepository::new(pool);
    setup_link_data(&repository).await?;

    let link: Link = repository.get_link_by_name("guni").await?;

    assert_eq!(link.name, "guni".to_string());
    assert_eq!(link.url, "https://guni1192.com".to_string());
    Ok(())
}

#[tokio::test]
pub async fn test_get_link_by_name_not_found() -> Result<(), anyhow::Error> {
    let pool = pg_connect(&DATABASE_URL).await?;
    let repository = ShortcutRepository::new(pool);
    setup_link_data(&repository).await?;

    let err = repository.get_link_by_name("foo").await.unwrap_err();

    match err {
        sqlx::Error::RowNotFound => {},
        _ => panic!("unexpected error: {:?}", err),
    }
    Ok(())
}
