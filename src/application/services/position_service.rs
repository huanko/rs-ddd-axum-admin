use std::collections::HashMap;
use tracing;
use crate::infrastructure::repository::position_repository::PositionRepository;
use crate::application::dto::position_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelect};
use crate::common::{
    result::response::{ApiOK, Result}
};
use std::sync::Arc;





pub struct PositionService {
    repository: Arc<PositionRepository>
}

impl PositionService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(PositionRepository::new())
        }
    }

    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
        tracing::info!("Creating position: {}", req.postname);
        self.repository.create(req).await
    }

    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        tracing::info!("Listing positions with query: {:?}", query);
        self.repository.list(query).await
    }

    pub async fn info(&self, postid: u64) -> Result<ApiOK<RespInfo>> {
        tracing::info!("Fetching position info: {}", postid);
        self.repository.info(postid).await
    }

    pub async fn select_list(&self) -> Result<ApiOK<Vec<RespSelect>>> {
        tracing::info!("Fetching position select list");
        self.repository.select_list().await
    }   

    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
        tracing::info!("Updating position: {}", req.postname);
        self.repository.update(req).await
    }

    pub async fn delete(&self, postid: u64) -> Result<ApiOK<()>> {
        tracing::info!("Deleting position: {}", postid);
        self.repository.delete(postid).await
    }   

}
