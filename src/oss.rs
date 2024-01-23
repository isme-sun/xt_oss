// re-export
// #[allow(unused)]
pub use bytes::Bytes;
// #[allow(unused)]
pub mod http {
    pub use reqwest::{
        header::{self, HeaderMap, HeaderName, HeaderValue},
        Method, StatusCode,
    };
}

pub use reqwest::{IntoUrl, Url};

pub(crate) mod api;
pub mod entities;

//-------------------------------------------------------------------------
use super::oss;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmacsha1;
use reqwest::{
    self,
    header::{AUTHORIZATION, CONTENT_TYPE, DATE},
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::time::Duration;

pub(crate) const BASE_URL: &str = "aliyuncs.com";
pub(crate) const DEFAULT_REGION: &str = "oss-cn-hangzhou";
pub(crate) const USER_AGENT: &str = "xt oss/0.1";
pub(crate) const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
pub(crate) const DEFAULT_CONNECT_TIMEOUT: u64 = 180;
pub(crate) const GMT_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S GMT";
// const XML_DOCTYPE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

#[derive(Debug, Default)]
pub struct Data<T = Bytes> {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub body: T,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename(deserialize = "Code"))]
    pub code: String,
    #[serde(rename(deserialize = "Message"))]
    pub message: String,
    #[serde(rename(deserialize = "RequestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "HostId"))]
    pub host_id: String,
    #[serde(rename(deserialize = "EC"))]
    pub ec: Option<String>,
    #[serde(rename(deserialize = "RecommendDoc"))]
    pub recommend_doc: Option<String>,
    #[serde(rename = "OSSAccessKeyId")]
    pub oss_access_key_id: Option<String>,
    #[serde(rename = "SignatureProvided")]
    pub signature_provided: Option<String>,
    #[serde(rename = "StringToSign")]
    pub string_to_sign: Option<String>,
    #[serde(rename = "StringToSignBytes")]
    pub string_to_sign_bytes: Option<String>,
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

#[allow(unused)]
type Result<T = Bytes> = std::result::Result<Data<T>, Message>;

#[derive(Debug)]
#[allow(unused)]
struct Authorization<'a> {
    access_key_id: &'a str,
    access_key_secret: &'a str,
    sts_token: &'a str,
    bucket: Option<String>,
    object: Option<String>,
    headers: &'a http::HeaderMap,
    method: &'a http::Method,
    date: &'a String,
    resourse: Option<&'a str>,
}

#[allow(unused)]
impl<'a> Authorization<'a> {
    fn complute(&self) -> String {
        format!("OSS {}:{}", self.access_key_id, self.signature())
    }

    fn signature(&self) -> String {
        let header_str = {
            let mut oss_key_name: Vec<&str> = Vec::new();
            let keys = self.headers.keys();
            for item in keys {
                let name = item.as_str();
                if name.starts_with("x-oss") {
                    oss_key_name.push(name);
                }
            }

            oss_key_name.sort();
            let mut value: Vec<String> = Vec::new();
            for key_name in oss_key_name {
                let key_value = self.headers.get(key_name).unwrap().to_str().unwrap();
                value.push(format!("{}:{}\n", key_name, key_value));
            }
            value.join("")
        };

        let value = format!(
            "{VERB}\n\n{ContentType}\n{Date}\n{Header}{Resource}",
            VERB = self.method,
            Header = header_str,
            ContentType = oss::DEFAULT_CONTENT_TYPE,
            Date = self.date,
            // Resource = self.canonicalized_resource()
            Resource = self.resourse.unwrap_or_default()
        );
        dbg!(println!("{}", value));
        let key = self.access_key_secret.as_bytes();
        let message = value.as_bytes();
        let value = hmacsha1::hmac_sha1(key, message);
        let encoded = general_purpose::STANDARD.encode(value.as_slice());
        encoded
    }

    fn canonicalized_resource(&self) -> String {
        let res_path = match (&self.bucket, &self.object) {
            (Some(bucket), Some(object)) => {
                format!("/{}/{}", bucket, object)
            }
            (Some(bucket), None) => {
                format!("/{}/", bucket)
            }
            (None, None) => "/".to_string(),
            (None, Some(_)) => {
                panic!("params error")
            }
        };
        if let Some(res) = self.resourse {
            format!("{}?{}", res_path, res)
        } else {
            res_path
        }
    }
}

#[allow(unused)]
pub struct RequestTask<'a> {
    request: &'a oss::Request<'a>,
    url: &'a str,
    resource: Option<&'a str>,
    timeout: Option<u64>,
    method: http::Method,
    headers: http::HeaderMap,
    body: Bytes,
}

