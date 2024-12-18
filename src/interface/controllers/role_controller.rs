use std::collections::HashMap;
use axum_extra::extract::WithRejection;
use validator::Validate;
use crate::common::tree;
use std::sync::Arc;
use crate::application::services::role_service::RoleService;
use crate::infrastructure::security::identity::Identity;
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use crate::common::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result},
};
use crate::application::dto::role_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelect, RespEmpList, RespRoleMenu};










pub struct RoleController;

impl RoleController {
    pub fn new() -> Self {
        Self
    }

   
    pub async fn create(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.create(req).await
    }
    
    pub async fn info(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        Path(role_id): Path<u64>,
    ) -> Result<ApiOK<RespInfo>> {
    
        service.info(role_id).await
    }
    
    pub async fn list(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        Query(query): Query<HashMap<String, String>>,
    ) -> Result<ApiOK<RespList>> {
    
        service.list(query).await
    }
    
    
    pub async fn update(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.update(req).await
    }
    
    pub async fn delete(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        Path(role_id): Path<u64>,
    ) -> Result<ApiOK<()>>  {
    
        service.delete(role_id).await
    }
    
    
    pub async fn select_list(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>
    ) -> Result<ApiOK<Vec<RespSelect>>> {
        service.select_list().await
    }
    
    
    // 根据角色Id查询对应角色下的员工列表,参数包含角色Id、员工姓名、员工手机号、登录名
    pub async fn role_emp_list(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        Query(query): Query<HashMap<String, String>>
    ) -> Result<ApiOK<RespEmpList>> {
        service.role_emp_list(query).await
    }
    
    //功能权限-查询所有功能权限
    pub async fn role_func_list(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
    ) -> Result<ApiOK<Vec<tree::TreeNode>>>{
        service.menu_list().await
    }
    
    //功能权限-根据角色Id查询对应角色下的功能ID列表
    pub async fn role_func_id(
        Extension(service): Extension<Arc<RoleService>>,
        Extension(identity): Extension<Identity>,
        Path(role_id): Path<i64>
    ) -> Result<ApiOK<Vec<RespRoleMenu>>>{
        service.role_menu(role_id).await
    }
    
}

