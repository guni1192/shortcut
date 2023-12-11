use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
};
use hyper::StatusCode;
use shortcut_api::services::shortcut::proto::{shortcut_client::ShortcutClient, FindByNameRequest};

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

    let request = tonic::Request::new(FindByNameRequest {
        name: short_url.clone(),
    });

    let response = client
        .find_by_name(request)
        .await
        .map_err(|e| match e.code() {
            tonic::Code::NotFound => {
                tracing::warn!("Link not found: name={}, Error={:?}", short_url.clone(), e);
                (
                    StatusCode::NOT_FOUND,
                    format!("Link not found: name={}", short_url),
                )
                    .into_response()
            }
            tonic::Code::Internal => {
                tracing::error!("shortcut-api request error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error: shortcut-api is not available".to_string(),
                )
                    .into_response()
            }
            _ => {
                tracing::error!("unknown shortcut-api request error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                )
                    .into_response()
            }
        })?;
    tracing::info!("Response: {:?}", response);
    let response = response.into_inner();

    let not_found = (
        StatusCode::NOT_FOUND,
        format!("Link not found: name={}", short_url),
    )
        .into_response();

    match response.link {
        Some(link) => Ok(Redirect::permanent(&link.url)),
        None => Err(not_found),
    }
}
