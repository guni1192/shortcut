use shortcut_api::repositories::MySqlClient;
use shortcut_api::repositories::links::Repository;
use shortcut_api::repositories::links::ShortcutRepository;

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
