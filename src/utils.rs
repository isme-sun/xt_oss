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

#[derive(Debug, Default, Clone)]
pub struct ByteRange(pub Option<usize>, pub Option<isize>);

impl fmt::Display for ByteRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.0, self.1) {
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
    fn range() {
        assert_eq!(ByteRange(None, None).to_string(), "bytes=0-");
        assert_eq!(ByteRange(None, Some(500)).to_string(), "bytes=0-499");
        assert_eq!(ByteRange(None, Some(-500)).to_string(), "bytes=-500");
        assert_eq!(ByteRange(Some(100), None).to_string(), "bytes=100-");
        assert_eq!(ByteRange(Some(100), Some(500)).to_string(), "bytes=100-599");
        assert_eq!(ByteRange(Some(100), Some(-500)).to_string(), "bytes=0-99");
        assert_eq!(ByteRange(Some(100), Some(-50)).to_string(), "bytes=50-99");
    }
}
