use serde::{Deserialize, Serialize};
use validator::Validate;

/** 封装添加数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "职务名称必填"))]
    pub postname: String,
    pub level: String,
    pub sort: i64,
    pub remark: String,
}

/** 封装返回数据对象 */
#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub postid: i64,
    pub postname: String,
    pub level: String,
    pub sort: i64,
    pub remark: String,
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
    pub postid: i64,
    #[validate(length(min = 1, message = "角色名称必填"))]
    pub postname: String,
    pub level: String,
    pub sort: i64,
    pub remark: String,
    pub create_time: i64,
    pub create_time_str: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RespSelect {
    pub postid: i64,
    pub postname: String,
}
