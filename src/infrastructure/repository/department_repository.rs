use std::collections::HashMap;
use crate::infrastructure::persistence::database as db;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set
};
use crate::application::dto::department_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelect};
use crate::domain::entities::{
    t_department, prelude::TDepartment,
    t_employee, prelude::TEmployee
};

use crate::common::{
    result::response::{ApiErr, ApiOK, Result},
    utils,xtime,
    tree,
};

use time::macros::offset;


pub struct DepartmentRepository{
    conn: DatabaseConnection
}

impl DepartmentRepository {

    pub fn new() -> Self {
        Self { conn: db::conn().clone() }
    }


    // 创建部门
    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
        let count: u64 = TDepartment::find()
        .filter(t_department::Column::DepartmentName.eq(req.deptname.clone()))
        .count(&self.conn)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "error count t_department");
            ApiErr::ErrSystem(None)
        })?;
    
    if count > 0 {
        return Err(ApiErr::ErrPerm(Some("部门名称重复".to_string())));
    }

    // 创建数据对象
    let now = xtime::now(offset!(+8)).unix_timestamp();
    let model = t_department::ActiveModel {
        department_name: Set(req.deptname),  
        sort: Set(req.sort),
        manager_id: Set(req.managerid),
        parent_id: Set(req.parentid),
        create_time: Set(now),
        ..Default::default()
    };
    /* 插入数据 */
    if let Err(e) = TDepartment::insert(model).exec(&self.conn).await {
        tracing::error!(error = ?e, "error insert t_department");
            return Err(ApiErr::ErrSystem(None));
        }

    Ok(ApiOK(None))
    }

    // 查询部门列表
    pub async fn select_list(&self) ->  Result<ApiOK<Vec<tree::TreeNode>>> {
        let department_list= TDepartment::find()
                .select_only()
                .column(t_department::Column::DepartmentId)
                .column(t_department::Column::DepartmentName)
                .column(t_department::Column::ParentId)
                .all(&self.conn)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "error find t_department");
                    ApiErr::ErrSystem(None)
                })?;
    
        let mut list = Vec::new();
        for department in &department_list {
            list.push(RespSelect {
                department_id: department.department_id,
                department_name: department.department_name.clone(),
                parent_id: department.parent_id,
            });
        }
    
        let tuple_list = list.iter().map(|item| (item.department_id,item.department_name.clone(),Some(item.parent_id))).collect::<Vec<_>>();
        let tuple_node: Option<Vec<tree::TreeNode>> = tree::build_tree(tuple_list);
        Ok(ApiOK(tuple_node))
    }

    /** 获取列表 */
    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        let mut builder = TDepartment::find();
        if let Some(deptrtment_name) = query.get("deptname") {
            builder = builder.filter(t_department::Column::DepartmentName.contains(deptrtment_name));

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
                .column_as(t_department::Column::DepartmentId.count(), "count")
                .into_tuple::<i64>()
                .one(&self.conn)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "error count t_department");
                    ApiErr::ErrSystem(None)
                })?
                .unwrap_or_default();
        }


        let models = builder
            .order_by(t_department::Column::DepartmentId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_role");
                ApiErr::ErrSystem(None)
            })?;

        let mut resp = RespList {
            total,
            list: (Vec::with_capacity(models.len())),
        };

        for model in models {
            let info = RespInfo {
                department_id: model.department_id,
                department_name: model.department_name,
                manager_id: model.manager_id,
                parent_id: model.parent_id,
                sort: model.sort,
                create_time: model.create_time,
                create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
                .unwrap_or_default(),
            };
            resp.list.push(info);
        }

        Ok(ApiOK(Some(resp)))

    }


    /** 获取详情 */
    pub async fn info(&self, department_id: i64) -> Result<ApiOK<RespInfo>> {

        let model = TDepartment::find_by_id(department_id)
            .one(&self.conn)
            .await 
            .map_err(|e| {
                        tracing::error!(error = ?e, "error find t_department");
                        ApiErr::ErrSystem(None)
            })?
            .ok_or(ApiErr::ErrNotFound(Some("部门信息不存在".to_string())))?;

    let mut resp = RespInfo {
            department_id: model.department_id,
            department_name: model.department_name,
            manager_id: model.manager_id,
            parent_id: model.parent_id,
            sort: model.sort,
            create_time: model.create_time,
            create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
        .unwrap_or_default(),
    };
    Ok(ApiOK(Some(resp)))
    }

    // 修改方法
    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
        let now = xtime::now(offset!(+8)).unix_timestamp();
        let model = t_department::ActiveModel {
            department_id: Set(req.deptid),
            department_name: Set(req.deptname),  
            sort: Set(req.sort),
            manager_id: Set(req.managerid),
            parent_id: Set(req.parentid),
            update_time: Set(now),
            ..Default::default()
        };

        if let Err(e) = TDepartment::update(model)
        .exec(&self.conn)
        .await {
            tracing::error!(error = ?e, "error update t_department");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(ApiOK(None))
    }

    // 删除部门
    pub async fn delete(&self, department_id: i64) -> Result<ApiOK<()>> {
        //判断是否有子部门
        let department_count = TDepartment::find()
            .filter(t_department::Column::ParentId.eq(department_id))
            .count(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_department");
                ApiErr::ErrSystem(None)
            })?;
        
        if department_count > 0 {
            return Err(ApiErr::ErrPerm(Some("该部门下有子部门，无法删除".to_string())));
        }

        //判断删除部门时，该部门下是否有用户
        let  employee_count = TEmployee::find()
            .filter(t_employee::Column::DepartmentId.eq(department_id))
            .count(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_employee");
                ApiErr::ErrSystem(None)
            })?;

        if employee_count > 0 {
            return Err(ApiErr::ErrPerm(Some("该部门下已分配员工，无法删除".to_string())));
        }


        if let Err(e) = TDepartment::delete_by_id(department_id).exec(&self.conn).await {
            tracing::error!(error = ?e, "error delete t_department");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(ApiOK(None))
    }

}


