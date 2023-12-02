use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    pub bind_address: String,

    #[arg(short, long, default_value = "1192")]
    pub port: u16,

    /// The database URL (e.g. postgres://user:pass@localhost:5432/dbname)
    #[arg(short, long, env = "DATABASE_URL")]
    pub database_url: String
}

