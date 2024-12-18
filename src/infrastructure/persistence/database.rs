use std::{sync::OnceLock, time::Duration};

use config::Config;
use serde::Deserialize;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use anyhow::{Result, Context};

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

//连接配置抽取
#[derive(Debug, Deserialize)]
struct DbOptions {
    min_conns: Option<u32>,
    max_conns: Option<u32>,
    conn_timeout: Option<u64>,
    idle_timeout: Option<u64>,
    max_lifetime: Option<u64>,
}

fn configure_connection_options(opt: &mut ConnectOptions, cfg: &Config) -> Result<()> {
    let options: DbOptions = cfg
        .get("db.options")
        .context("Failed to get database options")?;

    opt.min_connections(options.min_conns.unwrap_or(10))
        .max_connections(options.max_conns.unwrap_or(20))
        .connect_timeout(Duration::from_secs(options.conn_timeout.unwrap_or(10)))
        .idle_timeout(Duration::from_secs(options.idle_timeout.unwrap_or(300)))
        .max_lifetime(Duration::from_secs(options.max_lifetime.unwrap_or(600)))
        .sqlx_logging(cfg.get_bool("app.debug").unwrap_or_default());

    Ok(())
}



pub async fn init(cfg: &Config) -> Result<()> {
    let mut opt = ConnectOptions::new(
        cfg.get_string("db.dsn")
            .context("Missing database DSN configuration")?
    );

    // 配置连接参数
    configure_connection_options(&mut opt, cfg)?;

    // 建立连接
    let conn = Database::connect(opt)
        .await
        .context("Failed to connect to database")?;

    // 测试连接
    conn.ping()
        .await
        .context("Failed to ping database")?;

    // 存储连接
    DB.set(conn)
        .map_err(|_| anyhow::anyhow!("Database connection already initialized"))?;

    Ok(())
}


pub fn conn() -> &'static DatabaseConnection {
    DB.get().unwrap_or_else(|| panic!("数据库连接未初始化"))
}
