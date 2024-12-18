use std::sync::Arc;
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use crate::application::services::employee_service::EmployeeService;
use crate::common::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};
use crate::application::dto::employee_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelectOption};
use crate::infrastructure::security::identity::Identity;
use axum_extra::extract::WithRejection;
use validator::Validate;
use std::collections::HashMap;








pub struct EmployeeController;


impl EmployeeController {

    pub fn new() -> Self {  
        Self
    }

    pub async fn create(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.create(req).await
    }
    
    pub async fn info(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        Path(employee_id): Path<i64>,
    ) -> Result<ApiOK<RespInfo>> {
    
        service.info(employee_id).await
    }
    
    
    pub async fn list(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        Query(query): Query<HashMap<String, String>>,
    ) -> Result<ApiOK<RespList>> {
    
        service.list(query).await
    }
    
    
    pub async fn update(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.update(req).await
    }
    
    
    pub async fn disabled_flag(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        Path((employee_id, disabled_flag)): Path<(i64,u8)>,
    )-> Result<ApiOK<()>> {
        service.disabled_flag(employee_id, disabled_flag).await
    }
    
    
    pub async fn reset_password(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        Path(employee_id): Path<i64>,
    )-> Result<ApiOK<()>> {
        service.reset_password(employee_id).await
    }
    
    pub async fn change_department(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
        Path((employee_id, department_id)): Path<(Vec<i64>, i64)>,
    )-> Result<ApiOK<()>> {
        service.change_department(employee_id, department_id).await
    }
    
    pub async fn employee_select_list(
        Extension(service): Extension<Arc<EmployeeService>>,
        Extension(identity): Extension<Identity>,
    )-> Result<ApiOK<Vec<RespSelectOption>>> {
        service.employee_select_list().await
    }
}


