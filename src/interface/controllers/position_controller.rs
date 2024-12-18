use std::collections::HashMap;
use std::sync::Arc;
use crate::application::services::position_service::PositionService;
use crate::application::dto::position_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelect};
use crate::common::result::{
    rejection::IRejection,
    response::{ApiErr, ApiOK, Result}
};
use crate::infrastructure::security::identity::Identity;
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use validator::Validate;



pub struct PositionController;

impl PositionController {

    pub fn new() -> Self {
        Self
    }

    pub async fn create(
        Extension(service): Extension<Arc<PositionService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<ReqCreate>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.create(req).await
    }
    
    pub async fn info(
        Extension(service): Extension<Arc<PositionService>>,
        Extension(identity): Extension<Identity>,
        Path(role_id): Path<u64>,
    ) -> Result<ApiOK<RespInfo>> {
    
        service.info(role_id).await
    }
    
    pub async fn list(
        Extension(service): Extension<Arc<PositionService>>,
        Extension(identity): Extension<Identity>,
        Query(query): Query<HashMap<String, String>>,
    ) -> Result<ApiOK<RespList>> {
    
        service.list(query).await
    }
    
    
    pub async fn update(
        Extension(service): Extension<Arc<PositionService>>,
        Extension(identity): Extension<Identity>,
        WithRejection(Json(req), _): IRejection<Json<UpdateInfo>>,
    ) -> Result<ApiOK<()>> {
        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.update(req).await
    }
    
    pub async fn delete(
        Extension(service): Extension<Arc<PositionService>>,
        Extension(identity): Extension<Identity>,
        Path(post_id): Path<u64>,
    ) -> Result<ApiOK<()>>  {
    
        service.delete(post_id).await
    }
    
    
    pub async fn select_list(
        Extension(service): Extension<Arc<PositionService>>,
        Extension(identity): Extension<Identity>
    ) -> Result<ApiOK<Vec<RespSelect>>> {
        service.select_list().await
    }

}   
