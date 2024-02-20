use std::{
    fmt,
    fs::File,
    io::{self, Read},
};

use base64::{engine::general_purpose, Engine as _};
use crypto::{digest::Digest, md5::Md5};

use crate::oss;

pub fn get_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or(default.to_string())
}

pub fn get_env_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .unwrap_or(default.to_string())
        .parse()
        .unwrap_or(default)
}

pub fn options_from_env() -> oss::Options<'static> {
    oss::Options::new()
        .with_access_key_id(get_env("OSS_ACCESS_KEY_ID", "").leak())
        .with_access_key_secret(get_env("OSS_ACCESS_KEY_SECRET", "").leak())
        .with_region(get_env("OSS_REGION", "").leak())
        .with_bucket(get_env("OSS_BUCKET", "").leak())
        .with_sts_token(get_env("OSS_STS_TOKEN", "").leak())
        .with_internal(get_env_bool("OSS_INTERNAL", false))
        .with_cname(get_env_bool("OSS_CNAME", false))
        .with_is_request_pay(get_env_bool("OSS_IS_REQUEST_PAY", false))
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

/// 获取字节范围描述
///
/// - `start`  开始位置
/// - `amount` 获取字节数量，支持负数
///
/// # example
///
/// ```no_run
/// assert_eq!(ByteRange::new().to_string(), "bytes=0-");
/// assert_eq!(ByteRange::new().with_amount(500).to_string(), "bytes=0-499");
/// assert_eq!(ByteRange::new().with_amount(-500).to_string(), "bytes=-500");
/// assert_eq!(ByteRange::new().with_start(100).to_string(), "bytes=100-");
/// assert_eq!(ByteRange::from((100, 500)).to_string(), "bytes=100-599");
/// assert_eq!(ByteRange::from((100, -500)).to_string(), "bytes=0-99");
/// assert_eq!(ByteRange::from((100, -50)).to_string(), "bytes=50-99");
/// ```
///
#[derive(Debug, Default, Clone)]
pub struct ByteRange {
    start: Option<usize>,
    amount: Option<isize>,
}

impl ByteRange {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn chunk(total: usize, chunk_size: usize) -> Vec<Self> {
        let mut reuslt: Vec<ByteRange> = vec![];
        let mut max_count = 0;
        for i in 0..total / chunk_size as usize {
            reuslt.push((i * chunk_size, chunk_size as isize).into());
            max_count = i;
        }

        let rest = total - ((max_count + 1) * chunk_size as usize);
        if rest != 0 {
            let start = total - rest;
            reuslt.push((start, rest as isize).into());
        }
        reuslt
    }

    pub fn with_start(mut self, value: usize) -> Self {
        self.start = Some(value);
        self
    }

    pub fn with_amount(mut self, value: isize) -> Self {
        self.amount = Some(value);
        self
    }
}

impl From<(usize, isize)> for ByteRange {
    fn from(item: (usize, isize)) -> Self {
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
                write!(f, "bytes={}-{}", start, start + amount as usize - 1)
            }
            (Some(start), Some(amount)) => {
                let start_pos = if start as isize + amount > 0 {
                    start as isize + amount
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
