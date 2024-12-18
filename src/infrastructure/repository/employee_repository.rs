use std::collections::HashMap;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Order,        
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    Condition,
};
use sea_orm::prelude::Expr;
use crate::domain::entities::{
    t_department, prelude::TDepartment,
    t_employee, prelude::TEmployee
};
use crate::application::dto::employee_dto::{ReqCreate, UpdateInfo, RespInfo, 
    RespList, RespSelectOption, RespEmpInfo, RespDeptInfo};
use crate::common::result::response::{ApiErr, ApiOK, Result};
use time::macros::offset;
use crate::infrastructure::persistence::database as db;
use crate::common::crypto::hash::Crypto;
use crate::common::{
    xtime, utils
};





pub struct EmployeeRepository {
    conn: DatabaseConnection
}

impl EmployeeRepository {
    pub fn new() -> Self {
        Self {
            conn: db::conn().clone()
        }
    }


    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
        // 验证登录名是否已存在
        let login_name_count = TEmployee::find()
            .filter(t_employee::Column::LoginName.eq(req.login_name.clone()))
            .count(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count login_name");
                ApiErr::ErrSystem(None)
            })?;
    
        if login_name_count > 0 {
            return Err(ApiErr::ErrPerm(Some("登录名已重复".to_string())));
        }
    
        let phone_count = TEmployee::find()
            .filter(t_employee::Column::Phone.eq(req.phone.clone()))
            .count(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count phone");
                ApiErr::ErrSystem(None)   
            })?;
    
        if phone_count > 0 {
            return Err(ApiErr::ErrPerm(Some("手机号码已重复".to_string())));
        }
    
        let now = xtime::now(offset!(+8)).unix_timestamp();
        let model = t_employee::ActiveModel {
            realname: Set(req.realname),
            phone: Set(req.phone),
            department_id: Set(req.department_id),
            login_name: Set(req.login_name.clone()),
            login_pwd: Set(Crypto::md5(req.login_name.clone().as_bytes()).to_string()),
            email: Set(req.email),
            gender: Set(req.gender),
            disabled_flag: Set(req.disabled_flag),
            position_id: Set(req.position_id),
            create_time: Set(now),
            ..Default::default()
        };
    
        if let Err(e) = TEmployee::insert(model)
            .exec(&self.conn)
            .await{
                tracing::error!(error = ?e, "error insert t_employee");
                return Err(ApiErr::ErrSystem(None));
            }
    
        Ok(ApiOK(None))     
    }



    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        let mut builder = TEmployee::find();
        if let Some(disabled_flag) = query.get("disabled_flag") {
            if disabled_flag == "1" {
                builder = builder.filter(t_employee::Column::DisabledFlag.eq(1));
            } else {
                builder = builder.filter(t_employee::Column::DisabledFlag.eq(0));
            }
        }
    
        if let Some(login_name) = query.get("login_name") {
            if !login_name.is_empty() {
                builder = builder.filter(t_employee::Column::LoginName.contains(login_name));
            }
        }
    
        if let Some(phone) = query.get("phone") {
            if !phone.is_empty() {
                builder = builder.filter(t_employee::Column::Phone.contains(phone));  
            }
        }
    
    
    
        let mut total: i64 = 0;
        let pagination = utils::Pagination::from_query(&query).unwrap();
        let offset = pagination.offset;
        let limit = pagination.limit;
        // 仅在第一页计算数量
        if offset == 0 {
            total = builder
                .clone()
                .select_only()
                .column_as(t_employee::Column::EmployeeId.count(), "count")
                .into_tuple::<i64>()
                .one(&self.conn)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "error count t_employee");
                    ApiErr::ErrSystem(None)
                })?
                .unwrap_or_default();
        }
    
        let models = builder
            .order_by(t_employee::Column::EmployeeId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_employee");
                ApiErr::ErrSystem(None)
            })?;
        let mut resp = RespList {
            total,
            list: (Vec::with_capacity(models.len())),
        };
        for model in models {
            let info = RespInfo {
                employee_id: model.employee_id,
                login_name: model.login_name,
                realname: model.realname,
                phone: model.phone,
                email: model.email,
                gender: model.gender,
                disabled_flag: model.disabled_flag,
                position_id: model.position_id,
                department_id: model.department_id,
                create_time: model.create_time,
                create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
                .unwrap_or_default(),
            };
            resp.list.push(info);
        }
    
        Ok(ApiOK(Some(resp)))
    
    }
    
    
    pub async fn info(&self, employee_id: i64) -> Result<ApiOK<RespInfo>> {
        let model = TEmployee::find_by_id(employee_id)
            .one(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_employee");
                ApiErr::ErrSystem(None)
            })?
            .ok_or(ApiErr::ErrNotFound(Some("员工信息不存在".to_string())))?;
    
    
        let mut resp = RespInfo {
            employee_id: model.employee_id,
            login_name: model.login_name,
            realname: model.realname,
            gender: model.gender,
            phone: model.phone,
            email: model.email,
            disabled_flag: model.disabled_flag,
            position_id: model.position_id,
            department_id: model.department_id,
            create_time: model.create_time,
            create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
            .unwrap_or_default(),
        };
        Ok(ApiOK(Some(resp)))
    }



    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
        /* 判断登录名或者手机号是否重复*/
        let count = TEmployee::find()
        .filter(Condition::any().add(t_employee::Column::LoginName.eq(req.login_name.clone())).add(t_employee::Column::Phone.eq(req.phone.clone())))
        .count(&self.conn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count t_employee");
            ApiErr::ErrSystem(None)
        })?;
  
        if count > 0 {
          return Err(ApiErr::ErrPerm(Some("登录名称或手机号码重复".to_string())));
      }
  
      let now = xtime::now(offset!(+8)).unix_timestamp();
      let model = t_employee::ActiveModel {
          employee_id: Set(req.employee_id),
          login_name: Set(req.login_name),
          realname: Set(req.realname),
          phone: Set(req.phone),
          email: Set(req.email),
          gender: Set(req.gender),
          disabled_flag: Set(req.disabled_flag),
          position_id: Set(req.position_id),
          department_id: Set(req.department_id),
          update_time: Set(now),
          ..Default::default()
      };
  
      if let Err(e) = TEmployee::update(model)
              .exec(&self.conn)
              .await{
                  tracing::error!(error = ?e, "error update t_employee");
                  return Err(ApiErr::ErrSystem(None));
              }
              Ok(ApiOK(None))
  }
  
    // 禁用
    pub async fn disabled_flag(&self, employee_id: i64, disabled_flag:u8) -> Result<ApiOK<()>> {
        let _update_model = TEmployee::update_many()
            .col_expr(t_employee::Column::DeletedFlag, Expr::value(disabled_flag))
            .filter(t_employee::Column::EmployeeId.eq(employee_id))
            .exec(&self.conn)
            .await;   
        
        Ok(ApiOK(None))
    }
    
    // 重置密码
    pub async fn reset_password(&self, employee_id: i64) -> Result<ApiOK<()>> {
            let _update_model = TEmployee::update_many()
                .col_expr(t_employee::Column::LoginPwd, Expr::value(Crypto::md5("123456".as_bytes()).to_string()))
                .filter(t_employee::Column::EmployeeId.eq(employee_id))
                .exec(&self.conn)
                .await;     
    
        Ok(ApiOK(None))
    }
    
    // 调整部门
    pub async fn change_department(&self, employee_id: Vec<i64>, department_id:i64) -> Result<ApiOK<()>> {
            let _update_model = TEmployee::update_many()
                    .col_expr(t_employee::Column::DepartmentId, Expr::value(department_id))
                    .filter(t_employee::Column::EmployeeId.is_in(employee_id))
                    .exec(&self.conn)
                    .await;    
            Ok(ApiOK(None))
    }


    //人员下拉框
    pub async fn employee_select_list(&self) -> Result<ApiOK<Vec<RespSelectOption>>> {
        
        //查询所有未删除的员工，并取出员工id，姓名，部门id
        let employee_models = TEmployee::find()
            .select_only()
            .column(t_employee::Column::EmployeeId)
            .column(t_employee::Column::Realname)
            .column(t_employee::Column::DepartmentId)
            .filter(t_employee::Column::DeletedFlag.eq(0))
            .all(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_role");
                ApiErr::ErrSystem(None)
            })?;

        // 将查询出来的数据封装到临时结构体RespEmpInfo中
        let mut emp_list = Vec::with_capacity(employee_models.len());
        for emp in employee_models {
            emp_list.push(RespEmpInfo {
                employee_id: emp.employee_id,
                realname: emp.realname,
                department_id: emp.department_id,
            });
        }

        

        // 查询部门表，并取出部门id，名称
        let department_models = TDepartment::find()
            .select_only()
            .column(t_department::Column::DepartmentId)
            .column(t_department::Column::DepartmentName)
            .all(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_department");
                ApiErr::ErrSystem(None)
            })?;
        
        // 将查询出来的数据封装到临时结构体RespDeptInfo中
        let mut dept_list = Vec::with_capacity(department_models.len());
        for dept in department_models {
            dept_list.push(RespDeptInfo {
                department_id: dept.department_id,
                department_name: dept.department_name,
            });
        }

        // 将员工表和部门表的数据合并
        let list: Vec<RespSelectOption> = self.merge_employee_department(emp_list, dept_list);

        Ok(ApiOK(Some(list)))
    }

    // 根据员工表数据和部门数据，合并生成下拉框数据
    fn merge_employee_department(&self, employee_list: Vec<RespEmpInfo>, department_list:Vec<RespDeptInfo>) -> Vec<RespSelectOption> {

        // 定义存储的数据集合,并初始化
        let mut result = Vec::with_capacity(employee_list.len());

        // 将部门数据存储到hashmap中
        let department_map: HashMap<i64,String> = department_list
            .into_iter()
            .map(|dept| (dept.department_id, dept.department_name))
            .collect();

        // 遍历员工数据，如果部门id在hashmap中存在，则将数据添加到result
        for employee in employee_list {
            if let Some(department_name) = department_map.get(&employee.department_id){
                let info = RespSelectOption {
                    employee_id: employee.employee_id,
                    realname: employee.realname,
                    department_id: employee.department_id,
                    department_name: department_name.clone(),
                };
                result.push(info);
            }   
        }
        result

    }   

}   
