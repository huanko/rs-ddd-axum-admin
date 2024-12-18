use anyhow::{Result, Context};
use digest::{crypto_common::BlockSizeUser, Digest, Mac};
use hmac::SimpleHmac;
use md5::Md5;
use sha1::Sha1;
use sha2::Sha256;

/// 加密工具错误
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Encoding error: {0}")]
    EncodingError(String),
}

/// 哈希函数特征
pub trait Hasher {
    fn hash(&self, data: &[u8]) -> String;
    fn hmac(&self, key: &[u8], data: &[u8]) -> Result<String>;
}

/// 加密工具结构体
#[derive(Debug, Clone, Copy)]
pub struct Crypto;

impl Crypto {
    /// 计算 MD5 哈希
    pub fn md5(data: &[u8]) -> String {
        hash::<Md5>(data)
    }

    /// 计算 SHA1 哈希
    pub fn sha1(data: &[u8]) -> String {
        hash::<Sha1>(data)
    }

    /// 计算 SHA256 哈希
    pub fn sha256(data: &[u8]) -> String {
        hash::<Sha256>(data)
    }

    /// 计算 HMAC-SHA1
    pub fn hmac_sha1(key: &[u8], data: &[u8]) -> Result<String> {
        hmac::<Sha1>(key, data).context("HMAC-SHA1 failed")
    }

    /// 计算 HMAC-SHA256
    pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<String> {
        hmac::<Sha256>(key, data).context("HMAC-SHA256 failed")
    }
}

/// 通用哈希函数
fn hash<D: Digest>(data: &[u8]) -> String {
    let mut hasher = D::new();
    hasher.update(data);
    const_hex::encode(hasher.finalize())
}

/// 通用 HMAC 函数
fn hmac<D: Digest + BlockSizeUser>(key: &[u8], data: &[u8]) -> Result<String> {
    let mut mac = SimpleHmac::<D>::new_from_slice(key)
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    
    mac.update(data);
    Ok(const_hex::encode(mac.finalize().into_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let data = b"hello world";
        let hash = Crypto::md5(data);
        assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }

    #[test]
    fn test_hmac_sha256() {
        let key = b"secret";
        let data = b"hello world";
        let hmac = Crypto::hmac_sha256(key, data).unwrap();
        assert!(!hmac.is_empty());
    }

    #[test]
    fn test_invalid_key() {
        let key = [];  // 空key
        let data = b"test";
        let result = Crypto::hmac_sha256(&key, data);
        assert!(result.is_err());
    }
}