// src/infrastructure/security/identity.rs
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, encode, 
    DecodingKey, EncodingKey, 
    Header, Validation,
    errors::ErrorKind as JwtErrorKind
};
use std::fmt;
use thiserror::Error;
use time::{Duration, OffsetDateTime};
use tracing::{info, warn};

use crate::common::config;

// JWT 相关常量
const TOKEN_EXPIRE_HOURS: i64 = 24;
const TOKEN_ISSUER: &str = "your_app_name";

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Token is invalid")]
    InvalidToken,
    #[error("Token has expired")]
    TokenExpired,
    #[error("Token validation failed: {0}")]
    ValidationFailed(String),
    #[error("Failed to create token: {0}")]
    TokenCreationFailed(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    id: i64,
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Token 过期时间 (Unix timestamp)
    pub exp: i64,
    /// Token 签发时间 (Unix timestamp)
    pub iat: i64,
    /// Token 签发者
    pub iss: String,
    /// 用户 ID
    pub sub: i64,
    /// Token 类型
    #[serde(default = "default_token_type")]
    pub typ: String,
}

fn default_token_type() -> String {
    "access".to_string()
}

impl Identity {
    pub fn new(id: i64, token: impl Into<String>) -> Self {
        Self {
            id,
            token: token.into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: 0,
            token: String::new(),
        }
    }

    /// 从认证令牌创建身份
    pub fn from_auth_token(token: impl Into<String>) -> Result<Self> {
        let token = token.into();
        let secret = get_secret()?;
        
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ).map_err(|e| match *e.kind() {
            JwtErrorKind::ExpiredSignature => AuthError::TokenExpired,
            _ => AuthError::InvalidToken,
        })?;

        Ok(Self::new(token_data.claims.sub, token))
    }

    /// 生成认证令牌
    pub fn to_auth_token(&self) -> Result<String> {
        let now = OffsetDateTime::now_utc();
        let expire = now + Duration::hours(TOKEN_EXPIRE_HOURS);

        let claims = Claims {
            iat: now.unix_timestamp(),
            exp: expire.unix_timestamp(),
            iss: TOKEN_ISSUER.to_string(),
            sub: self.id,
            typ: default_token_type(),
        };

        let secret = get_secret()?;
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|e| AuthError::TokenCreationFailed(e.to_string()).into())
    }

    /// 获取用户 ID
    pub fn id(&self) -> i64 {
        self.id
    }

    /// 验证令牌是否匹配
    pub fn match_token(&self, token: impl AsRef<str>) -> bool {
        self.token == token.as_ref()
    }

    /// 验证令牌是否有效
    pub fn validate_token(&self) -> Result<bool> {
        if self.token.is_empty() {
            return Ok(false);
        }

        match Self::from_auth_token(&self.token) {
            Ok(identity) => Ok(identity.id == self.id),
            Err(e) => {
                warn!("Token validation failed: {}", e);
                Ok(false)
            }
        }
    }
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.id == 0 {
            write!(f, "<none>")
        } else {
            write!(f, "id:{}|token:{}", self.id, self.token)
        }
    }
}

// 辅助函数
fn get_secret() -> Result<String> {
    config::global()
        .get_string("app.secret")
        .context("Failed to get JWT secret from config")
}

#[cfg(test)]
mod tests {
    use super::*;

     // 移除 async，因为没有异步操作
      fn setup_test_env() -> Result<tempfile::NamedTempFile> {
        let test_config = r#"
            app:
              secret: "test_secret"
              name: "test_app"
              env: "test"
        "#;

        // 创建临时配置文件
        let mut temp_file = tempfile::NamedTempFile::new()?;
        std::io::Write::write_all(&mut temp_file, test_config.as_bytes())?;

        // 初始化配置
        let cfg_path = temp_file.path().to_str().unwrap();
        config::init(cfg_path);
        
        // 返回 temp_file 以确保其生命周期
        Ok(temp_file)
    }

    #[test]
     fn test_token_lifecycle() -> Result<()> {
        // 保持临时文件的引用
        let _temp_file = setup_test_env()?;

        let user_id = 1;
        
        // 生成 token
        let token = {
            let temp_identity = Identity::new(user_id, "");
            let token = temp_identity.to_auth_token()?;         
            assert!(!token.is_empty(), "Generated token should not be empty");
            token
        };

        // 使用生成的 token 创建 Identity
        let identity = Identity::new(user_id, &token);
        
        // 验证 identity 的完整性
        assert_eq!(identity.id(), user_id, "User ID should match");
        assert!(identity.match_token(&token), "Token should match");
        assert!(identity.validate_token()?, "Token should be valid");
        
        // 验证解码后的 token
        let decoded = Identity::from_auth_token(&token)?;
        assert_eq!(decoded.id(), user_id, "Decoded user ID should match");
        assert!(decoded.match_token(&token), "Decoded token should match");
        assert!(decoded.validate_token()?, "Decoded token should be valid");

        Ok(())
    }

    #[test]
    fn test_empty_identity() {
        let identity = Identity::empty();
        assert_eq!(identity.id(), 0);
        assert!(identity.token.is_empty());
        assert_eq!(identity.to_string(), "<none>");
    }

    #[tokio::test]
    async fn test_token_validation() -> Result<()> {
        setup_test_env()?;

        let identity = Identity::new(1, "invalid_token");
        assert!(!identity.validate_token()?);

        let token = identity.to_auth_token()?;
        let valid_identity = Identity::new(1, token);
        assert!(valid_identity.validate_token()?);

        Ok(())
    }
}