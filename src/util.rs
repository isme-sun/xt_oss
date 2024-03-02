use std::{
    fmt,
    fs::File,
    io::{self, Read},
};

use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Local, Utc};
use crypto::{digest::Digest, md5::Md5};
use oss::http;

use crate::oss;

fn get_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or(default.to_string())
}

fn get_env_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .unwrap_or(default.to_string())
        .parse()
        .unwrap_or(default)
}

/// UTC to GMT format
#[inline]
pub fn utc_to_gmt(datetime: DateTime<Utc>) -> String {
    datetime.format(super::oss::GMT_DATE_FMT).to_string()
}

/// LOCAL to GMT format
#[inline]
pub fn local_to_gmt(local_datetime: DateTime<Local>) -> String {
    let utc_datetime: DateTime<Utc> = local_datetime.with_timezone(&Utc);
    utc_datetime.format(super::oss::GMT_DATE_FMT).to_string()
}


pub enum AllowedOriginItem<'a> {
    Any,
    Urls(Vec<&'a str>),
}

impl<'a> fmt::Display for AllowedOriginItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AllowedOriginItem::Any => "*".to_string(),
                AllowedOriginItem::Urls(urls) => urls.join(","),
            }
        )
    }
}

pub enum AllowedMethodItem {
    Any,
    Methods(Vec<http::Method>),
}

impl fmt::Display for AllowedMethodItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AllowedMethodItem::Any => "*".to_string(),
                AllowedMethodItem::Methods(methods) => {
                    methods
                        .into_iter()
                        .map(|entry| entry.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                }
            }
        )
    }
}

pub enum AllowedHeaderItem {
    Any,
    Headers(Vec<http::header::HeaderName>),
}

impl fmt::Display for AllowedHeaderItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AllowedHeaderItem::Any => "*".to_string(),
                AllowedHeaderItem::Headers(headers) => {
                    headers
                        .into_iter()
                        .map(|entry| entry.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                }
            }
        )
    }
}

pub fn options_from_env() -> oss::Options<'static> {
    oss::Options::new()
        .with_access_key_id(get_env("OSS_ACCESS_KEY_ID", "").leak())
        .with_access_key_secret(get_env("OSS_ACCESS_KEY_SECRET", "").leak())
        .with_region(get_env("OSS_REGION", oss::DEFAULT_REGION).leak())
        .with_endpoint(get_env("OSS_ENDPOINT", "").leak())
        .with_bucket(get_env("OSS_BUCKET", "").leak())
        .with_sts_token(get_env("OSS_STS_TOKEN", "").leak())
        .with_internal(get_env_bool("OSS_INTERNAL", false))
        .with_cname(get_env_bool("OSS_CNAME", false))
        // .with_is_request_pay(get_env_bool("OSS_IS_REQUEST_PAY", false))
        .with_secret(get_env_bool("OSS_SECURE", false))
        .with_timeout(
            get_env("OSS_TIMEOUT", "60")
                .parse::<u64>()
                .unwrap_or(oss::DEFAULT_TIMEOUT),
        )
}

/// 获取文件md5值
///
/// [doc](https://help.aliyun.com/zh/oss/developer-reference/include-signatures-in-the-authorization-header#section-i74-k35-5w4)
pub fn oss_file_md5<'a>(file: &'a str) -> Result<String, io::Error> {
    let mut file = File::open(file)?;
    let mut hasher = Md5::new();
    let mut buffer = [0; 1024];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.input(&buffer[..bytes_read]);
    }
    let bytes = hex::decode(&hasher.result_str()).unwrap();
    Ok(general_purpose::STANDARD.encode(&bytes))
}

pub fn oss_md5<'a>(content: &'a [u8]) -> Result<String, io::Error> {
    let mut hasher = Md5::new();
    hasher.input(content);
    let bytes = hex::decode(&hasher.result_str()).unwrap();
    Ok(general_purpose::STANDARD.encode(&bytes))
}

/// 获取字节范围描述
///
/// - `start`  开始位置
/// - `amount` 获取字节数量，支持负数
///
/// # example
///
/// ```rust no_run
/// use xt_oss::util::ByteRange;
/// assert_eq!(ByteRange::new().to_string(), "bytes=0-");
/// assert_eq!(ByteRange::new().with_amount(500).to_string(), "bytes=0-499");
/// assert_eq!(ByteRange::new().with_amount(-500).to_string(), "bytes=-500");
/// assert_eq!(ByteRange::new().with_start(100).to_string(), "bytes=100-");
/// assert_eq!(ByteRange::from((100, 500)).to_string(), "bytes=100-599");
/// assert_eq!(ByteRange::from((100, -500)).to_string(), "bytes=0-99");
/// assert_eq!(ByteRange::from((100, -50)).to_string(), "bytes=50-99");
/// ```
///
#[derive(Debug, Default, Clone, Copy)]
pub struct ByteRange {
    start: Option<u64>,
    amount: Option<i64>,
}

impl ByteRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chunk(total: u64, chunk_size: u64) -> Vec<Self> {
        let mut reuslt: Vec<ByteRange> = vec![];
        let mut max_count = 0;
        for i in 0..total / chunk_size {
            reuslt.push((i * chunk_size, chunk_size as i64).into());
            max_count = i;
        }

        let rest = total - ((max_count + 1) * chunk_size);
        if rest != 0 {
            let start = total - rest;
            reuslt.push((start, rest as i64).into());
        }
        reuslt
    }

    pub fn with_start(mut self, value: u64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn with_amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn start(&self) -> u64 {
        self.start.unwrap_or_default()
    }

    pub fn amount(&self) -> i64 {
        self.amount.unwrap_or_default()
    }
}

impl From<(u64, i64)> for ByteRange {
    fn from(item: (u64, i64)) -> Self {
        Self {
            start: Some(item.0),
            amount: Some(item.1),
        }
    }
}

impl fmt::Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.start, self.amount) {
            (None, None) => write!(f, "bytes=0-"),
            (None, Some(amount)) => {
                if amount >= 0 {
                    write!(f, "bytes=0-{}", amount - 1)
                } else {
                    write!(f, "bytes=-{}", amount.abs())
                }
            }
            (Some(start), None) => write!(f, "bytes={}-", start),
            (Some(start), Some(amount)) if amount > 0 => {
                write!(f, "bytes={}-{}", start, start + amount as u64 - 1)
            }
            (Some(start), Some(amount)) => {
                let start_pos = if start as i64 + amount > 0 {
                    start as i64 + amount
                } else {
                    0
                };
                write!(f, "bytes={}-{}", start_pos.max(0), start - 1)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::ByteRange;

    #[test]
    fn range_1() {
        assert_eq!(ByteRange::new().to_string(), "bytes=0-");
        assert_eq!(ByteRange::new().with_amount(500).to_string(), "bytes=0-499");
        assert_eq!(ByteRange::new().with_amount(-500).to_string(), "bytes=-500");
        assert_eq!(ByteRange::new().with_start(100).to_string(), "bytes=100-");
        // 通过元组生成
        assert_eq!(ByteRange::from((100, 500)).to_string(), "bytes=100-599");
        assert_eq!(ByteRange::from((100, -500)).to_string(), "bytes=0-99");
        assert_eq!(ByteRange::from((100, -50)).to_string(), "bytes=50-99");
    }

    #[test]
    fn range_2() {
        let range_list = ByteRange::chunk(87475, 1024);
        assert_eq!("bytes=87040-87474", range_list.last().unwrap().to_string())
    }
}
