use crate::application::services::department_service::DepartmentService;
use crate::common::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};
use crate::application::dto::department_dto::{ReqCreate, UpdateInfo, RespInfo, RespList};
use crate::infrastructure::security::identity::Identity;
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use std::collections::HashMap;
use axum_extra::extract::WithRejection;
use validator::Validate;
use crate::common::tree;
use std::sync::Arc;

pub struct DepartmentController;


impl DepartmentController {

    pub fn new() -> Self {
        Self 
    }

    
    pub async fn create( 
        Extension(service): Extension<Arc<DepartmentService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
       service.create(req).await
    }

    pub async fn info(
        Extension(service): Extension<Arc<DepartmentService>>,
        Extension(identity): Extension<Identity>,
        Path(department_id): Path<i64>,
    ) -> Result<ApiOK<RespInfo>> {
        service.info(department_id).await
    }

    pub async fn list(
        Extension(service): Extension<Arc<DepartmentService>>,
        Extension(identity): Extension<Identity>,
        Query(query): Query<HashMap<String, String>>,
    ) -> Result<ApiOK<RespList>> {
    
        service.list(query).await
    }


    pub async fn update(
        Extension(service): Extension<Arc<DepartmentService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.update(req).await
    }
    
    pub async fn delete(
        Extension(service): Extension<Arc<DepartmentService>>,
        Extension(identity): Extension<Identity>,
        Path(department_id): Path<i64>,
    ) -> Result<ApiOK<()>>  {
        service.delete(department_id).await
    }
    
    
    pub async fn select_list(
        Extension(service): Extension<Arc<DepartmentService>>,
        Extension(identity): Extension<Identity>
    ) -> Result<ApiOK<Vec<tree::TreeNode>>>{
        service.select_list().await
    }
}