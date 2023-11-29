use shortcut_api::services::shortcut::{proto::shortcut_server::ShortcutServer, ShortcutService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:1192".parse().unwrap();
    let shortcut = ShortcutService::default();

    Server::builder()
        .add_service(ShortcutServer::new(shortcut))
        .serve(addr)
        .await?;

    Ok(())
}
