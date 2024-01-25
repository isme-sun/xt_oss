pub(crate) const BASE_URL: &str = "aliyuncs.com";
pub(crate) const DEFAULT_REGION: &str = "oss-cn-hangzhou";
pub(crate) const USER_AGENT: &str = "xt oss/0.1";
pub(crate) const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
pub(crate) const DEFAULT_CONNECT_TIMEOUT: u64 = 180;
pub(crate) const DEFAULT_TIMEOUT: u64 = 60;
pub(crate) const GMT_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S GMT";
// pub(crate) const XML_DOCTYPE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

// re-export
pub use bytes::Bytes;
pub mod http {
    pub use reqwest::{
        header::{self, HeaderMap, HeaderName, HeaderValue},
        IntoUrl, Method, StatusCode, Url,
    };
}

// entity defined
pub mod entities;
// api impl
// pub(crate) mod api;

// core
use super::oss::{
    self,
    http::header::{AUTHORIZATION, CONTENT_TYPE, DATE},
};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmacsha1;
use reqwest::{Response, Result};
use std::time::Duration;

#[derive(Debug)]
#[allow(unused)]
struct Authorization<'a> {
    access_key_id: &'a str,
    access_key_secret: &'a str,
    sts_token: &'a str,
    bucket: Option<&'a str>,
    object: Option<&'a str>,
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
        // dbg!(println!("{}", value));
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
            method: http::Method::GET,
            headers: http::HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    fn resource(&self) -> &'a str {
        self.resource.unwrap_or("/")
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

    // pub async fn execute(&self) -> oss::Result<Bytes> {
    pub async fn execute(&self) -> oss::Result<Response> {
        self.inner_execute(None).await
    }

    // pub async fn execute_timeout(&self, value:u64) -> oss::Result<Bytes> {
    pub async fn execute_timeout(&self, value: u64) -> oss::Result<Response> {
        self.inner_execute(Some(value)).await
    }

    // async fn inner_execute(&self, timeout:Option<u64>) -> oss::Result<Bytes> {
    async fn inner_execute(&self, timeout: Option<u64>) -> oss::Result<Response> {
        // let (_, bucket, object) = Self::parse_url(self.url);

        // dbg!(println!("{:#?}", (&bucket, &object)));
        let bucket = None;
        let object = None;

        let date = Utc::now().format(oss::GMT_DATE_FMT).to_string();
        let mut headers = http::HeaderMap::new();
        headers.insert(DATE, date.parse().unwrap());
        headers.extend(self.headers.to_owned());

        // println!("{:#?}", headers);

        headers.insert(
            AUTHORIZATION,
            Authorization {
                access_key_id: self.request.access_key_id.unwrap_or_default(),
                access_key_secret: self.request.access_key_secret.unwrap_or_default(),
                sts_token: self.request.sts_token.unwrap_or_default(),
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

        let mut request_builder = self
            .request
            .client
            .request(self.method.to_owned(), self.url)
            .headers(headers)
            .body(self.body.to_owned());

        let request_builder = match timeout {
            Some(timeout) => {
                let timeout = Duration::from_secs(timeout);
                request_builder.timeout(timeout)
            }
            e => {
                let timeout = Duration::from_secs(oss::DEFAULT_TIMEOUT);
                request_builder.timeout(timeout)
            }
        };

        let result = request_builder.send().await;
        result
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

    pub fn task(&self) -> RequestTask<'_> {
        RequestTask::new(&self)
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
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
            endpoint: Default::default(),
            region: oss::DEFAULT_REGION,
            internal: false,
            cname: false,
            is_request_pay: false,
            secure: false,
            timeout: 60u64,
        }
    }
}

impl<'a> Options<'a> {
    pub fn new() -> Self {
        Self::default()
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
        self.endpoint = value;
        self.cname = true;
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
    pub fn with_is_request_pay(mut self, value: bool) -> Self {
        self.is_request_pay = value;
        self
    }

    pub fn with_secret(mut self, value: bool) -> Self {
        self.secure = value;
        self
    }
    pub fn with_timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    pub fn root_url(&self) -> String {
        format!("{}://{}", self.schema(), self.host()).to_string()
    }

    pub fn base_url(&self) -> String {
        match self.cname {
            true => format!("{}://{}", self.schema(), self.host()),
            false => format!("{}://{}.{}", self.schema(), self.bucket, self.host()).to_string(),
        }
    }

    fn schema(&self) -> String {
        match self.secure {
            true => "https".to_string(),
            false => "http".to_string(),
        }
    }

    fn host(&self) -> String {
        match self.cname {
            true => match self.endpoint.is_empty() {
                true => panic!("must set endpoint"),
                false => {
                    if self.endpoint.starts_with("http://") {
                        self.endpoint["https://".len() - 1..].to_string()
                    } else if self.endpoint.starts_with("https://") {
                        self.endpoint["http://".len() - 1..].to_string()
                    } else {
                        self.endpoint.to_string()
                    }
                }
            },
            false => match self.internal {
                true => format!("{}-internal.{}", self.region, oss::BASE_URL),
                false => format!("{}.{}", self.region, oss::BASE_URL),
            },
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
            .with_access_key_secret(options.access_key_secret);
        Self { options, request }
    }

    pub fn options(&self) -> &oss::Options {
        &self.options
    }

    pub fn with_region(mut self, value: &'a str) -> Self {
        self.options.region = value;
        self
    }

    pub fn with_bucket(mut self, value: &'a str) -> Self {
        self.options.bucket = value;
        self
    }

    pub fn with_internal(mut self, value: bool) -> Self {
        self.options.internal = value;
        self
    }

    pub fn with_cname(mut self, value: bool) -> Self {
        self.options.cname = value;
        self
    }

    pub fn with_secure(mut self, value: bool) -> Self {
        self.options.secure = value;
        self
    }

    pub fn with_timeout(mut self, value: u64) -> Self {
        self.options.timeout = value;
        self
    }
}

#[cfg(test)]
pub mod tests {
    use crate::oss;

    #[test]
    fn options_new_normal() {
        let options = oss::Options::new()
            .with_access_key_id("access_key_id")
            .with_access_key_secret("access_key_secret")
            .with_region("oss-sn-shanghai1")
            .with_bucket("xtoss-t1")
            .with_internal(true)
            .with_secret(true);

        let host = "oss-sn-shanghai1-internal.aliyuncs.com";
        let root_url = "https://oss-sn-shanghai1-internal.aliyuncs.com";
        let base_url = "https://xtoss-t1.oss-sn-shanghai1-internal.aliyuncs.com";

        assert_eq!(options.host(), host);
        assert_eq!(options.root_url(), root_url);
        assert_eq!(options.base_url(), base_url);
    }

    #[test]
    fn options_new_endpoint() {
        let options = oss::Options::new()
            .with_access_key_id("access_key_id")
            .with_access_key_secret("access_key_secret")
            .with_endpoint("http://cdn-dev.xuetube.com")
            .with_bucket("xtoss-t1")
            .with_cname(true)
            .with_secret(false);

        let host = "cdn-dev.xuetube.com";
        let root_url = "http://cdn-dev.xuetube.com";
        let base_url = "http://cdn-dev.xuetube.com";

        assert_eq!(options.host(), host);
        assert_eq!(options.root_url(), root_url);
        assert_eq!(options.base_url(), base_url);
    }
}
