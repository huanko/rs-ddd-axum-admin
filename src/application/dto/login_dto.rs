use validator::Validate;
use serde::{Deserialize, Serialize};

/** 封装输入参数 */
#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ReqLogin {
    #[validate(length(min = 1, message = "用户名必填"))]
    pub username: String,
    #[validate(length(min = 1, message = "密码必填"))]
    pub password: String,
}


/** 封装返回参数 */
#[derive(Debug, Deserialize, Serialize)]
pub struct RespLogin {
    pub name: String,
    pub role: i64,
    pub auth_token: String,
}