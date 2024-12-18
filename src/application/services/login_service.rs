use std::sync::Arc;
use crate::application::dto::login_dto::{ReqLogin, RespLogin};
use crate::infrastructure::repository::login_repository::LoginRepository;
use crate::infrastructure::security::identity::Identity;
use tracing;
use crate::common::result::response::{ApiOK, Result};


pub struct LoginService {
    repository: Arc<LoginRepository>,
}

impl LoginService {

    pub fn new() -> Self {
        Self {
            repository: Arc::new(LoginRepository::new())
        }
    }   

    pub async fn login(&self, req: ReqLogin) -> Result<ApiOK<RespLogin>> {
        tracing::info!("Login request: {}", req.username);
        self.repository.login(req).await
    }

    pub async fn logout(&self, identity: Identity) -> Result<ApiOK<()>> {
        tracing::info!("identity request: {}", identity.id());
        self.repository.logout(identity).await
    }
}
