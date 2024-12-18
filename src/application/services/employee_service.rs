use std::sync::Arc;
use tracing;

use crate::infrastructure::repository::employee_repository::EmployeeRepository;
use crate::application::dto::employee_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelectOption};
use crate::common::result::response::{ApiOK, Result};

use std::collections::HashMap;










pub struct EmployeeService {
    repository: Arc<EmployeeRepository>
}

impl EmployeeService {
    pub fn new() -> Self {
        Self {
            repository: Arc::new(EmployeeRepository::new())
        }
    }


    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
        tracing::info!("Creating employee: {}", req.login_name);
        self.repository.create(req).await
    }

    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        tracing::info!("Listing employees with query: {:?}", query);
        self.repository.list(query).await
    }

    pub async fn info(&self, employee_id: i64) -> Result<ApiOK<RespInfo>> {
        tracing::info!("Fetching employee info for ID: {}", employee_id);
        self.repository.info(employee_id).await
    }

    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
        tracing::info!("Updating employee: {}", req.login_name);
        self.repository.update(req).await
    }

    pub async fn reset_password(&self, employee_id: i64) -> Result<ApiOK<()>> {
        tracing::info!("Resetting password for employee ID: {}", employee_id);
        self.repository.reset_password(employee_id).await   
    }

    pub async fn disabled_flag(&self, employee_id: i64, disabled_flag: u8) -> Result<ApiOK<()>> {
        tracing::info!("Disabling employee ID: {}", employee_id);
        self.repository.disabled_flag(employee_id, disabled_flag).await
    }

    pub async fn change_department(&self, employee_id: Vec<i64>, department_id: i64) -> Result<ApiOK<()>> {
        tracing::info!("Changing department for employees: {:?}, to department ID: {}", employee_id, department_id);
        self.repository.change_department(employee_id, department_id).await
    }

    pub async fn employee_select_list(&self) -> Result<ApiOK<Vec<RespSelectOption>>> {
        tracing::info!("Fetching employee select list");
        self.repository.employee_select_list().await
    }

}
