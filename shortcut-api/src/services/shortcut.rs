use prost_types::Timestamp;
use proto::shortcut_server::Shortcut;
use proto::Link;
use sqlx::types::chrono;

use crate::repositories::links::{LinkRepository, ScLinkRepository};

pub mod proto {
    tonic::include_proto!("shortcut");
}

pub struct ShortcutService {
    repository: ScLinkRepository,
}

impl ShortcutService {
    pub fn new(repository: ScLinkRepository) -> Self {
        Self { repository }
    }
}

fn to_prost_timestamp(datetime: chrono::DateTime<chrono::Utc>) -> Timestamp {
    Timestamp {
        seconds: datetime.timestamp(),
        nanos: datetime.timestamp_subsec_nanos() as i32,
    }
}

#[tonic::async_trait]
impl Shortcut for ShortcutService {
    async fn create(
        &self,
        request: tonic::Request<proto::CreateRequest>,
    ) -> Result<tonic::Response<proto::CreateResponse>, tonic::Status> {
        let request = request.into_inner();

        let link = self
            .repository
            .create(&request.name, &request.url)
            .await
            .map_err(|e| tonic::Status::internal(format!("failed to create link: {:?}", e)))?;

        let created_at = to_prost_timestamp(link.created_at);
        let updated_at = to_prost_timestamp(link.updated_at);

        Ok(tonic::Response::new(proto::CreateResponse {
            link: Some(Link {
                id: link.id.to_string(),
                name: link.name,
                url: link.url,
                created_at: Some(created_at),
                updated_at: Some(updated_at),
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
                id: "".to_string(),
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
                id: "".to_string(),
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
