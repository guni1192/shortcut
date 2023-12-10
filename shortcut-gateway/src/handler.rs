use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
};
use hyper::StatusCode;
use shortcut_api::services::shortcut::proto::shortcut_client::ShortcutClient;

#[derive(Clone)]
pub struct ShortcutClientConfig {
    pub shortcut_api_url: String,
}

impl ShortcutClientConfig {
    pub fn new(shortcut_api_url: &str) -> Self {
        Self {
            shortcut_api_url: shortcut_api_url.to_string(),
        }
    }
}

pub async fn redirect(
    State(client_config): State<ShortcutClientConfig>,
    Path(short_url): Path<String>,
) -> Result<Redirect, Response> {
    tracing::info!(
        "Connecting to shortcut-api: {}",
        client_config.shortcut_api_url
    );
    let mut client = ShortcutClient::connect(client_config.shortcut_api_url)
        .await
        .map_err(|e| {
            tracing::error!("shortcut-api connection error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error: shortcut-api connection error".to_string(),
            )
                .into_response()
        })?;
    let request = tonic::Request::new(shortcut_api::services::shortcut::proto::ListRequest {});

    let response = client.list(request).await.map_err(|e| {
        tracing::error!("shortcut-api request error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error: shortcut-api is not available".to_string(),
        )
            .into_response()
    })?;
    tracing::info!("Response: {:?}", response);
    let response = response.into_inner();

    let not_found = (
        StatusCode::NOT_FOUND,
        format!("Link not found: name={}", short_url),
    )
        .into_response();

    let link = response.links.iter().find(|link| link.name == short_url);

    match link {
        Some(link) => Ok(Redirect::permanent(&link.url)),
        None => Err(not_found),
    }
}
