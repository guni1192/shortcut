use shortcut::shortcut_server::Shortcut;
use shortcut::Link;

pub mod shortcut {
    tonic::include_proto!("shortcut");
}

#[derive(Default)]
pub struct ShortcutService {}

#[tonic::async_trait]
impl Shortcut for ShortcutService {
    async fn create(
        &self,
        _request: tonic::Request<shortcut::CreateRequest>,
    ) -> Result<tonic::Response<shortcut::CreateResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(shortcut::CreateResponse {
            link: Some(Link {
                name: "alias".to_string(),
                url: "url".to_string(),
                created_at: None,
                updated_at: None,
            }),
        }))
    }

    async fn list(
        &self,
        _request: tonic::Request<shortcut::ListRequest>,
    ) -> Result<tonic::Response<shortcut::ListResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(shortcut::ListResponse {
            links: vec![Link {
                name: "alias".to_string(),
                url: "url".to_string(),
                created_at: None,
                updated_at: None,
            }],
        }))
    }

    async fn show(
        &self,
        _request: tonic::Request<shortcut::ShowRequest>,
    ) -> Result<tonic::Response<shortcut::ShowResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(shortcut::ShowResponse {
            link: Some(Link {
                name: "alias".to_string(),
                url: "url".to_string(),
                created_at: None,
                updated_at: None,
            }),
        }))
    }

    async fn update(
        &self,
        _request: tonic::Request<shortcut::UpdateRequest>,
    ) -> Result<tonic::Response<shortcut::UpdateResponse>, tonic::Status> {
        // let request = request.into_inner();

        Ok(tonic::Response::new(shortcut::UpdateResponse {
            link: Some(Link {
                name: "alias".to_string(),
                url: "url".to_string(),
                created_at: None,
                updated_at: None,
            }),
        }))
    }

    async fn delete(
        &self,
        _request: tonic::Request<shortcut::DeleteRequest>,
    ) -> Result<tonic::Response<shortcut::DeleteResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(shortcut::DeleteResponse {}))
    }
}
