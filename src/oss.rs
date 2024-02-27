pub const BASE_URL: &str = "aliyuncs.com";
pub const DEFAULT_REGION: &str = "oss-cn-hangzhou";
pub const USER_AGENT: &str = "xt oss/0.1";
pub const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
pub const DEFAULT_CONNECT_TIMEOUT: u64 = 180;
pub const DEFAULT_TIMEOUT: u64 = 60;
pub const GMT_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S GMT";
pub const XML_CONTENT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

pub use bytes::Bytes;
use std::time::Duration;
pub mod api;
pub(super) mod auth;
pub mod entities;
pub mod http;

use super::oss::{
    self,
    http::header::{AUTHORIZATION, CONTENT_TYPE, DATE},
};
use chrono::Utc;
use reqwest::{header::HeaderMap, Response, Result};

pub struct RequestTask<'a> {
    request: &'a oss::Request<'a>,
    url: &'a str,
    resource: Option<&'a str>,
    method: http::Method,
    headers: http::HeaderMap,
    body: Bytes,
}

impl<'a> RequestTask<'a> {
    pub(crate) fn new(request: &'a oss::Request<'a>) -> Self {
        Self {
            request,
            url: Default::default(),
            resource: None,
            method: http::Method::GET,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
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

    pub async fn execute(&self) -> oss::Result<Response> {
        self.inner_execute(None).await
    }

    pub async fn execute_timeout(&self, value: u64) -> oss::Result<Response> {
        self.inner_execute(Some(value)).await
    }

    fn authorization(&self, headers: &HeaderMap, date: &String) -> String {
        let access_key_id = self.request.access_key_id.unwrap_or_default();
        let access_key_secret = self.request.access_key_secret.unwrap_or_default();
        let sts_token = self.request.sts_token;
        let resourse = self.resource;
        auth::SingerV1 {
            access_key_id,
            access_key_secret,
            sts_token,
            headers: &headers,
            method: &self.method,
            date: &date,
            resourse,
        }
        .complute()
    }

    async fn inner_execute(&self, timeout: Option<u64>) -> oss::Result<Response> {
        let date = Utc::now().format(oss::GMT_DATE_FMT).to_string();
        let mut headers = http::HeaderMap::new();
        headers.insert(DATE, date.parse().unwrap());
        if let Some(sts_token) = self.request.sts_token {
            headers.insert("x-oss-security-token", sts_token.parse().unwrap());
        }
        headers.extend(self.headers.to_owned());
        let auth = self.authorization(&headers, &date);
        headers.insert(AUTHORIZATION, auth.parse().unwrap());
        // dbg!(&headers);
        let timeout = Duration::from_secs(timeout.unwrap_or(oss::DEFAULT_TIMEOUT));
        self.request
            .client
            .request(self.method.to_owned(), self.url)
            .headers(headers)
            .timeout(timeout)
            .body(self.body.to_owned())
            .send()
            .await
    }
}

#[derive(Debug, Default, Clone)]
pub struct Request<'a> {
    access_key_id: Option<&'a str>,
    access_key_secret: Option<&'a str>,
    sts_token: Option<&'a str>,
    client: reqwest::Client,
}

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

    pub fn with_sts_token(mut self, value: Option<&'a str>) -> Self {
        self.sts_token = value;
        self
    }

    pub fn task(&self) -> RequestTask<'_> {
        RequestTask::new(&self)
    }
}

#[derive(Debug, Clone, Default, Copy)]
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
    endpoint: &'a str,
    /// Bucket所在的区域,默认值为oss-cn-hangzhou
    region: &'a str,
    /// 是否使用阿里云内网访问,默认值为false
    internal: bool,
    /// 是否支持上传自定义域名,默认值为false
    cname: bool,
    // /// Bucket是否开启请求者付费模,默认值为false
    // is_request_pay: bool,
    /// 设置secure为true,则使用HTTPS;设置secure为false,则使用HTTP
    secure: bool,
    /// 超时时间,默认值为60秒
    timeout: u64,
}

impl<'a> Options<'a> {
    pub fn new() -> Self {
        Self {
            region: oss::DEFAULT_REGION,
            internal: false,
            cname: false,
            // is_request_pay: false,
            secure: false,
            timeout: 60u64,
            ..Self::default()
        }
    }

    pub fn with_access_key_id(mut self, value: &'a str) -> Self {
        self.access_key_id = value;
        self
    }

