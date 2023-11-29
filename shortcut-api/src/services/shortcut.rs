use proto::shortcut_server::Shortcut;
use proto::Link;

pub mod proto {
    tonic::include_proto!("shortcut");
}

#[derive(Default)]
pub struct ShortcutService {}

#[tonic::async_trait]
impl Shortcut for ShortcutService {
    async fn create(
        &self,
        request: tonic::Request<proto::CreateRequest>,
    ) -> Result<tonic::Response<proto::CreateResponse>, tonic::Status> {
        let request = request.into_inner();
        Ok(tonic::Response::new(proto::CreateResponse {
            link: Some(Link {
                name: request.name.clone(),
                url: request.url.clone(),
                created_at: None,
                updated_at: None,
            }),
        }))
    }

    async fn list(
        &self,
        _request: tonic::Request<proto::ListRequest>,
    ) -> Result<tonic::Response<proto::ListResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(proto::ListResponse { links: vec![] }))
    }

    async fn show(
        &self,
        _request: tonic::Request<proto::ShowRequest>,
    ) -> Result<tonic::Response<proto::ShowResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(proto::ShowResponse {
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
        request: tonic::Request<proto::UpdateRequest>,
    ) -> Result<tonic::Response<proto::UpdateResponse>, tonic::Status> {
        let request = request.into_inner();

        Ok(tonic::Response::new(proto::UpdateResponse {
            link: Some(Link {
                name: request.name.to_string(),
                url: request.url.to_string(),
                created_at: None,
                updated_at: None,
            }),
        }))
    }

    async fn delete(
        &self,
        _request: tonic::Request<proto::DeleteRequest>,
    ) -> Result<tonic::Response<proto::DeleteResponse>, tonic::Status> {
        // let request = request.into_inner();
        Ok(tonic::Response::new(proto::DeleteResponse {}))
    }
}
