use shortcut_api::services::shortcut::{proto::shortcut_server::ShortcutServer, ShortcutService};
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "0.0.0.0:1192".parse().unwrap();
    let shortcut = ShortcutService::default();

    info!("Starting server at {}", addr);
    Server::builder()
        .add_service(ShortcutServer::new(shortcut))
        .serve(addr)
        .await?;

    Ok(())
}
