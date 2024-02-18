use crate::oss;
use serde::{Deserialize, Serialize};
use std::fmt;

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
pub mod website;
pub mod worm;

pub enum Directive {
  COPY,
  REPLACE,
}

impl fmt::Display for Directive {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::COPY => "COPY",
        Self::REPLACE => "REPLACE",
      }
    )
  }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum ContentEncoding {
  /// 表示Object未经过压缩或编码
  #[default]
  IDENTITY,
  /// 表示Object采用Lempel-Ziv（LZ77）压缩算法以及32位CRC校验的编码方式。
  GZIP,
  /// 表示Object采用Lempel-Ziv-Welch（LZW）压缩算法的编码方式。
  COMPRESS,
  /// 表示Object采用zlib结构和deflate压缩算法的编码方式。
  DEFLATE,
  /// 表示Object采用Brotli算法的编码方式。
  BR,
}

impl fmt::Display for ContentEncoding {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::IDENTITY => "IDENTITY",
        Self::GZIP => "GZIP",
        Self::COMPRESS => "COMPRESS",
        Self::DEFLATE => "DEFLATE",
        Self::BR => "br",
      }
    )
  }
}

#[derive(Default, Debug, Clone)]
pub enum ContentDisposition {
  #[default]
  INLINE,
  ATTACHMENT(Option<String>),
}

impl fmt::Display for ContentDisposition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::INLINE => "inline".to_owned(),
        Self::ATTACHMENT(Some(filename)) => format!("attachment;filename={}", filename),
        Self::ATTACHMENT(None) => "attachment".to_owned(),
      }
    )
  }
}

/// 指定该Object被下载时网页的缓存行为
#[derive(Debug, Default, Clone, Copy)]
pub enum CacheControl {
  /// 不可直接使用缓存，而是先到服务端验证Object是否已更新。如果Object已更新，表明缓存已过期，需从服务端重新下载Object；如果Object未更新，表明缓存未过期，此时将使用本地缓存。
  NoCache,
  ///所有内容都不会被缓存。
  NoStore,
  /// 所有内容都将被缓存。
  #[default]
  PUBLIC,
  /// 所有内容只在客户端缓存。
  PRIVATE,
  /// 缓存内容的相对过期时间，单位为秒。此选项仅在HTTP 1.1中可用。
  MaxAge(u32),
}

impl fmt::Display for CacheControl {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CacheControl::NoCache => "no-cache".into(),
        CacheControl::NoStore => "no-store".into(),
        CacheControl::PUBLIC => "public".into(),
        CacheControl::PRIVATE => "private".into(),
        CacheControl::MaxAge(seconds) => format!("max-age=<{}>", seconds),
      }
    )
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum Status {
  Enabled,
  #[default]
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
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(tag = "DataRedundancyType")]
pub enum DataRedundancyType {
  /// 本地冗余LRS将您的数据冗余存储在同一个可用区的不同存储设备上，可支持两个存储设备并发损坏时，仍维持数据不丢失，可正常访问
  #[default]
  LRS,
  /// 同城冗余ZRS采用多可用区（AZ）内的数据冗余存储机制，将用户的数据冗余存储在同一地域（Region）的多个可用区。当某个可用区不可用时，仍然能够保障数据的正常访问
  ZRS,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum OssAcl {
  PublicReadWrite,
  #[default]
  PublicRead,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ObjectACL {
  #[default]
  Default,
  PublicReadWrite,
  PublicRead,
  Private,
}

impl fmt::Display for ObjectACL {
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
  /// 冷归档存储
  ColdArchive,
  /// 深度冷归档存储
  DeepColdArchive,
}

impl fmt::Display for StorageClass {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Standard => "Standard",
        Self::IA => "IA",
        Self::Archive => "Archive",
        Self::ColdArchive => "ColdArchive",
        Self::DeepColdArchive => "DeepColdArchive",
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

impl fmt::Display for ServerSideEncryption {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::AES256 => "AES256",
        Self::KMS => "KMS",
        Self::SM4 => "SM$",
      }
    )
  }
}
