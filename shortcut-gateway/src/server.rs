use crate::handler::{redirect, ShortcutClientConfig};
use axum::{routing::get, Router};

pub async fn init_server(gateway: ShortcutClientConfig) -> Router {
    Router::new().route("/:short_url", get(redirect).with_state(gateway))
}
