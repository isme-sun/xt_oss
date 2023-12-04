use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmacsha1;
use reqwest::{
    self,
    header::{AUTHORIZATION, CONTENT_TYPE, DATE},
    IntoUrl, StatusCode, Url,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{
    cell::RefCell,
    fmt::{self, Display},
};

pub(crate) mod api;
pub mod arguments;
pub mod entities;

// re -export
pub use bytes::Bytes;
pub use reqwest::header::HeaderMap;
pub use reqwest::header::HeaderValue;
pub use reqwest::Method;

pub const BASE_URL: &'static str = "aliyuncs.com";
pub const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";
const USER_AGENT: &'static str = "xt oss/0.1";
const DEFAULT_CONTENT_TYPE: &'static str = "application/octet-stream";
const DEFAULT_CONNECT_TIMEOUT: u64 = 180;
const GMT_DATE_FMT: &'static str = "%a, %d %b %Y %H:%M:%S GMT";

#[derive(Debug, Default)]
pub struct Data<T> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub data: T,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Error {
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

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

type Result<T> = std::result::Result<Data<T>, Error>;

#[derive(Debug)]
#[allow(unused)]
struct Authorization<'a> {
    access_key_id: &'a str,
    access_key_secret: &'a str,
    sts_token: &'a str,
    bucket: Option<String>,
    object: Option<String>,
    headers: &'a HeaderMap,
    method: &'a Method,
    date: &'a String,
    resourse: Option<&'a str>,
}

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
            VERB = self.method.to_string(),
            Header = header_str,
            ContentType = crate::oss::DEFAULT_CONTENT_TYPE,
            Date = self.date,
            Resource = self.canonicalized_resource()
        );
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
            format!("{}", res_path)
        }
    }
}

#[allow(unused)]
pub struct RequestTask<'a> {
    request: &'a crate::oss::Request<'a>,
    url: &'a str,
    resourse: Option<&'a str>,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
}

impl<'a> RequestTask<'a> {
    pub fn new(request: &'a crate::oss::Request<'a>, url: &'a str) -> Self {
        Self {
            request,
            url,
            resourse: None,
            method: Method::GET,
            headers: HeaderMap::new(),
            body: Bytes::new(),
        }
    }

    pub fn resourse(mut self, value: &'a str) -> Self {
        self.resourse = Some(value);
        self
    }

    pub fn headers(mut self, value: HeaderMap) -> Self {
        self.headers = value;
        self
    }

    pub fn method(mut self, value: Method) -> Self {
        self.method = value;
        self
    }

    pub fn body(mut self, value: Bytes) -> Self {
        self.body = value;
        self
    }

    pub async fn send(&self) -> crate::oss::Result<Bytes> {
        let (_, bucket, object) = Self::parse_url(self.url);
        let date = Utc::now().format(crate::oss::GMT_DATE_FMT).to_string();
        let mut headers = HeaderMap::new();
        headers.insert(DATE, date.parse().unwrap());
        headers.extend(self.headers.to_owned());

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
                resourse: self.resourse,
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
        let headers = resp.headers().clone();
        let data = resp.bytes().await.unwrap();

        if status.is_success() {
            let oss_data = Data {
                status,
                headers,
                data,
            };
            Ok(oss_data)
        } else {
            let content = String::from_utf8_lossy(&data);
            if content.len() > 0 {
                let oss_error: Error = serde_xml_rs::from_str(&content).unwrap();
                Err(oss_error)
            } else {
                if headers.contains_key("x-oss-err") {
                    let error_info = headers.get("x-oss-err").unwrap();
                    let error_info = general_purpose::STANDARD.decode(error_info).unwrap();
                    let content = String::from_utf8_lossy(&error_info);
                    let oss_error: Error = serde_xml_rs::from_str(&content).unwrap();
                    Err(oss_error)
                } else {
                    let oss_error = Error::default();
                    Err(oss_error)
                }
            }
        }
    }

    pub fn parse_url<T>(input: T) -> (Option<String>, Option<String>, Option<String>)
    where
        T: IntoUrl,
    {
        let url: Url = input.into_url().unwrap();
        let host = url.host().unwrap().to_string();
        if host == crate::oss::BASE_URL {
            (None, None, None)
        } else {
            let fragment = &host[..(host.len() - crate::oss::BASE_URL.len() - 1)];
            let result = fragment.split_once('.');
            let (bucket, region) = match result {
                Some((bucket, region)) => (Some(bucket.to_string()), Some(region.to_string())),
                _ => (None, Some(fragment.to_string())),
            };
            let object = url.path().trim_start_matches('/');
            let object = if object == "" {
                None
            } else {
                Some(object.to_string())
            };
            (region, bucket, object)
        }
    }
}

// #[allow(unused)]
#[derive(Debug)]
pub struct Request<'a> {
    access_key_id: Option<&'a str>,
    access_key_secret: Option<&'a str>,
    sts_token: Option<&'a str>,
    timeout: u64,
    client: reqwest::Client,
}

impl<'a> Default for Request<'a> {
    fn default() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            CONTENT_TYPE,
            crate::oss::DEFAULT_CONTENT_TYPE.parse().unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .user_agent(crate::oss::USER_AGENT)
            .connect_timeout(Duration::from_secs(crate::oss::DEFAULT_CONNECT_TIMEOUT))
            .build()
            .unwrap();
        Self {
            access_key_id: None,
            access_key_secret: None,
            sts_token: None,
            timeout: 60,
            client,
        }
    }
}

#[allow(unused)]
impl<'a> Request<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn access_key_id(mut self, value: &'a str) -> Self {
        self.access_key_id = Some(value);
        self
    }

    pub fn access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = Some(value);
        self
    }

    pub fn sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = Some(value);
        self
    }

    pub fn timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    pub fn execute(&self, url: &'a str) -> RequestTask<'_> {
        RequestTask::new(self, url)
        // todo!()
    }
}

#[allow(unused)]
#[derive(Debug)]
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
            region: crate::oss::DEFAULT_REGION,
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

    pub fn endpoint(mut self, value: &'a str) -> Self {
        self.endpoint = value;
        self
    }
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
                format!("{}-internal.{}", self.region, crate::oss::BASE_URL)
            }
            false => {
                format!("{}.{}", self.region, crate::oss::BASE_URL)
            }
        }
    }
}

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Client<'a> {
    options: Options<'a>,
    request: RefCell<self::Request<'a>>,
}

impl<'a> Client<'a> {
    pub fn new(options: Options<'a>) -> Self {
        let request = self::Request::new()
            .access_key_id(options.access_key_id)
            .access_key_secret(options.access_key_secret);
        Self {
            options,
            request: RefCell::new(request),
        }
    }

    #[allow(non_snake_case)]
    pub async fn DescribeRegions(&self, _region: arguments::DescribeRegionsQuery) {
        todo!()
        // let url = {
        //     let root_url = self.options.root_url();
        //     let query_str = region.to_string();
        //     format!("{root_url}?{query_str}")
        // };
        // let data = self
        //     .request
        //     .borrow_mut()
        //     .execute(Box::leak(url.into_boxed_str()))
        //     .await
        //     .unwrap();
        // println!("{:#?}", data.headers);
        // println!("{:#?}", data.status);
        // println!("{:#?}", data.data);
    }
}
