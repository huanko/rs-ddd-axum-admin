use serde::{Deserialize, Serialize};
use validator::Validate;





/** 封装添加数据对象 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqCreate{
    #[validate(length(min = 1, message = "员工姓名必填"))]
    pub realname:String,
    #[validate(length(min = 1, message = "手机号码必填"))]
    pub phone:String,
    pub department_id:i64,
    #[validate(length(min = 1, message = "登录名必填"))]
    pub login_name: String,
    #[validate(length(min = 1, message = "邮箱必填"))]
    pub email: String,
    pub gender:u8,
    pub disabled_flag:u8,
    pub position_id:i64,

}


#[derive(Debug, Serialize)]
pub struct RespInfo{
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
pub struct RespList {
    pub total: i64,
    pub list: Vec<RespInfo>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UpdateInfo{

    pub employee_id:i64,

    #[validate(length(min = 1, message = "员工姓名必填"))]
    pub realname:String,
    #[validate(length(min = 1, message = "手机号码必填"))]
    pub phone:String,
    pub department_id:i64,
    #[validate(length(min = 1, message = "登录名必填"))]
    pub login_name: String,
    #[validate(length(min = 1, message = "邮箱必填"))]
    pub email: String,
    pub gender:u8,
    pub disabled_flag:u8,
    pub position_id:i64,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespEmpInfo {
    pub employee_id: i64,
    pub realname: String,
    pub department_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespDeptInfo {
    pub department_id: i64,
    pub department_name: String,
}


// 临时存储人员下拉数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespSelectOption {
    pub employee_id: i64,
    pub realname: String,
    pub department_id: i64,
    pub department_name: String,
}
