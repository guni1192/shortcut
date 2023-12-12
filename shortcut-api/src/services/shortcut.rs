use prost_types::Timestamp;
use proto::shortcut_server::Shortcut;
use proto::Link;
use sqlx::{
    types::chrono,
    Error::{Database, RowNotFound},
};
use tracing::{info, warn};

use crate::repositories::links::{LinkRepository, ScLinkRepository};

pub mod proto {
    tonic::include_proto!("shortcut");
}

#[derive(Debug)]
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
    #[tracing::instrument(skip(self, request))]
    async fn create(
        &self,
        request: tonic::Request<proto::CreateRequest>,
    ) -> Result<tonic::Response<proto::CreateResponse>, tonic::Status> {
        let request = request.into_inner();

        let link = self
            .repository
            .create(&request.name, &request.url)
            .await
            .map_err(|e| match e {
                Database(pg_err) if pg_err.kind() == sqlx::error::ErrorKind::UniqueViolation => {
                    tonic::Status::already_exists(format!(
                        "link name \"{}\" already exists",
                        request.name
                    ))
                }
                e => {
                    warn!("failed to create link: {:?}, request: {:?}", e, request);
                    tonic::Status::internal(format!("failed to create link: {:?}", e))
                }
            })?;

        info!("Shortcut::Create: {:?}", link);

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
        let links = self.repository.list().await.map_err(|e| {
            warn!("failed to list links: {:?}", e);
            tonic::Status::internal(format!("failed to list links: {:?}", e))
        })?;

        let links: Vec<Link> = links
            .iter()
            .map(|link| {
                let created_at = to_prost_timestamp(link.created_at);
                let updated_at = to_prost_timestamp(link.updated_at);

                Link {
                    id: link.id.to_string(),
                    name: link.name.clone(),
                    url: link.url.clone(),
                    created_at: Some(created_at),
                    updated_at: Some(updated_at),
                }
            })
            .collect();

        Ok(tonic::Response::new(proto::ListResponse { links }))
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

    async fn find_by_name(
        &self,
        request: tonic::Request<proto::FindByNameRequest>,
    ) -> Result<tonic::Response<proto::FindByNameResponse>, tonic::Status> {
        let request = request.into_inner();

        let link = self
            .repository
            .find_by_name(&request.name)
            .await
            .map_err(|e| {
                warn!("failed to find link by name: {:?}", e);
                tonic::Status::not_found(format!("failed to find link by name: {:?}", e))
            })?;

        let created_at = to_prost_timestamp(link.created_at);
        let updated_at = to_prost_timestamp(link.updated_at);

        Ok(tonic::Response::new(proto::FindByNameResponse {
            link: Some(Link {
                id: link.id.to_string(),
                name: link.name,
                url: link.url,
                created_at: Some(created_at),
                updated_at: Some(updated_at),
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
        request: tonic::Request<proto::DeleteRequest>,
    ) -> Result<tonic::Response<proto::DeleteResponse>, tonic::Status> {
        let request = request.into_inner();
        self.repository
            .delete(&request.name)
            .await
            .map_err(|e| match e {
                RowNotFound => {
                    warn!("link name \"{}\" is not found: Error={:?}", request.name, e);
                    tonic::Status::not_found(format!("link name \"{}\" not found", request.name))
                }
                _ => { 
                    warn!("failed to delete link \"{}\" : Error={:?}", request.name, e);
                    tonic::Status::internal(format!("failed to delete link \"{}\": {:?}", request.name, e)) 
                },
            })?;

        Ok(tonic::Response::new(proto::DeleteResponse {}))
    }
}
