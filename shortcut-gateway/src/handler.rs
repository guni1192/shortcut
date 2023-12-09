use axum::{
    extract::{Path, State},
    response::Redirect,
};
use hyper::StatusCode;

#[derive(Clone)]
pub struct ShortcutClientConfig {
    _shortcut_api_url: String,
}

impl ShortcutClientConfig {
    pub fn new(shortcut_api_url: &str) -> Self {
        Self {
            _shortcut_api_url: shortcut_api_url.to_string(),
        }
    }
}

pub async fn redirect(
    State(_client_config): State<ShortcutClientConfig>,
    Path(_short_url): Path<String>,
) -> Result<Redirect, StatusCode> {

    let original_url = "https://www.google.com";

    Ok(Redirect::permanent(original_url))
}
