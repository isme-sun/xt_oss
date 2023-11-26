//!
//! # 阿里云OSS SDK
//! # 阿里云OSS OssClient
//!
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{self, Display};

/// *阿里云OSS服务地址*
const OSS_BASE_URL: &'static str = "aliyuncs.com";
/// *默认区域*
const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";

pub(crate) mod api;
pub mod arguments;
pub mod entities;

/// OSS 返回结果
// pub type OssResult = Result<(), Box<dyn std::error::Error>>;
pub type OssResult<T> = Result<OssData<T>, OssError>;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OssError {
    #[serde(rename(deserialize = "Code"))]
    pub code: String,
    #[serde(rename(deserialize = "Message"))]
    pub message: String,
    #[serde(rename(deserialize = "RequestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "HostId"))]
    pub host_id: String,
    #[serde(rename(deserialize = "EC"))]
    pub ec: String,
    #[serde(rename(deserialize = "RecommendDoc"))]
    pub recommend_doc: String,
}

impl Display for OssError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sorry, something is wrong! Please Try Again!")
    }
}

#[derive(Debug)]
pub struct OssData<T> {
    // pub request: Request,
    // pub response: Response,
    pub headers: HeaderMap,
    pub data: T,
}

/// 客户端配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OssOptions {
    /// 通过阿里云控制台创建的AccessKey ID
    pub access_key_id: String,
    /// 通过阿里云控制台创建的AccessKey Secret
    pub access_key_secret: String,
    /// 使用临时授权方式。更多信息，请参见 [使用STS进行临时授权](https://help.aliyun.com/zh/oss/developer-reference/authorized-access-3#section-zkq-3rq-dhb)。
    pub sts_token: String,
    /// 通过控制台或PutBucket创建的Bucket
    pub bucket: String,
    /// OSS访问域名。
    pub endpoint: String,
    /// Bucket所在的区域， 默认值为oss-cn-hangzhou
    pub region: String,
    /// 是否使用阿里云内网访问，默认值为false
    pub internal: bool,
    /// 是否支持上传自定义域名，默认值为false
    pub cname: bool,
    /// Bucket是否开启请求者付费模式，默认值为false
    pub is_request_pay: bool,
    /// 设置secure为true，则使用HTTPS；设置secure为false，则使用HTTP
    pub secure: bool,
    /// 超时时间，默认值为60000
    pub timeout: i32,
}

impl Default for OssOptions {
    fn default() -> Self {
        Self {
            access_key_id: Default::default(),
            access_key_secret: Default::default(),
            sts_token: Default::default(),
            bucket: Default::default(),
            endpoint: OSS_BASE_URL.to_string(),
            region: DEFAULT_REGION.to_string(),
            internal: false,
            cname: false,
            is_request_pay: false,
            secure: true,
            timeout: 60,
        }
    }
}

#[allow(dead_code)]
impl OssOptions {
    pub fn from_env() -> Self {
        let mut options = OssOptions::default();
        options.access_key_id = env::var("OSS_ACCESS_KEY_ID").unwrap_or_default();
        options.access_key_secret = env::var("OSS_ACCESS_KEY_SECRET").unwrap_or_default();
        options.sts_token = env::var("OSS_STS_TOKEN").unwrap_or_default();
        options.bucket = env::var("OSS_BUCKET").unwrap_or_default();
        options.region = env::var("OSS_REGION").unwrap_or_default();
        if let Ok(value) = env::var("OSS_INTERNAL") {
            options.internal = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_CNAME") {
            options.cname = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_IS_REQUEST_PAY") {
            options.is_request_pay = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_SECURE") {
            options.secure = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_TIMEOUT") {
            options.timeout = value.parse::<i32>().unwrap_or(60);
        }
        options
    }

    pub fn common_headers(&self) -> HeaderMap {
        let mut headers = header::HeaderMap::new();
        // host
        // let host = self.get_host().parse().unwrap();
        // headers.insert(header::HOST, host);
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
        // user_agent
        // headers.insert(
        //     header::USER_AGENT,
        //     HeaderValue::from_static("xt oss"),
        // );
        return headers;
    }

    fn get_schema(&self) -> String {
        if self.secure == true {
            "https".to_string()
        } else {
            "http".to_string()
        }
    }

