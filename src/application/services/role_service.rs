use std::collections::HashMap;
use tracing;

use crate::application::dto::role_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelect, RespEmpList, RespRoleMenu};
use crate::common::{
    result::response::{ApiOK, Result},
    tree,
};

use std::sync::Arc;
use crate::infrastructure::repository::role_repository::RoleRepository;

pub struct RoleService {
    repository: Arc<RoleRepository>,
}

impl RoleService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(RoleRepository::new())
        }
    }


    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
        tracing::info!("Creating role: {}", req.rolename);
        self.repository.create(req).await
    }

    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        tracing::info!("Fetching role list");
        self.repository.list(query).await
    }

    pub async fn info(&self, roleid: u64) -> Result<ApiOK<RespInfo>> {
        tracing::info!("Fetching role info: {}", roleid);
        self.repository.info(roleid).await
    }

    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
        tracing::info!("Updating role: {}", req.rolename);
        self.repository.update(req).await
    }

    pub async fn delete(&self, roleid: u64) -> Result<ApiOK<()>> {
        tracing::info!("Deleting role: {}", roleid);
        self.repository.delete(roleid).await
    }

    
    pub async fn select_list(&self) -> Result<ApiOK<Vec<RespSelect>>> {
        tracing::info!("Fetching select list");
        self.repository.select_list().await
    }

    pub async fn role_emp_list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespEmpList>> {
        tracing::info!("Fetching role employee list");
        self.repository.role_emp_list(query).await
    }

    pub async fn menu_list(&self) -> Result<ApiOK<Vec<tree::TreeNode>>> {
        tracing::info!("Fetching menu list");
        self.repository.menu_list().await
    }

    pub async fn role_menu(&self, roleid: i64) -> Result<ApiOK<Vec<RespRoleMenu>>> {
        tracing::info!("Fetching role menu: {}", roleid);
        self.repository.role_menu(roleid).await
    }
}
