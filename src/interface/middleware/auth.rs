use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::infrastructure::security::identity::Identity;
use crate::interface::auth_check;
use crate::common::result::response::ApiErr;


pub async fn handle(request: Request, next: Next) -> Response {
    let identity = request.extensions().get::<Identity>();
    match identity {
        None => return ApiErr::ErrAuth(None).into_response(),
        Some(v) => match auth_check(v).await {
            Ok(_) => (),
            Err(e) => return ApiErr::ErrAuth(Some(e.to_string())).into_response(),
        },
    }
    next.run(request).await
}


