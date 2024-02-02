pub(crate) const BASE_URL: &str = "aliyuncs.com";
pub(crate) const DEFAULT_REGION: &str = "oss-cn-hangzhou";
pub(crate) const USER_AGENT: &str = "xt oss/0.1";
pub(crate) const DEFAULT_CONTENT_TYPE: &str = "application/octet-stream";
pub(crate) const DEFAULT_CONNECT_TIMEOUT: u64 = 180;
pub(crate) const DEFAULT_TIMEOUT: u64 = 60;
pub(crate) const GMT_DATE_FMT: &str = "%a, %d %b %Y %H:%M:%S GMT";
// pub(crate) const XML_DOCTYPE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

use std::time::Duration;

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
pub mod api;

// core
use super::oss::{
  self,
  http::header::{AUTHORIZATION, CONTENT_TYPE, DATE},
};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmacsha1;
use reqwest::{header::HeaderMap, Response, Result};

#[allow(unused)]
#[derive(Debug)]
struct Authorization<'a> {
  access_key_id: &'a str,
  access_key_secret: &'a str,
  sts_token: Option<&'a str>,
  headers: &'a http::HeaderMap,
  method: &'a http::Method,
  date: &'a String,
  resourse: Option<&'a str>,
}

impl<'a> Authorization<'a> {
  fn complute(&self) -> String {
    format!("OSS {}:{}", self.access_key_id, self.signature())
  }

  fn headers_str(&self) -> String {
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
    value.join("").to_string()
  }

  fn signature(&self) -> String {
    let header_str = self.headers_str();
    let value = format!(
      "{VERB}\n\n{ContentType}\n{Date}\n{Header}{Resource}",
      VERB = self.method,
      Header = header_str,
      ContentType = oss::DEFAULT_CONTENT_TYPE,
      Date = self.date,
      Resource = self.resourse.unwrap_or("/")
    );
    // dbg!(println!("{}", value));
    let key = self.access_key_secret.as_bytes();
    let message = value.as_bytes();
    let value = hmacsha1::hmac_sha1(key, message);
    let encoded = general_purpose::STANDARD.encode(value.as_slice());
    encoded
  }
}

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
    Authorization {
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

    let timeout = Duration::from_secs(timeout.unwrap_or(oss::DEFAULT_TIMEOUT));
    self
      .request
      .client
      .request(self.method.to_owned(), self.url)
      .headers(headers)
      .timeout(timeout)
      .body(self.body.to_owned())
      .send()
      .await
  }
}

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

  pub fn with_sts_token(mut self, value: Option<&'a str>) -> Self {
    self.sts_token = value;
    self
  }

  pub fn task(&self) -> RequestTask<'_> {
    RequestTask::new(&self)
  }
}

#[derive(Debug, Clone, Default)]
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

impl<'a> Options<'a> {
  pub fn new() -> Self {
    Self {
      region: oss::DEFAULT_REGION,
      internal: false,
      cname: false,
      is_request_pay: false,
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

  pub fn object_url(&self, object: &'a str) -> String {
    format!("{}/{}", self.base_url(), object)
  }

  fn schema(&self) -> String {
    match self.secure {
      true => "https".to_string(),
      false => "http".to_string(),
    }
  }

  fn host(&self) -> String {
    if self.cname {
      if self.endpoint.is_empty() {
        panic!("must set endpoint");
      }
      let https_prefix = "https://";
      let http_prefix = "http://";
      self
        .endpoint
        .strip_prefix(https_prefix)
        .or_else(|| self.endpoint.strip_prefix(http_prefix))
        .unwrap_or(&self.endpoint)
        .to_string()
    } else {
      format!(
        "{}{}.{}",
        self.region,
        if self.internal { "-internal" } else { "" },
        oss::BASE_URL
      )
    }
  }
}

#[derive(Debug, Default)]
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
  fn options_new_normal_2() {
    let options = oss::Options::new()
      .with_access_key_id("access_key_id")
      .with_access_key_secret("access_key_secret")
      .with_region("oss-sn-shanghai1")
      .with_bucket("xtoss-t1")
      .with_secret(false);

    let host = "oss-sn-shanghai1.aliyuncs.com";
    let root_url = "http://oss-sn-shanghai1.aliyuncs.com";
    let base_url = "http://xtoss-t1.oss-sn-shanghai1.aliyuncs.com";

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