    pub fn with_access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = value;
        self
    }

    pub fn with_bucket(mut self, value: &'a str) -> Self {
        self.bucket = value;
        self
    }

    pub fn with_region(mut self, value: &'a str) -> Self {
        self.region = value;
        self
    }

    pub fn with_sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = value;
        self
    }

    pub fn with_endpoint(mut self, value: &'a str) -> Self {
        self.endpoint = if let Some(v) = value.strip_prefix("http://") {
            v
        } else if let Some(v) = value.strip_prefix("https://") {
            v
        } else {
            value
        };
        self
    }

    pub fn with_internal(mut self, value: bool) -> Self {
        self.internal = value;
        self
    }

    pub fn with_cname(mut self, value: bool) -> Self {
        self.cname = value;
        self
    }

    // pub fn with_is_request_pay(mut self, value: bool) -> Self {
    //     self.is_request_pay = value;
    //     self
    // }

    pub fn with_secret(mut self, value: bool) -> Self {
        self.secure = value;
        self
    }
    pub fn with_timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    pub fn root_url(&self) -> String {
        format!(
            "{}://{}{}.{}",
            self.schema(),
            oss::DEFAULT_REGION,
            if self.internal == true {
                "-internal"
            } else {
                ""
            },
            oss::BASE_URL
        )
    }

    pub fn base_url(&self) -> String {
        if self.internal == true {
            format!("{}://{}.{}", self.schema(), self.bucket, self.host())
        } else if self.cname == true {
            format!("{}://{}", self.schema(), self.host())
        } else {
            if self.bucket.is_empty() {
                panic!("Bucket parameter must be provided.");
            }
            format!("{}://{}.{}", self.schema(), self.bucket, self.host())
        }
    }

    pub fn object_url(&self, object: &'a str) -> String {
        format!("{}/{}", self.base_url(), object)
    }

    fn schema(&self) -> String {
        match self.secure {
            true => "https".to_string(),
            false => "http".to_string(),
        }
    }

    // 当`cname`为true时,`endpoint`,`bucket`为必填,否则产生panic错误.
    // 当internal为true时，忽略cname与endpoint
    // 无论是否使用cname正确的设置region(location)与bucket
    fn host(&self) -> String {
        if self.internal == true {
            format!(
                "{}{}.{}",
                self.region,
                if self.internal { "-internal" } else { "" },
                oss::BASE_URL
            )
        } else if self.cname == true {
            if self.endpoint.is_empty() {
                panic!("Endpoint parameter must be provided.");
            }
            self.endpoint.to_string()
        } else {
            format!("{}.{}", self.region, oss::BASE_URL)
        }
    }

    pub fn client(self) -> oss::Client<'a> {
        oss::Client::new(self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Client<'a> {
    options: Options<'a>,
    request: Request<'a>,
}

impl<'a> Client<'a> {
    pub fn new(options: Options<'a>) -> Self {
        let request = self::Request::new()
            .with_access_key_id(options.access_key_id)
            .with_access_key_secret(options.access_key_secret)
            .with_sts_token((!options.sts_token.is_empty()).then_some(options.sts_token));
        Self { options, request }
    }

    pub fn options(&self) -> &Options {
        &self.options
    }

    pub fn region(&self) -> &'a str {
        self.options.region
    }

    pub fn bucket(&self) -> &'a str {
        self.options.bucket
    }

    pub fn root_url(&self) -> String {
        self.options.root_url()
    }

    pub fn base_url(&self) -> String {
        self.options.base_url()
    }

    pub fn object_url(&self, object: &'a str) -> String {
        self.options.object_url(object)
    }

    pub fn timeout(&self) -> u64 {
        self.options.timeout
    }
}

#[cfg(test)]
pub mod tests {
    use crate::oss;

    #[test]
    fn options_new_normal_1() {
        let options = oss::Options::new()
            .with_access_key_id("access_key_id")
            .with_access_key_secret("access_key_secret")
            .with_region("oss-cn-shanghai")
            .with_endpoint("cdn.xuetube.com")
            .with_bucket("xuetube")
            .with_cname(true)
            .with_internal(true)
            .with_secret(true);
        assert_eq!(
            options.root_url(),
            "https://oss-cn-hangzhou-internal.aliyuncs.com"
        );
        assert_eq!(
            options.base_url(),
            "https://xuetube.oss-cn-shanghai-internal.aliyuncs.com"
        );
    }

    #[test]
    fn options_new_normal_2() {
        let options = oss::Options::new()
            .with_access_key_id("access_key_id")
            .with_access_key_secret("access_key_secret")
            .with_region("oss-cn-shanghai")
            .with_bucket("xtoss-ex")
            .with_secret(true)
            .with_internal(false);

        let host = "oss-cn-shanghai.aliyuncs.com";
        let root_url = "https://oss-cn-hangzhou.aliyuncs.com";
        let base_url = "https://xtoss-ex.oss-cn-shanghai.aliyuncs.com";

        assert_eq!(options.host(), host);
        assert_eq!(options.root_url(), root_url);
        assert_eq!(options.base_url(), base_url);
    }

    #[test]
    fn options_new_endpoint() {
        let options = oss::Options::new()
            .with_access_key_id("access_key_id")
            .with_access_key_secret("access_key_secret")
            .with_bucket("xtoss-ex1")
            .with_cname(true)
            .with_endpoint("https://cdn.xuetube.com")
            .with_internal(false)
            .with_region("oss-cn-shanghai")
            .with_secret(true)
            // .with_sts_token("sts token")
            .with_timeout(60);

        let host = "cdn.xuetube.com";
        let root_url = "https://oss-cn-hangzhou.aliyuncs.com";
        let base_url = "https://cdn.xuetube.com";

        assert_eq!(options.host(), host);
        assert_eq!(options.root_url(), root_url);
        assert_eq!(options.base_url(), base_url);
    }
}
