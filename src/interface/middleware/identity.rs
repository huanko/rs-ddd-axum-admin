use axum::{extract::Request, middleware::Next, response::Response};
use axum::http::header::AUTHORIZATION;

use crate::infrastructure::security::identity::Identity;

pub async fn handle(mut request: Request, next: Next) -> Response {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_str| {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())  // 去掉 "Bearer " 前缀
            } else {
                tracing::warn!("Invalid authorization header format");
                None
            }
        });

    let identity = match token {
        Some(token) => {
            match Identity::from_auth_token(token) {
                Ok(identity) => identity,
                Err(e) => {
                    tracing::error!(error = ?e, "Failed to parse identity from token");
                    Identity::empty()
                }
            }
        }
        None => {
            tracing::debug!("No authorization header found");
            Identity::empty()
        }
    };

    request.extensions_mut().insert(identity);
    next.run(request).await
}