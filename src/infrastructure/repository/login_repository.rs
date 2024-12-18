use crate::common::utils;
use crate::infrastructure::persistence::database as db;
use crate::application::dto::login_dto::{ReqLogin, RespLogin};
use crate::common::{
    result::response::{ApiErr, ApiOK, Result},
    xtime
};
use sea_orm::{
    ColumnTrait, EntityTrait, QueryFilter,Set, DatabaseConnection
};

use crate::domain::entities::{
    t_employee, prelude::TEmployee,
    t_role_employee, prelude::TRoleEmployee
};
use sea_orm::sea_query::Expr;

use crate::common::crypto::hash::Crypto;
use time::macros::offset;
use crate::infrastructure::security::identity::Identity;


pub struct LoginRepository {
    conn: DatabaseConnection
}

impl LoginRepository {

    pub fn new() -> Self {
        Self {
            conn: db::conn().clone()
        }
    }


    /**
 * 登录接口
 */
    pub async fn login(&self, req: ReqLogin) -> Result<ApiOK<RespLogin>> {
        /* 根据用户名查询sys_user表，返回用户对象 */
        let  model = TEmployee::find()
            .filter(t_employee::Column::LoginName.eq(req.username))
            .one(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_employee");
                ApiErr::ErrSystem(None)
            })?
            .ok_or(ApiErr::ErrAuth(Some("账号不存在".to_string())))?;

            /* 根据用户ID查询 sys_user_role表，返回用户角色关系表对象 */
            let t_role_employee  = TRoleEmployee::find()
                .filter(t_role_employee::Column::EmployeeId.eq(model.employee_id))
                .one(&self.conn)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "error find sys_user_role");
                    ApiErr::ErrSystem(None)
                })?
                .ok_or(ApiErr::ErrAuth(Some("账号角色关系不存在".to_string())))?;

            let pass = format!("{}", req.password);
            // Crypto::md5(data);
            if Crypto::md5(pass.as_bytes()) != model.login_pwd {
                return Err(ApiErr::ErrAuth(Some("密码错误".to_string())));
            }

            let now = xtime::now(offset!(+8)).unix_timestamp();
            //自定义token
            let login_token = Crypto::md5(format!("auth.{}.{}.{}", model.employee_id, now, utils::nonce(16)).as_bytes());
            // 加密token
            let auth_token = Identity::new(model.employee_id, login_token.clone())
            .to_auth_token()
            .map_err(|e| {
                tracing::error!(error = ?e, "error identity encrypt");
                ApiErr::ErrSystem(None)
            })?;

            // 封装修改model
            let update_model = t_employee::ActiveModel {
                login_at: Set(now),
                login_token: Set(login_token),
                update_time: Set(now),
                ..Default::default()
            };
            // 更新T_employee表数据
            let ret_update = TEmployee::update_many()
                .filter(t_employee::Column::EmployeeId.eq(model.employee_id))
                .set(update_model)
                .exec(&self.conn)
                .await;
            if let Err(e) = ret_update {
                tracing::error!(error = ?e, "error update t_employee");
                return Err(ApiErr::ErrSystem(None));
            }
        
            let resp = RespLogin {
                name: model.realname,
                role: t_role_employee.role_id,
                auth_token,
            };
        
            Ok(ApiOK(Some(resp)))
    }

    /**退出接口 */
    pub async fn logout(&self, identity: Identity) -> Result<ApiOK<()>> {
        let ret: std::result::Result<_, _> = TEmployee::update_many()
            .filter(t_employee::Column::EmployeeId.eq(identity.id()))
            .col_expr(t_employee::Column::LoginToken, Expr::value(""))
            .col_expr(
                t_employee::Column::CreateTime,
                Expr::value(xtime::now(offset!(+8)).unix_timestamp()),
            )
            .exec(&self.conn)
            .await;

        if let Err(e) = ret {
            tracing::error!(error = ?e, "error update t_employee");
            return Err(ApiErr::ErrSystem(None));
        }

        Ok(ApiOK(None))
    }
}