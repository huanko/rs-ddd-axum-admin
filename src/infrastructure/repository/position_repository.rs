use std::collections::HashMap;
use crate::infrastructure::persistence::database as db;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, Order, 
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use crate::domain::entities::{
    t_position, prelude::TPosition
};
use crate::application::dto::position_dto::{ReqCreate, UpdateInfo, RespInfo, RespList, RespSelect};
use crate::common::{
    result::response::{ApiErr, ApiOK, Result},
    utils,xtime,
};
use time::macros::offset;



pub struct PositionRepository {
    conn: DatabaseConnection
}

impl PositionRepository {
    pub fn new() -> Self {
        Self {
            conn: db::conn().clone()
        }
    }


    /** 添加方法 */
    pub async fn create(&self, req: ReqCreate) -> Result<ApiOK<()>> {
            
            let count = TPosition::find()
                .filter(t_position::Column::PositionName.eq(req.postname.clone()))
                .count(&self.conn)
                .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error count t_position");
                ApiErr::ErrSystem(None)
            })?;
        
        if count > 0 {
            return Err(ApiErr::ErrPerm(Some("职务名称重复".to_string())));
        }

        /** 创建数据对象 */
        let now = xtime::now(offset!(+8)).unix_timestamp();
        let model = t_position::ActiveModel {
            position_name: Set(req.postname),
            level: Set(req.level),
            sort: Set(req.sort),
            remark: Set(req.remark),
            deleted_flag: Set(0),
            create_time: Set(now),
            ..Default::default()
        };
        /* 插入数据 */
        if let Err(e) = TPosition::insert(model).exec(&self.conn).await {
            tracing::error!(error = ?e, "error insert t_position");
            return Err(ApiErr::ErrSystem(None));
        }

        Ok(ApiOK(None))
    }



    /** 获取列表 */
    pub async fn list(&self, query: HashMap<String, String>) -> Result<ApiOK<RespList>> {
        /** 查询条件 */
        let mut builder = TPosition::find();
        if let Some(postname) = query.get("postname") {
            if !postname.is_empty() {
                builder = builder.filter(t_position::Column::PositionName.contains(postname));
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
                .column_as(t_position::Column::PositionId.count(), "count")
                .into_tuple::<i64>()
                .one(&self.conn)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "error count t_position");
                    ApiErr::ErrSystem(None)
                })?
                .unwrap_or_default();
        }

        let models = builder
            .order_by(t_position::Column::PositionId, Order::Desc)
            .offset(offset)
            .limit(limit)
            .all(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_position");
                ApiErr::ErrSystem(None)
            })?;
        let mut resp = RespList {
            total,
            list: (Vec::with_capacity(models.len())),
        };
        for model in models {
            let info = RespInfo {
                postid: model.position_id,
                postname: model.position_name,
                level: model.level,
                sort: model.sort,
                remark: model.remark,
                create_time: model.create_time,
                create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
                .unwrap_or_default(),
            };
            resp.list.push(info);
        }

        Ok(ApiOK(Some(resp)))
    }

    /** 获取详情 */
    pub async fn info(&self, postid: u64) -> Result<ApiOK<RespInfo>> {
        let model = TPosition::find_by_id(postid as i64)
            .one(&self.conn)
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "error find t_position");
                ApiErr::ErrSystem(None)
            })?
            .ok_or(ApiErr::ErrNotFound(Some("职务信息不存在".to_string())))?;

        let mut resp = RespInfo {
            postid: model.position_id,
            postname: model.position_name,
            level: model.level,
            sort: model.sort,
            remark: model.remark,
            create_time: model.create_time,
            create_time_str: xtime::to_string(xtime::DATETIME, model.create_time, offset!(+8))
        .unwrap_or_default(),
        };
        Ok(ApiOK(Some(resp)))
    }

    /** 修改方法 */
    pub async fn update(&self, req: UpdateInfo) -> Result<ApiOK<()>> {
    
        let now = xtime::now(offset!(+8)).unix_timestamp();
        let model = t_position::ActiveModel {
            position_name: Set(req.postname),
            level: Set(req.level),
            sort: Set(req.sort),
            remark: Set(req.remark),
            update_time: Set(now),
            ..Default::default()
        };

        if let Err(e) = TPosition::update(model).exec(&self.conn).await {
            tracing::error!(error = ?e, "error update t_position");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(ApiOK(None))
    }

    /** 删除 */
    pub async fn delete(&self, postid: u64) -> Result<ApiOK<()>> {

        if let Err(e) = TPosition::delete_by_id(postid as i64).exec(&self.conn).await {
            tracing::error!(error = ?e, "error delete t_position");
            return Err(ApiErr::ErrSystem(None));
        }
        Ok(ApiOK(None))
    }


    // 获取下拉列表
    pub async fn select_list(&self) -> Result<ApiOK<Vec<RespSelect>>> {

        let models = TPosition::find()
                .all(&self.conn)
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "error find t_position");
                    ApiErr::ErrSystem(None)
                })?;
    
        let mut list: Vec<RespSelect> = Vec::with_capacity(models.len());
        for model in models {
            list.push(RespSelect {
                postid: model.position_id,
                postname: model.position_name,
            });
        }        
    
        Ok(ApiOK(Some(list)))
    }
}
