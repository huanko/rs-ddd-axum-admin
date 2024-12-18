use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use axum::extract::Extension;
use std::sync::Arc;

use crate::application::services::department_service::DepartmentService;
use crate::application::services::login_service::LoginService;
use crate::application::services::role_service::RoleService;
use crate::application::services::position_service::PositionService;
use crate::application::services::employee_service::EmployeeService;
use crate::interface::middleware::auth;
use crate::interface::middleware::log;
use crate::interface::middleware::identity;
use crate::interface::middleware::cors;
use crate::interface::middleware::req_id;

use crate::interface::controllers::department_controller::DepartmentController as department;
use crate::interface::controllers::login_controller::LoginController as login;
use crate::interface::controllers::role_controller::RoleController as role;
use crate::interface::controllers::position_controller::PositionController as position;
use crate::interface::controllers::employee_controller::EmployeeController as employee;

pub fn init() -> Router {
    

    let department_service = Arc::new(DepartmentService::new());
    let login_service = Arc::new(LoginService::new());
    let role_service = Arc::new(RoleService::new());
    let position_service = Arc::new(PositionService::new());
    let employee_service = Arc::new(EmployeeService::new());


     // 开放
     let open = Router::new().route("/login", post(login::login))
     .route("/logout", post(login::logout))
     .layer(Extension(login_service));


    // 需要鉴权的路由
    let auth = Router::new()
        .nest("/api", api_routes(department_service, role_service, position_service, employee_service))
        .layer(axum::middleware::from_fn(auth::handle));

        Router::new()
        .route("/", get(|| async { "☺ welcome to Rust app" }))
        .nest("/v1", open.merge(auth))
        .layer(axum::middleware::from_fn(log::handle)) // 请求日志
        .layer(axum::middleware::from_fn(identity::handle))// 请求身份验证
        .layer(axum::middleware::from_fn(cors::handle))// 请求跨域
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let req_id = match request
                    .headers()
                    .get("x-request-id")
                    .and_then(|value| value.to_str().ok())
                {
                    Some(v) => v.to_string(),
                    None => String::from("unknown"),
                };

                tracing::error_span!("request_id", id = req_id)
            }),
        )
        .layer(axum::middleware::from_fn(req_id::handle))
}

// API 路由组
fn api_routes(
    service: Arc<DepartmentService>, 
    role_service: Arc<RoleService>,
    position_service: Arc<PositionService>,
    employee_service: Arc<EmployeeService>) -> Router {
    Router::new()
        // 部门相关路由
        .nest("/departments", department_routes(service))
        // 角色相关路由
        .nest("/roles", role_routes(role_service))
        // 职位相关路由
        .nest("/positions", position_routes(position_service))
        // 员工相关路由
        .nest("/employees", employee_routes(employee_service))
}

// 部门路由
fn department_routes(service: Arc<DepartmentService>) -> Router {
    Router::new()
        .route("/departments", get(department::list).post(department::create))
        .route("/departments/:department_id", get(department::info).delete(department::delete))
        .route("/departments/update", post(department::update))
        .route("/departments/select_list", get(department::select_list))
        .layer(Extension(service))
}

// 角色路由
fn role_routes(service: Arc<RoleService>) -> Router {
    Router::new()
    .route("/roles", get(role::list).post(role::create))  
    .route("/roles/:role_id", get(role::info).delete(role::delete))
    .route("/roles/update", post(role::update))
    .route("/roles/select_list", get(role::select_list))
    .route("/roles/role_emp_list", get(role::role_emp_list))
    .route("/roles/role_func_list", get(role::role_func_list))
    .route("/roles/role_func_id", get(role::role_func_id))
    .layer(Extension(service))
}

// 职位路由
fn position_routes(service: Arc<PositionService>) -> Router {
    Router::new()
    .route("/positions", get(position::list).post(position::create))
    .route("/positions/:post_id", get(position::info).delete(position::delete))
    .route("/positions/update", post(position::update))
    .route("/positions/select_list", get(position::select_list))
    .layer(Extension(service))
}

// 员工路由
fn employee_routes(service: Arc<EmployeeService>) -> Router {
    Router::new()
    .route("/employees", get(employee::list).post(employee::create))
    .route("/employees/:employee_id", get(employee::info))
    .route("/employees/update", post(employee::update))
    .route("/employees/disabled_flag/:employee_id/:disabled_flag", get(employee::disabled_flag))
    .route("/employees/reset_password/:employee_id", get(employee::reset_password))
    .route("/employees/change_department/:employee_ids/:department_id", get(employee::change_department))
    .route("/employees/employee_select_list", get(employee::employee_select_list))
    .layer(Extension(service))
}

