use anyhow::Result;
use rand::distributions::{Alphanumeric, DistString};
use std::{borrow::Cow, collections::HashMap};
use thiserror::Error;
use validator::ValidationError;

// 分页参数的配置常量
const DEFAULT_PAGE_SIZE: u64 = 20;
const MAX_PAGE_SIZE: u64 = 100;

// 自定义错误类型
#[derive(Error, Debug)]
pub enum PaginationError {
    #[error("Invalid page number: {0}")]
    InvalidPage(String),
    #[error("Invalid page size: {0}")]
    InvalidSize(String),
}

// 分页参数结构体
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub offset: u64,
    pub limit: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: DEFAULT_PAGE_SIZE,
        }
    }
}

impl Pagination {
    pub fn new(page: Option<u64>, size: Option<u64>) -> Result<Self> {
        let limit = size.unwrap_or(DEFAULT_PAGE_SIZE).min(MAX_PAGE_SIZE);
        if limit == 0 {
            return Err(PaginationError::InvalidSize("Page size cannot be zero".into()).into());
        }

        let offset = match page {
            Some(p) if p > 0 => (p - 1) * limit,
            Some(_) => return Err(PaginationError::InvalidPage("Page number must be positive".into()).into()),
            None => 0,
        };

        Ok(Self { offset, limit })
    }

    pub fn from_query(args: &HashMap<String, String>) -> Result<Self> {
        let size = args
            .get("size")
            .and_then(|v| v.parse::<u64>().ok());
        
        let page = args
            .get("page")
            .and_then(|v| v.parse::<u64>().ok());

        Self::new(page, size)
    }
}

/// 生成指定长度的随机字符串
/// 
/// # Arguments
/// * `size` - 要生成的随机字符串长度
/// 
/// # Examples
/// ```
/// use crate::common::utils::nonce;
/// let random_string = nonce(16);
/// assert_eq!(random_string.len(), 16);
/// ```
pub fn nonce(size: usize) -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, size)
}

/// 创建新的验证错误
/// 
/// # Arguments
/// * `message` - 错误信息
pub fn new_validation_err(message: impl Into<String>) -> ValidationError {
    ValidationError {
        code: Cow::from("validation_error"),
        message: Some(Cow::from(message.into())),
        params: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination() -> Result<()> {
        // 测试默认值
        let default_pagination = Pagination::default();
        assert_eq!(default_pagination.limit, DEFAULT_PAGE_SIZE);
        assert_eq!(default_pagination.offset, 0);

        // 测试有效输入
        let pagination = Pagination::new(Some(2), Some(30))?;
        assert_eq!(pagination.limit, 30);
        assert_eq!(pagination.offset, 30);

        // 测试超出最大限制
        let pagination = Pagination::new(Some(1), Some(200))?;
        assert_eq!(pagination.limit, MAX_PAGE_SIZE);

        // 测试从查询参数创建
        let mut args = HashMap::new();
        args.insert("page".to_string(), "2".to_string());
        args.insert("size".to_string(), "30".to_string());
        let pagination = Pagination::from_query(&args)?;
        assert_eq!(pagination.limit, 30);
        assert_eq!(pagination.offset, 30);

        Ok(())
    }

    #[test]
    fn test_nonce() {
        let n1 = nonce(16);
        let n2 = nonce(16);
        assert_eq!(n1.len(), 16);
        assert_eq!(n2.len(), 16);
        assert_ne!(n1, n2);
    }

    #[test]
    fn test_validation_err() {
        let err = new_validation_err("test error");
        assert_eq!(err.code, "validation_error");
        assert_eq!(err.message.unwrap(), "test error");
        assert!(err.params.is_empty());
    }
} 