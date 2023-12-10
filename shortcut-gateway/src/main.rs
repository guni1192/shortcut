pub use clap::Parser;
pub use shortcut_gateway::cli::Cli;
use shortcut_gateway::handler::ShortcutClientConfig;
pub use shortcut_gateway::server::init_server;
pub use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = init_server(ShortcutClientConfig::new(&cli.shortcut_api_url)).await;
    let addr = format!("{}:{}", cli.bind_address, cli.port);
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