#[allow(unused)]
impl<'a> RequestTask<'a> {
    pub(crate) fn new(request: &'a oss::Request<'a>) -> Self {
        Self {
            request,
            url: Default::default(),
            resource: None,
            timeout: None,
            method: http::Method::GET,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    fn resource(&self) -> String {
        self.resource.unwrap_or("/").to_string()
    }

    pub fn with_url(mut self, value: &'a str) -> Self {
        self.url = value;
        self
    }

    pub fn with_resource(mut self, value: &'a str) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn with_headers(mut self, value: http::HeaderMap) -> Self {
        self.headers = value;
        self
    }

    pub fn with_method(mut self, value: http::Method) -> Self {
        self.method = value;
        self
    }

    pub fn with_body(mut self, value: Bytes) -> Self {
        self.body = value;
        self
    }

    pub fn with_timeout(mut self, value: u64) -> Self {
        self.timeout = Some(value);
        self
    }

    pub async fn execute(&self) -> oss::Result<Bytes> {
        // let (_, bucket, object) = Self::parse_url(self.url);

        // dbg!(println!("{:#?}", (&bucket, &object)));
        let bucket = None;
        let object = None;

        let date = Utc::now().format(oss::GMT_DATE_FMT).to_string();
        let mut headers = http::HeaderMap::new();
        headers.insert(DATE, date.parse().unwrap());
        headers.extend(self.headers.to_owned());

        println!("{:#?}", headers);

        headers.insert(
            AUTHORIZATION,
            Authorization {
                access_key_id: self.request.access_key_id.unwrap_or_default(),
                access_key_secret: self.request.access_key_secret.unwrap_or_default(),
                sts_token: self.request.access_key_secret.unwrap_or_default(),
                headers: &headers,
                method: &self.method,
                bucket,
                object,
                date: &date,
                resourse: Some(&self.resource()),
            }
            .complute()
            .parse()
            .unwrap(),
        );

        let request_builder = self
            .request
            .client
            .request(self.method.to_owned(), self.url)
            .headers(headers)
            .body(self.body.to_owned());

        let resp = request_builder.send().await.unwrap();

        let status = resp.status();
        let headers = resp.headers().to_owned();
        let body = resp.bytes().await.unwrap();

        if status.is_success() {
            let oss_data = Data {
                status,
                headers,
                body,
            };
            Ok(oss_data)
        } else {
            let content = String::from_utf8_lossy(&body);
            if content.len() > 0 {
                let message: Message = quick_xml::de::from_str(&content).unwrap();
                Err(message)
            } else if headers.contains_key("x-oss-err") {
                let error_info = headers.get("x-oss-err").unwrap();
                let error_info = general_purpose::STANDARD.decode(error_info).unwrap();
                let content = String::from_utf8_lossy(&error_info);
                let message: Message = quick_xml::de::from_str(&content).unwrap();
                Err(message)
            } else {
                let message = Message::default();
                Err(message)
            }
        }
    }

    // fn parse_url<T>(input: T) -> (Option<String>, Option<String>, Option<String>)
    // where
    //     T: IntoUrl,
    // {
    //     let url: Url = input.into_url().unwrap();
    //     dbg!(&url.to_string());
    //     let host = url.host().unwrap().to_string();
    //     dbg!(&host);
    //     if host == oss::BASE_URL {
    //         (None, None, None)
    //     } else {
    //         let fragment = &host[..(host.len() - oss::BASE_URL.len() - 1)];
    //         dbg!(fragment);
    //         let result = fragment.split_once('.');
    //         let (bucket, region) = match result {
    //             Some((bucket, region)) => (Some(bucket.to_string()), Some(region.to_string())),
    //             _ => (None, Some(fragment.to_string())),
    //         };
    //         let object = url.path().trim_start_matches('/');
    //         let object = if object.is_empty() {
    //             None
    //         } else {
    //             Some(object.to_string())
    //         };
    //         (region, bucket, object)
    //     }
    // }
}

#[allow(unused)]
#[derive(Debug, Default)]
pub struct Request<'a> {
    access_key_id: Option<&'a str>,
    access_key_secret: Option<&'a str>,
    region: Option<&'a str>,
    bucket: Option<&'a str>,
    internal: Option<bool>,
    sts_token: Option<&'a str>,
    endpoint: Option<&'a str>,
    timeout: u64,
    client: reqwest::Client,
}

#[allow(unused)]
impl<'a> Request<'a> {
    pub fn new() -> Self {
        let mut headers = http::HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            http::HeaderValue::from_static(DEFAULT_CONTENT_TYPE),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(oss::USER_AGENT)
            .connect_timeout(Duration::from_secs(DEFAULT_CONNECT_TIMEOUT))
            .build()
            .unwrap();
        Self {
            client,
            ..Self::default()
        }
    }

    pub fn with_access_key_id(mut self, value: &'a str) -> Self {
        self.access_key_id = Some(value);
        self
    }

