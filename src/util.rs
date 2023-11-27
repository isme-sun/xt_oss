use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use hmacsha1;
// use reqwest::header::HeaderMap;

/// 通用base64编码
pub(crate) fn base64_encode(content: &[u8]) -> String {
    let encoded = general_purpose::STANDARD.encode(content);
    encoded
}

/// 给出字符串的md5值
#[allow(unused)]
pub(crate) fn md5(text: &String) -> String {
    let mut hasher = Md5::new();
    hasher.input_str(&text[..]);
    let hex = hasher.result_str();
    hex
}

// 计算给出字符串的sha1加密值
#[allow(unused)]
pub(crate) fn sha1(text: &String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&text[..]);
    let hex = hasher.result_str();
    hex.to_string()
}

pub(crate) fn hmac_sha1(key: &String, message: &String) -> [u8; 20] {
    let key = key.as_bytes();
    let message = message.as_bytes();
    let hash = hmacsha1::hmac_sha1(key, message);
    hash
}

// 获取GMT时间格式
pub(crate) fn get_gmt_date(dt: &DateTime<Utc>) -> String {
    let fmt = "%a, %d %b %Y %H:%M:%S GMT";
    dt.format(fmt).to_string()
}

#[derive(Debug)]
pub(crate) struct Authorization {
    pub(super) verb: reqwest::Method,
    pub(super) date: DateTime<Utc>,
    pub(super) object_key: Option<String>,
    pub(super) bucket: Option<String>,
    pub(super) sub_res: Option<String>,
    // pub(super) headers: Option<HeaderMap>
}

impl Default for Authorization {
    fn default() -> Self {
        Self {
            verb: reqwest::Method::GET,
            date: Utc::now(),
            object_key: None,
            bucket: None,
            sub_res: None,
            // headers: None
        }
    }
}

impl Authorization {
    pub(super) fn canonicalized_resource(&self) -> String {
        let res_path = match (&self.bucket, &self.object_key) {
            (Some(bucket), Some(object_key)) => {
                format!("/{}/{}", bucket, object_key)
            }
            (Some(bucket), None) => {
                format!("/{}/", bucket)
            }
            (None, None) => "/".to_string(),
            (None, Some(_)) => {
                panic!("params error")
            }
        };
        if let Some(res) = &self.sub_res {
            format!("{}?{}", res_path, res)
        } else {
            format!("{}", res_path)
        }
    }

    pub(super) fn signature(&self, key_secret: &str) -> String {
        let value = format!(
            "{VERB}\n\napplication/octet-stream\n{Date}\n{cr}",
            VERB = &self.verb.to_string(),
            Date = get_gmt_date(&self.date),
            cr = &self.canonicalized_resource()
        );
        // println!("{}",value);
        let value = hmac_sha1(&key_secret.to_string(), &value.to_string());
        base64_encode(value.as_slice())
    }

    pub(crate) fn to_value(&self, access_key_id: &str, key_secret: &str) -> String {
        format!("OSS {}:{}", access_key_id, self.signature(key_secret))
    }
}
