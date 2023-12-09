use shortcut_api::services::shortcut::proto::{shortcut_client::ShortcutClient, CreateRequest};

#[tokio::test]
async fn test_shortcut_create() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ShortcutClient::connect("http://localhost:1192").await?;

    let name: String = uuid::Uuid::new_v4().to_string();

    let request = tonic::Request::new(CreateRequest {
        url: "https://guni1192.com".into(),
        name: name.clone(),
    });

    let response = client.create(request).await?;
    let response = response.into_inner();

    let link = response.link.clone().expect("Shortcut::Create has no link");

    dbg!(&link);
    assert_eq!(link.id.len(), 36);
    assert_eq!(link.url, "https://guni1192.com".to_string());
    assert_eq!(link.name, name);
    assert!(link.created_at.is_some());
    assert!(link.updated_at.is_some());

    // twice create should return error
    let request = tonic::Request::new(CreateRequest {
        url: "https://guni1192.com".into(),
        name: name.clone(),
    });
    let err = client.create(request).await.unwrap_err();
    dbg!(&err);

    Ok(())
}