    pub fn with_access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = Some(value);
        self
    }

    pub fn with_region(mut self, value: &'a str) -> Self {
        self.region = Some(value);
        self
    }

    pub fn with_bucket(mut self, value: &'a str) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn with_internal(mut self, value: bool) -> Self {
        self.internal = Some(value);
        self
    }

    pub fn with_sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = Some(value);
        self
    }

    pub fn with_endpoint(mut self, value: &'a str) -> Self {
        self.endpoint = Some(value);
        self
    }

    pub fn with_timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    #[allow(private_interfaces)]
    pub fn task(&self) -> RequestTask<'_> {
        RequestTask::new(self)
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Options<'a> {
    /// 通过阿里云控制台创建的AccessKey ID
    access_key_id: &'a str,
    /// 通过阿里云控制台创建的AccessKey Secret
    access_key_secret: &'a str,
    /// 使用临时授权方式
    sts_token: &'a str,
    /// 通过控制台或PutBucket创建的Bucket
    bucket: &'a str,
    /// OSS访问域名。
    // endpoint: &'a str,
    /// Bucket所在的区域， 默认值为oss-cn-hangzhou
    region: &'a str,
    /// 是否使用阿里云内网访问，默认值为false
    internal: bool,
    /// 是否支持上传自定义域名，默认值为false
    cname: bool,
    /// Bucket是否开启请求者付费模式，默认值为false
    is_request_pay: bool,
    /// 设置secure为true，则使用HTTPS；设置secure为false，则使用HTTP
    secure: bool,
    /// 超时时间，默认值为60000
    timeout: u64,
}

impl<'a> Default for Options<'a> {
    fn default() -> Self {
        Self {
            access_key_id: Default::default(),
            access_key_secret: Default::default(),
            sts_token: Default::default(),
            bucket: Default::default(),
            // endpoint: Default::default(),
            region: oss::DEFAULT_REGION,
            internal: false,
            cname: false,
            is_request_pay: false,
            secure: true,
            timeout: 60u64,
        }
    }
}

#[allow(unused)]
impl<'a> Options<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn access_key_id(mut self, value: &'a str) -> Self {
        self.access_key_id = value;
        self
    }

    pub fn access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = value;
        self
    }

    pub fn bucket(mut self, value: &'a str) -> Self {
        self.bucket = value;
        self
    }

    pub fn region(mut self, value: &'a str) -> Self {
        self.region = value;
        self
    }

    pub fn sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = value;
        self
    }

    // pub fn endpoint(mut self, value: &'a str) -> Self {
    //     self.endpoint = value;
    //     self
    // }
    pub fn internal(mut self, value: bool) -> Self {
        self.internal = value;
        self
    }

    pub fn cname(mut self, value: bool) -> Self {
        self.cname = value;
        self
    }
    pub fn is_request_pay(mut self, value: bool) -> Self {
        self.is_request_pay = value;
        self
    }

    pub fn secret(mut self, value: bool) -> Self {
        self.is_request_pay = value;
        self
    }
    pub fn timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    pub fn root_url(&self) -> String {
        format!("{}://{}", self.schema(), self.host()).to_string()
    }

    pub fn base_url(&self) -> String {
        format!("{}://{}.{}", self.schema(), self.bucket, self.host()).to_string()
    }

    fn schema(&self) -> String {
        match self.secure {
            true => "https".to_string(),
            false => "http".to_string(),
        }
    }

    fn host(&self) -> String {
        match self.internal {
            true => {
                format!("{}-internal.{}", self.region, oss::BASE_URL)
            }
            false => {
                format!("{}.{}", self.region, oss::BASE_URL)
            }
        }
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Client<'a> {
    options: Options<'a>,
    request: Request<'a>,
}

impl<'a> Client<'a> {
    pub fn new(options: Options<'a>) -> Self {
        let request = self::Request::new()
            .with_access_key_id(options.access_key_id)
            .with_access_key_secret(options.access_key_secret)
            .with_timeout(options.timeout);
        Self { options, request }
    }

    pub fn options(&self) -> &oss::Options {
        &self.options
    }

    pub fn region(mut self, value: &'a str) -> Self {
        self.options.region = value;
        self
    }

    pub fn bucket(mut self, value: &'a str) -> Self {
        self.options.bucket = value;
        self
    }

    pub fn internal(mut self, value: bool) -> Self {
        self.options.internal = value;
        self
    }

    pub fn cname(mut self, value: bool) -> Self {
        self.options.cname = value;
        self
    }

    pub fn secure(mut self, value: bool) -> Self {
        self.options.secure = value;
        self
    }

    pub fn timeout(mut self, value: u64) -> Self {
        self.options.timeout = value;
        self
    }
}
