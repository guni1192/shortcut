use clap::Parser;
use shortcut_api::cli::Cli;
use shortcut_api::middlewares::db::postgres::pg_connect;
use shortcut_api::repositories::links::ScLinkRepository;
use shortcut_api::services::shortcut::{proto::shortcut_server::ShortcutServer, ShortcutService};
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.bind_address, cli.port)
        .parse()
        .unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Setup shortcut service ...");
    let shortcut = ShortcutService::new(ScLinkRepository::new(
        pg_connect(&cli.database_url)
            .await
            .expect("failed to connect database"),
    ));

    info!("Starting server at {}", addr);
    Server::builder()
        .trace_fn(|_| tracing::info_span!("shortcut_api_server"))
        .add_service(ShortcutServer::new(shortcut))
        .serve(addr)
        .await?;

    Ok(())
}
