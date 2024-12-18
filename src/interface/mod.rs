pub mod router;
pub mod controllers;
pub mod middleware;

use anyhow::Result;
use anyhow::anyhow;
use sea_orm::EntityTrait;
use crate::infrastructure::security::identity::Identity;




use crate::domain::entities::prelude::TEmployee;
use crate::infrastructure::persistence::database as db;


pub async fn auth_check(identity: &Identity) -> Result<()> {
    tracing::debug!("Checking auth for user_id: {}", identity.id());

    if identity.id() == 0 {
        tracing::warn!("Unauthorized access attempt with empty user_id");
        return Err(anyhow!("未授权，请先登录"));
    }

    match TEmployee::find_by_id(identity.id()).one(db::conn()).await {
        Ok(Some(employee)) => {
            if employee.login_token.is_empty() || !identity.match_token(&employee.login_token) {
                tracing::warn!("Invalid token for user_id: {}", identity.id());
                return Err(anyhow!("授权已失效"));
            } else {
                tracing::debug!("Auth check passed for user_id: {}", identity.id());
                Ok(())
            }
        }
        Ok(None) => {
            tracing::warn!("User not found: {}", identity.id());
            Err(anyhow!("授权账号不存在"))
        }
        Err(e) => {
            tracing::error!("Database error during auth check: {:?}", e);
            Err(anyhow!("授权服务异常"))
        }
    }
}
