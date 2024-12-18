mod cli;
mod common;
mod infrastructure;
mod interface;
mod application;
mod domain;

use anyhow::{Context, Result};
use tracing::info;

use crate::cli::{Cli, Command, hello, serve};
use clap::Parser;
use crate::common::{config, logger};
use crate::infrastructure::persistence::database;





#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    println!("{:?}", cli.config);

    // 初始化系统
    let _guard = init(&cli.config).await?;

    // 执行命令
    if let Some(v) = cli.command {
        match v {
            Command::Hello { name } => hello::execute(name).await?,
            Command::Serve => serve::execute("127.0.0.1".into(), 8080).await?,
        }
    }
    Ok(())
}

/// 应用初始化
async fn init(config_path: &str) -> Result<tracing_appender::non_blocking::WorkerGuard> {
    // 初始化配置
    config::init(config_path);
    info!("配置初始化完成");

    // 初始化日志
    let guard = logger::init(Some(config::global()));
    info!("日志系统初始化完成");
    
    // 初始化数据库
    database::init(config::global()).await.context("Failed to initialize database")?;
    info!("数据库连接初始化完成");

    guard
}




