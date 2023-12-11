use shortcut_api::services::shortcut::proto::{
    shortcut_client::ShortcutClient, CreateRequest, FindByNameRequest, ListRequest,
};

pub struct ShortcutAPIClient {
    client: ShortcutClient<tonic::transport::Channel>,
}

impl ShortcutAPIClient {
    pub async fn new(shortcut_api_url: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = ShortcutClient::connect(shortcut_api_url).await?;
        Ok(Self { client })
    }

    pub async fn create(
        &mut self,
        name: &str,
        url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(CreateRequest {
            name: name.to_string(),
            url: url.to_string(),
        });

        let response = self.client.create(request).await?;
        let link = response.into_inner().link.unwrap();

        println!("Link={:?}", link);

        Ok(())
    }

    pub async fn list(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(ListRequest {});

        let response = self.client.list(request).await?;

        use prettytable::{format, row, Table};
        let mut table = Table::new();

        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.add_row(row!["name", "url"]);

        for link in response.into_inner().links {
            table.add_row(row![link.name, link.url]);
        }
        table.printstd();

        Ok(())
    }

    pub async fn find_by_name(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = tonic::Request::new(FindByNameRequest {
            name: name.to_string(),
        });

        let response = self.client.find_by_name(request).await?;
        let link = response.into_inner().link.unwrap();

        println!("Link={:?}", link);

        Ok(())
    }
}
