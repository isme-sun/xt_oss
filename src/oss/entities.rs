use crate::oss;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

pub mod acceleration;
pub mod acl;
pub mod bucket;
pub mod cname;
pub mod cors;
pub mod encryption;
pub mod lifecycle;
pub mod log;
pub mod object;
pub mod private;
pub mod referer;
pub mod region;
pub mod style;
pub mod tag;
pub mod version;
pub mod worm;

pub enum Status {
    Enabled,
    Disabled,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::Enabled => "Enabled",
            Self::Disabled => "Disabled",
        };
        write!(f, "{}", desc)
    }
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
    PublicReadWrite,
    #[default]
    PublicRead,
    Private,
}

impl Display for OssAcl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
            Self::Private => "private",
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ObjectACL {
    #[default]
    Default,
    PublicReadWrite,
    PublicRead,
    Private,
}

impl Display for ObjectACL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Default => "default",
            Self::PublicReadWrite => "public-read-write",
            Self::PublicRead => "public-read",
            Self::Private => "private",
        };
        write!(f, "{}", value)
    }
}

/// OSS 存储类型
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum StorageClass {
    /// 标准存储
    #[default]
    Standard,
    /// 低频访问存储
    IA,
    /// 归档存储
    Archive,
}

impl Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Archive => "Archive",
                Self::IA => "IA",
                Self::Standard => "STANDARD",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ServerSideEncryption {
    //使用OSS完全托管密钥进行加解密（SSE-OSS）。
    #[default]
    AES256,
    // 使用KMS托管密钥进行加解密。
    KMS,
    // 国密SM4算法。
    SM4,
}

#[derive(Default)]
pub enum ContentDisposition<'a> {
    #[default]
    INLINE,
    ATTACHMENT(Option<&'a str>),
}

impl<'a> Display for ContentDisposition<'a> {
    // TODO 协议完善
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::INLINE => "inline".to_string(),
            Self::ATTACHMENT(Some(filename)) => format!("attachment;filename={}", filename),
            Self::ATTACHMENT(None) => "attachment".to_string(),
        };
        write!(f, "{}", content)
    }
}

#[derive(Default)]
pub enum ContentEncoding {
    #[default]
    IDENTITY,
    GZIP,
    COMPRESS,
    DEFLATE,
    BR,
}

impl Display for ContentEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IDENTITY => "identity",
                Self::GZIP => "gzip",
                Self::COMPRESS => "compress",
                Self::DEFLATE => "deflate",
                Self::BR => "br",
            }
        )
    }
}
