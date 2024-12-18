use anyhow::Result;
use config::Config;
use std::path::PathBuf;
use time::OffsetDateTime;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::{self, fmt::time::FormatTime};

// 日志配置结构体
#[derive(Debug)]
struct LogConfig {
    level: Level,
    path: PathBuf,
    filename: String,
    is_dev: bool,
    use_json: bool,
}

impl LogConfig {
    fn from_config(cfg: &Config) -> Result<Self> {
        Ok(Self {
            level: if cfg.get_bool("app.debug").unwrap_or_default() {
                Level::DEBUG
            } else {
                Level::INFO
            },
            path: PathBuf::from(cfg.get_string("log.path").unwrap_or_else(|_| "logs".to_string())),
            filename: cfg.get_string("log.filename").unwrap_or_else(|_| "tracing.log".to_string()),
            is_dev: cfg.get_string("app.env").unwrap_or_else(|_| "dev".to_string()) == "dev",
            use_json: cfg.get_bool("log.json").unwrap_or(true),
        })
    }

    fn default() -> Self {
        Self {
            level: Level::DEBUG,
            path: PathBuf::from("logs"),
            filename: "tracing.log".to_string(),
            is_dev: true,
            use_json: true,
        }
    }
}

// 格式化日志的输出时间格式
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            OffsetDateTime::now_utc()
                .format(&time::format_description::parse(
                    "[year]-[month]-[day] [hour]:[minute]:[second]"
                )
                .unwrap())
                .unwrap_or_else(|_| String::from(""))
        )
    }
}

pub fn init(cfg: Option<&Config>) -> Result<WorkerGuard> {
    let config = match cfg {
        Some(cfg) => LogConfig::from_config(cfg)?,
        None => LogConfig::default(),
    };

    let (non_blocking, guard) = if config.is_dev {
        // 开发环境，日志输出到控制台
        tracing_appender::non_blocking(std::io::stdout())
    } else {
        // 生产环境，输出到文件
        tracing_appender::non_blocking(tracing_appender::rolling::daily(
            config.path,
            config.filename,
        ))
    };

    // 构建日志订阅器
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(config.level)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false)
        .with_timer(LocalTimer)
        .with_writer(non_blocking);

    // 根据配置决定是否使用JSON格式
    if config.use_json {
        subscriber.json().flatten_event(true).init();
    } else {
        subscriber.init();
    }

    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, File};
    use std::path::PathBuf;

    #[test]
    fn test_log_config() -> Result<()> {
        let mut cfg = Config::default();
        cfg.merge(File::from_str(
            r#"
            app:
              debug: true
              env: "dev"
            log:
              path: "test_logs"
              filename: "test.log"
              json: true
            "#,
            config::FileFormat::Yaml,
        ))?;

        let log_config = LogConfig::from_config(&cfg)?;
        assert_eq!(log_config.level, Level::DEBUG);
        assert_eq!(log_config.path, PathBuf::from("test_logs"));
        assert_eq!(log_config.filename, "test.log");
        assert!(log_config.is_dev);
        assert!(log_config.use_json);

        Ok(())
    }
} 