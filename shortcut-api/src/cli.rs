use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Cli {
    /// The database URL (e.g. postgres://user:pass@localhost:5432/dbname)
    database_url: String
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        let cli = Cli::parse();
        assert_eq!(cli.database_url, "postgres://user:pass@localhost:5432/dbname");
    }
}
