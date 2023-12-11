use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version)]
pub struct Cli {
    /// shortcut-api server URL
    #[clap(long, env = "SHORTCUT_API_URL")]
    pub shortcut_api_url: String,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Create a new shortcut
    Create(Create),
    /// Delete a shortcut
    Delete(Delete),
    /// List all shortcuts or search for a shortcut
    Get(Get),
    /// Update a shortcut
    Update(Update),
}


#[derive(Parser, Debug)]
pub struct Create {
    /// The shortcut name
    #[clap(short, long)]
    pub name: String,

    /// The shortcut URL
    #[clap(short, long)]
    pub url: String,
}

#[derive(Parser, Debug)]
pub struct Delete {
    /// The shortcut name
    pub name: String,
}

#[derive(Parser, Debug)]
pub struct Get {
    /// The shortcut name
    pub name: Option<String>,
}

#[derive(Parser, Debug)]
pub struct Update {
    /// The shortcut name
    pub name: String,

    /// The shortcut URL
    #[clap(short, long)]
    pub url: String,
}
