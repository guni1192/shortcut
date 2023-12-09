use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind_address: String,

    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// shortcut-api server address
    #[clap(long)]
    pub shortcut_api_url: String,
}
