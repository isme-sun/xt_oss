use serde::{Deserialize, Serialize};
use std::fmt;

pub use reqwest::{
    header::{self, HeaderMap, HeaderName, HeaderValue},
    IntoUrl, Method, StatusCode, Url,
};

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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
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
                Self::ATTACHMENT(Some(filename)) => format!("attachment;filename=\"{}\"", filename),
                Self::ATTACHMENT(None) => "attachment".to_owned(),
            }
        )
    }
}

/// 指定该Object被下载时网页的缓存行为
#[derive(Debug, Default, Clone, Copy)]
pub enum CacheControl {
    /// 不可直接使用缓存,而是先到服务端验证Object是否已更新。如果Object已更新,表明缓存已过期,需从服务端重新下载Object;如果Object未更新,表明缓存未过期，此时将使用本地缓存。
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

#[cfg(test)]
mod tests {

    #[test]
    fn content_disposition_1() {
        let filename: Option<String> = Some("测试.txt".to_string());
        let value = crate::oss::http::ContentDisposition::ATTACHMENT(filename);
        assert!(value.to_string().contains("测试"))
    }
}
