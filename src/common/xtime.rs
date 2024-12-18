use anyhow::{Result, bail};
use time::{OffsetDateTime, format_description::FormatItem};

pub const DATE: &str = "[year]-[month]-[day]";
pub const TIME: &str = "[hour]:[minute]:[second]";
pub const DATETIME: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

// 定义扩展 trait
pub trait DateTimeFormat {
    fn format_datetime(&self) -> Result<String>;
    fn format_date(&self) -> Result<String>;
    fn format_time(&self) -> Result<String>;
}

// 为 OffsetDateTime 实现扩展 trait
impl DateTimeFormat for OffsetDateTime {
    fn format_datetime(&self) -> Result<String> {
        let format = parse_format(DATETIME)?;
        Ok(self.format(&format)?)
    }

    fn format_date(&self) -> Result<String> {
        let format = parse_format(DATE)?;
        Ok(self.format(&format)?)
    }

    fn format_time(&self) -> Result<String> {
        let format = parse_format(TIME)?;
        Ok(self.format(&format)?)
    }
}

// 获取当前时间
pub fn now(offset: time::UtcOffset) -> OffsetDateTime {
    OffsetDateTime::now_utc().to_offset(offset)
}

// 解析时间格式字符串的辅助函数
fn parse_format(fmt: &str) -> Result<Vec<FormatItem<'_>>> {
    Ok(time::format_description::parse(fmt)?)
}

// 根据时间字符串生成时间对象
pub fn from_str(
    fmt: &str,
    datetime: &str,
    offset: time::UtcOffset,
) -> Result<OffsetDateTime> {
    let format = parse_format(fmt)?;
    let v = time::PrimitiveDateTime::parse(datetime, &format)?
        .assume_offset(offset);
    Ok(v)
}

// 根据Unix时间戳生成时间对象
pub fn from_timestamp(
    timestamp: i64,
    offset: time::UtcOffset,
) -> Result<OffsetDateTime> {
    if timestamp < 0 {
        bail!("Invalid negative timestamp: {}", timestamp);
    }
    Ok(OffsetDateTime::from_unix_timestamp(timestamp)?.to_offset(offset))
}

// Unix时间戳格式化
pub fn to_string(fmt: &str, timestamp: i64, offset: time::UtcOffset) -> Result<String> {
    if timestamp < 0 {
        bail!("Invalid negative timestamp: {}", timestamp);
    }
    let format = parse_format(fmt)?;
    Ok(OffsetDateTime::from_unix_timestamp(timestamp)?
        .to_offset(offset)
        .format(&format)?)
}

// 日期转Unix时间戳
pub fn to_timestamp(fmt: &str, datetime: &str, offset: time::UtcOffset) -> Result<i64> {
    if datetime.is_empty() {
        return Ok(0);
    }
    let format = parse_format(fmt)?;
    Ok(time::PrimitiveDateTime::parse(datetime, &format)?
        .assume_offset(offset)
        .unix_timestamp())
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::offset;

    #[test]
    fn test_timestamp_conversion() -> Result<()> {
        let offset = offset!(+8);
        let datetime = "2024-03-20 12:00:00";
        let ts = to_timestamp(DATETIME, datetime, offset)?;
        let str = to_string(DATETIME, ts, offset)?;
        assert_eq!(datetime, str);
        Ok(())
    }

    #[test]
    fn test_negative_timestamp() {
        let offset = offset!(+8);
        assert!(from_timestamp(-1, offset).is_err());
    }

    #[test]
    fn test_format_methods() -> Result<()> {
        let offset = offset!(+8);
        let now = now(offset);

        // 测试日期时间格式化
        let datetime = now.format_datetime()?;
        assert!(!datetime.is_empty());

        // 测试日期格式化
        let date = now.format_date()?;
        assert!(!date.is_empty());

        // 测试时间格式化
        let time = now.format_time()?;
        assert!(!time.is_empty());

        Ok(())
    }

    #[test]
    fn test_empty_datetime() -> Result<()> {
        let offset = offset!(+8);
        let ts = to_timestamp(DATETIME, "", offset)?;
        assert_eq!(ts, 0);
        Ok(())
    }
} 