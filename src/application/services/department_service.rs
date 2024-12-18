use std::collections::HashMap;
use tracing;

use crate::infrastructure::repository::department_repository::DepartmentRepository;
use crate::application::dto::department_dto::{ReqCreate, UpdateInfo, RespInfo, RespList};
use crate::common::{
    result::response::{ApiOK, Result},
    tree,
};

use std::sync::Arc;

pub struct DepartmentService {
    repository: Arc<DepartmentRepository>,  // 使用 Arc 包装以支持共享
}

impl DepartmentService {

    pub fn new() -> Self {
        Self {
            repository: Arc::new(DepartmentRepository::new())
        }
    }
   
    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
        tracing::info!("Creating department: {}", req.deptname);
        self.repository.create(req).await
    }

    pub async fn select_list(&self) -> Result<ApiOK<Vec<tree::TreeNode>>> {
        tracing::info!("Fetching department tree");
        self.repository.select_list().await
    }

    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        tracing::info!("Listing departments with query: {:?}", query);
        self.repository.list(query).await
    }

    pub async fn info(&self, department_id: i64) -> Result<ApiOK<RespInfo>> {
        tracing::info!("Fetching department info: {}", department_id);
        self.repository.info(department_id).await
    }

    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
        tracing::info!("Updating department: {}", req.deptid);
        self.repository.update(req).await
    }

    pub async fn delete(&self, department_id: i64) -> Result<ApiOK<()>> {
        tracing::info!("Deleting department: {}", department_id);
        self.repository.delete(department_id).await
    }
}
