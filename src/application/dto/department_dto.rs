
use serde::{Deserialize, Serialize};
use validator::Validate;


/** 封装添加数据对象 */
#[derive(Debug, Validate,  Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "部门名称必填"))]
    pub deptname: String,
    pub sort: i32,
    pub managerid: i64,
    pub parentid: i64,
}

// 封装部门下拉列表数据对象
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct RespSelect {
    pub department_id: i64,
    pub department_name: String,
    pub parent_id: i64,
}

// 封装返回数据对象
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct RespInfo{
    pub department_id: i64,
    pub department_name: String,
    pub manager_id: i64,
    pub parent_id: i64,
    pub sort: i32,
    pub create_time: i64,
    pub create_time_str: String,
}

/** 返回列表数据对象 */
#[derive(Debug, Serialize)]
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

/** 封装修改数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateInfo {
    pub deptid: i64,
    #[validate(length(min = 1, message = "部门名称必填"))]
    pub deptname: String,
    pub sort: i32,
    pub managerid: i64,
    pub parentid: i64,
    pub create_time: i64,
    pub create_time_str: String,
}