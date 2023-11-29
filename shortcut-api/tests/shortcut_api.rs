use shortcut_api::services::shortcut::proto::{shortcut_client::ShortcutClient, CreateRequest};

#[tokio::test]
async fn test_shortcut_create() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ShortcutClient::connect("http://[::1]:1192").await?;

    let request = tonic::Request::new(CreateRequest{
        url: "https://guni1192.com".into(),
        name: "guni".into(),
    });

    let response = client.create(request).await?;
    let response = response.into_inner();

    let link = response.link.clone().expect("Shortcut::Create has no link");
    assert_eq!(link.url, "https://guni1192.com".to_string());
    assert_eq!(link.name, "guni".to_string());

    Ok(())
}