    fn get_host(&self) -> String {
        if self.internal == true {
            format!("{}-internal.{}", self.region, self.endpoint)
        } else {
            format!("{}.{}", self.region, self.endpoint)
        }
    }

    pub fn get_root_url(&self) -> String {
        format!("{}://{}", self.get_schema(), self.get_host()).to_string()
    }

    pub fn get_base_url(&self) -> String {
        format!(
            "{}://{}.{}",
            self.get_schema(),
            self.bucket,
            self.get_host()
        )
        .to_string()
    }
}

mod inner {
    use base64::{engine::general_purpose, Engine as _};
    use chrono::{DateTime, Utc};
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    use crypto::sha1::Sha1;
    use hmacsha1;

    /// 通用base64编码
    pub(super) fn base64_encode(content: &[u8]) -> String {
        let encoded = general_purpose::STANDARD.encode(content);
        encoded
    }

    /// 给出字符串的md5值
    #[allow(unused)]
    pub(super) fn md5(text: &String) -> String {
        let mut hasher = Md5::new();
        hasher.input_str(&text[..]);
        let hex = hasher.result_str();
        hex
    }

    // 计算给出字符串的sha1加密值
    #[allow(unused)]
    pub(super) fn sha1(text: &String) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(&text[..]);
        let hex = hasher.result_str();
        hex.to_string()
    }

    /// hmac sha1 计算
    ///
    /// ~~~no_run
    /// /* 转成16进制字符串 */
    /// hash.iter()
    ///     .map(|b| format!("{:02x}", b))
    ///     .collect::<Vec<String>>()
    ///     .join("")
    /// ~~~
    ///
    pub(super) fn hmac_sha1(key: &String, message: &String) -> [u8; 20] {
        let key = key.as_bytes();
        let message = message.as_bytes();
        let hash = hmacsha1::hmac_sha1(key, message);
        hash
    }

    // 获取GMT时间格式
    pub(super) fn get_gmt_date(dt: &DateTime<Utc>) -> String {
        let fmt = "%a, %d %b %Y %H:%M:%S GMT";
        dt.format(fmt).to_string()
    }

    #[derive(Debug)]
    pub(super) struct Authorization {
        pub(super) verb: reqwest::Method,
        pub(super) date: DateTime<Utc>,
        pub(super) object_key: Option<String>,
        pub(super) bucket: Option<String>,
        // ! 命名
        pub(super) sub_res: Option<String>,
    }

    impl Default for Authorization {
        fn default() -> Self {
            Self {
                // 请求方法
                verb: reqwest::Method::GET,
                // 请求时间
                date: Utc::now(),
                // 请求文件对象
                object_key: None,
                // 当前bucket
                bucket: None,
                // 资源符
                sub_res: None,
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
            let value = hmac_sha1(&key_secret.to_string(), &value.to_string());
            base64_encode(value.as_slice())
        }

        pub(crate) fn to_value(&self, access_key_id: &str, key_secret: &str) -> String {
            format!("OSS {}:{}", access_key_id, self.signature(key_secret))
        }
    }
}

use self::inner::{get_gmt_date, Authorization};
use reqwest::StatusCode;

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct OssClient {
    pub options: OssOptions,
    _client: reqwest::Client,
}
// *----------------------------------------------------------------------------------
/// 初始化，私有方法
impl OssClient {
    #[allow(dead_code)]
    pub fn builder(options: OssOptions) -> Self {
        let client = reqwest::Client::builder().default_headers(options.common_headers());
        OssClient {
            options,
            _client: client.build().unwrap(),
        }
    }

    async fn request(
        &self,
        url: String,
        auth: Authorization,
    ) -> Result<(StatusCode, HeaderMap, String), OssError> {
        let value = auth
            .to_value(&self.options.access_key_id, &self.options.access_key_secret)
            .to_string();
        let request = self
            ._client
            .request(auth.verb, url)
            .header(header::DATE, get_gmt_date(&auth.date))
            .header(header::AUTHORIZATION, value);

        let response = request.send().await.unwrap_or_else(|err| {
            panic!("Error: {}", err.to_string());
        });

        let status = response.status();
        let headers = response.headers().clone();
        let content = response.text().await.unwrap().to_string();

        if !status.is_success() {
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        } else {
            Ok((status, headers, content))
        }
    }
}
