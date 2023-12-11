use clap::Parser;
use scctl::cli::{Cli, SubCommand};
use scctl::client::ShortcutAPIClient;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut shortcut = ShortcutAPIClient::new(cli.shortcut_api_url).await.unwrap();

    match cli.subcmd {
        SubCommand::Create(create) => {
            shortcut
                .create(&create.name, &create.url)
                .await
                .unwrap();
        }
        SubCommand::Delete(delete) => {
            println!("Delete: {:?}", delete);
        }
        SubCommand::Get(get) => {
            match get.name {
                Some(name) => {
                    shortcut.find_by_name(&name).await.unwrap();
                }
                None => {
                    shortcut.list().await.unwrap();
                }
            }
        }
        SubCommand::Update(update) => {
            println!("Update: {:?}", update);
        }
    }
}
