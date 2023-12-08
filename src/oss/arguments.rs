use std::fmt;

// use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

/// 指定存储空间的存储类型
///
/// - `Standard` 标准存储
/// - `IA` 低频访问
/// - `Archive` 归档存储
/// - `DeepColdArchive` 深度冷归档存储
///
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "StorageClass")]
pub enum StorageClass {
    #[default]
    Standard,
    IA,
    Archive,
    ColdArchive,
    DeepColdArchive,
}

/// 指定存储空间的数据容灾类型
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "DataRedundancyType")]
pub enum DataRedundancyType {
    /// 本地冗余LRS将您的数据冗余存储在同一个可用区的不同存储设备上，可支持两个存储设备并发损坏时，仍维持数据不丢失，可正常访问
    #[default]
    LRS,
    /// 同城冗余ZRS采用多可用区（AZ）内的数据冗余存储机制，将用户的数据冗余存储在同一地域（Region）的多个可用区。当某个可用区不可用时，仍然能够保障数据的正常访问
    ZRS,
}

#[derive(Debug, Default)]
pub enum OssAcl {
    // #[serde(rename = "public-read-write")]
    PublicReadWrite,
    #[default]
    // #[serde(rename = "public-read")]
    PublicRead,
    // #[serde(rename = "private")]
    Private,
}

impl fmt::Display for OssAcl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
            Self::Private => "private",
        };
        write!(f, "{}", desc)
    }
}

