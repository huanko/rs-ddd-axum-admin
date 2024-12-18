use axum::{
    extract::Request,
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS, ACCESS_CONTROL_MAX_AGE,
};

pub async fn handle(request: Request, next: Next) -> Response {
    let mut cors_headers = HeaderMap::new();

    // 允许所有源访问，在生产环境中应该设置具体的域名
    cors_headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    
    // 允许携带认证信息
    cors_headers.insert(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    
    // 允许的 HTTP 方法
    cors_headers.insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
    );
    
    // 允许的请求头
    cors_headers.insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static(
            "Content-Type, Authorization, X-Requested-With, Accept, Origin, Access-Control-Request-Method, Access-Control-Request-Headers"
        ),
    );
    
    // 允许浏览器访问的响应头
    cors_headers.insert(
        ACCESS_CONTROL_EXPOSE_HEADERS,
        HeaderValue::from_static("Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers"),
    );
    
    // 预检请求的缓存时间
    cors_headers.insert(ACCESS_CONTROL_MAX_AGE, HeaderValue::from_static("86400"));

    // 对 OPTIONS 预检请求的处理
    if request.method() == Method::OPTIONS {
        return (StatusCode::NO_CONTENT, cors_headers).into_response();
    }

    // 处理实际请求
    let response = next.run(request).await;
    
    // 将 CORS 头部添加到响应中
    (cors_headers, response).into_response()
}