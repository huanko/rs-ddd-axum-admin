use serde::{Deserialize, Serialize};
use validator::Validate;






/** 封装添加数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate {
    #[validate(length(min = 1, message = "角色名称必填"))]
    pub rolename: String,
    #[validate(length(min = 1, message = "角色编码必填"))]
    pub rolecode: String,
    pub remark: String,
}

/** 封装返回数据对象 */
#[derive(Debug, Serialize)]
pub struct RespInfo {
    pub roleid: i64,
    pub rolename: String,
    pub rolecode: String,
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
    pub roleid: i64,
    #[validate(length(min = 1, message = "角色名称必填"))]
    pub rolename: String,
    #[validate(length(min = 1, message = "角色编码必填"))]
    pub rolecode: String,
    pub remark: String,
    pub create_time: i64,
    pub create_time_str: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct RespSelect {
    pub roleid: i64,
    pub rolename: String,
}


#[derive(Debug, Serialize)]
pub struct RespEmpInfo{
    pub employee_id:i64,
    pub realname:String,
    pub phone:String,
    pub department_id:i64,
    pub login_name: String,
    pub email: String,
    pub gender:u8,
    pub disabled_flag:u8,
    pub position_id:i64,
    pub create_time:i64,
    pub create_time_str:String,
}


/** 返回列表数据对象 */
#[derive(Debug, Serialize)]
pub struct RespEmpList {
    pub total: i64,
    pub list: Vec<RespEmpInfo>,
}


/** 返回列表数据对象 */
#[derive(Debug, Serialize)]
pub struct RespMenuSelect {
    pub menu_id: i64,
    pub menu_name: String,
    pub parent_id: i64,
}



#[derive(Debug, Serialize)]
pub struct RespRoleMenu {
    pub role_id: i64,
    pub menu_id: i64,
}