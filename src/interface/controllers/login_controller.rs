use std::sync::Arc;
use axum::{Extension, Json};
use crate::application::services::login_service::LoginService;
use crate::common::result::response::{ApiOK, Result};
use crate::application::dto::login_dto::{ReqLogin, RespLogin};
use crate::infrastructure::security::identity::Identity;
use axum_extra::extract::WithRejection;
use validator::Validate;
use crate::common::result::rejection::IRejection;
use crate::common::result::response::ApiErr;






pub struct LoginController;

impl LoginController {

    pub fn new() -> Self {
        Self
    }   

    pub async fn login(
        Extension(service): Extension<Arc<LoginService>>,
        WithRejection(Json(req), _): IRejection<Json<ReqLogin>>,
    ) -> Result<ApiOK<RespLogin>> {
        tracing::info!("Login attempt for user: {}", req.username);
        tracing::info!("Login attempt for password: {}", req.password);

        if let Err(e) = req.validate() {
            return Err(ApiErr::ErrParams(Some(e.to_string())));
        }
        service.login(req).await
    }
    
    pub async fn logout(
        Extension(service): Extension<Arc<LoginService>>,
        Extension(identity): Extension<Identity>) -> Result<ApiOK<()>> {
        if identity.id() == 0 {
            return Ok(ApiOK(None));
        }
        service.logout(identity).await
    }
    
}
