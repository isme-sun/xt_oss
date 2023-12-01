#[allow(unused_imports)]
use base64::{engine::general_purpose, Engine as _};
#[allow(unused_imports)]
use chrono::{DateTime, Utc};
use hmacsha1;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
#[allow(unused_imports)]
use std::{str::from_utf8, time::Duration};
use xt_oss::arguments::{CreateBucketConfiguration, StorageClass};

use bytes::Bytes;
#[allow(unused_imports)]
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, DATE},
    Client, IntoUrl, Method, StatusCode, Url,
};

#[derive(Debug, Default)]
pub struct OSSData<T> {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub data: T,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct OSSError {
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

impl Display for OSSError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

pub type OSSResult<T> = Result<OSSData<T>, OSSError>;

#[allow(unused)]
#[derive(Debug)]
pub struct OSSRequest<'a> {
    access_key_id: Option<&'a str>,
    access_key_secret: Option<&'a str>,
    sts_token: Option<&'a str>,
    timeout: u64,
    client: Client,
    bucket: Option<String>,
    object: Option<String>,
    method: Method,
    headers: Option<HeaderMap>,
    resourse: Option<&'a str>,
    body: Option<Bytes>,
}

impl<'a> Default for OSSRequest<'a> {
    fn default() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(CONTENT_TYPE, Self::DEFAULT_CONTENT_TYPE.parse().unwrap());
        let client = Client::builder()
            .default_headers(default_headers)
            .user_agent(Self::USER_AGENT)
            .connect_timeout(Duration::from_secs(Self::DEFAULT_CONNECT_TIMEOUT))
            .build()
            .unwrap();
        Self {
            access_key_id: None,
            access_key_secret: None,
            sts_token: None,
            timeout: 60,
            client,
            method: Method::GET,
            bucket: None,
            object: None,
            headers: None,
            resourse: None,
            body: None,
        }
    }
}

#[allow(unused)]
impl<'a> OSSRequest<'a> {
    const OSS_BASE_URL: &'static str = "aliyuncs.com";
    const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";
    const USER_AGENT: &'static str = "xt oss/0.1";
    const DEFAULT_CONTENT_TYPE: &'static str = "application/octet-stream";
    const DEFAULT_CONNECT_TIMEOUT: u64 = 180;
    const GMT_DATE_FMT: &'static str = "%a, %d %b %Y %H:%M:%S GMT";

    fn new() -> Self {
        Self::default()
    }

    fn access_key_id(mut self, value: &'a str) -> Self {
        self.access_key_id = Some(value);
        self
    }

    fn access_key_secret(mut self, value: &'a str) -> Self {
        self.access_key_secret = Some(value);
        self
    }

    fn sts_token(mut self, value: &'a str) -> Self {
        self.sts_token = Some(value);
        self
    }

    fn timeout(mut self, value: u64) -> Self {
        self.timeout = value;
        self
    }

    fn method(mut self, value: Method) -> Self {
        self.method = value;
        self
    }

    fn headers(mut self, value: HeaderMap) -> Self {
        self.headers = Some(value);
        self
    }

    // fn bucket(mut self, value: &'a str) -> Self {
    //     self.bucket = Some(value);
    //     self
    // }

    // fn object(mut self, value: &'a str) -> Self {
    //     self.object = Some(value);
    //     self
    // }

    fn resource(mut self, value: &'a str) -> Self {
        self.resourse = Some(value);
        self
    }

    fn body(mut self, value: Bytes) -> Self {
        self.body = Some(value);
        self
    }

    fn parse_url<T>(input: T) -> (Option<String>, Option<String>, Option<String>)
    where
        T: IntoUrl,
    {
        let url: Url = input.into_url().unwrap();
        let host = url.host().unwrap().to_string();
        if host == Self::OSS_BASE_URL {
            (None, None, None)
        } else {
            let fragment = &host[..(host.len() - Self::OSS_BASE_URL.len() - 1)];

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

    async fn execute(mut self, url: &'a str) -> OSSResult<Bytes> {
        let (_, bucket, object) = Self::parse_url(url);
        self.bucket = bucket;
        self.object = object;

        let date = Utc::now().format(OSSRequest::GMT_DATE_FMT).to_string();

        let mut headers = HeaderMap::new();
        headers.insert(DATE, date.parse().unwrap());
        headers.extend(self.headers.clone().unwrap_or(HeaderMap::new()));
        let auth = self.authorization(&date).parse().unwrap();
        headers.insert(AUTHORIZATION, auth);

        let body = self.body.unwrap_or(Bytes::new());

        let request_builder = self
            .client
            .request(self.method, url)
            .headers(headers)
            .body(body);

        let resp = request_builder.send().await.unwrap();
        let status = resp.status();
        let headers = resp.headers().clone();
        let data = resp.bytes().await.unwrap();

        if status.is_success() {
            let oss_data = OSSData {
                status,
                headers,
                data,
            };
            Ok(oss_data)
        } else {
            let content = String::from_utf8_lossy(&data);
            if content.len() > 0 {
                let oss_error: OSSError = serde_xml_rs::from_str(&content).unwrap();
                Err(oss_error)
            } else {
                if headers.contains_key("x-oss-err") {
                    let error_info = headers.get("x-oss-err").unwrap();
                    let error_info = general_purpose::STANDARD.decode(error_info).unwrap();
                    let content = String::from_utf8_lossy(&error_info);
                    let oss_error: OSSError = serde_xml_rs::from_str(&content).unwrap();
                    Err(oss_error)
                } else {
                    let oss_error = OSSError::default();
                    Err(oss_error)
                }
            }
        }
    }

    fn authorization(&self, dt: &String) -> String {
        let auth = if let (Some(key), secret) = (self.access_key_id, self.signature(dt.clone())) {
            format!("OSS {}:{}", key, secret)
        } else {
            "".into()
        };
        auth
    }

    fn signature(&self, date: String) -> String {
        let header_str = match &self.headers {
            Some(headers) => {
                let mut oss_key_name: Vec<&str> = Vec::new();
                let keys = headers.keys();
                for item in keys {
                    let name = item.as_str();
                    if name.starts_with("x-oss") {
                        oss_key_name.push(name);
                    }
                }

                oss_key_name.sort();
                let mut value: Vec<String> = Vec::new();
                for key_name in oss_key_name {
                    let key_value = headers.get(key_name).unwrap().to_str().unwrap();
                    value.push(format!("{}:{}\n", key_name, key_value));
                }
                value.join("")
            }
            None => "".to_string(),
        };

        let value = format!(
            "{VERB}\n\n{ContentType}\n{Date}\n{Header}{Resource}",
            VERB = &self.method.to_string(),
            Header = header_str,
            ContentType = OSSRequest::DEFAULT_CONTENT_TYPE,
            Date = date,
            Resource = &self.canonicalized_resource()
        );
        println!("{}", value);
        let key = self.access_key_secret.unwrap().as_bytes();
        let message = &value.as_bytes();
        let value = hmacsha1::hmac_sha1(key, message);
        let encoded = general_purpose::STANDARD.encode(value.as_slice());
        encoded
    }

    fn canonicalized_resource(&self) -> String {
        let res_path = match (self.bucket.to_owned(), self.object.to_owned()) {
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
        if let Some(res) = &self.resourse {
            format!("{}?{}", res_path, res)
        } else {
            format!("{}", res_path)
        }
    }
}

#[allow(unused)]
async fn get_file() {
    let url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg";
    let oss_req = OSSRequest::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj");

    let resp = oss_req
        // .bucket("xuetube-dev")
        // .object("upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg")
        .method(Method::GET)
        .execute(url)
        .await;

    match resp {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.data.len());
        }
        Err(oss_err) => {
            println!("{}", oss_err);
        }
    }
}

#[allow(unused)]
async fn get_file_stat() {
    let url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg?objectMeta";
    let resp = OSSRequest::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .method(Method::HEAD)
        .resource("objectMeta")
        .execute(url)
        .await;

    match resp {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers);
        }
        Err(oss_err) => {
            println!("{}", oss_err);
        }
    }
}

#[allow(unused)]
async fn get_file_head() {
    let url = "https://xuetube-dev.oss-cn-shanghai.aliyuncs.com/upload/2022/05/2d3b8dc1-6955-40de-a23b-21a1389d218f.jpg";
    let resp = OSSRequest::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .method(Method::HEAD)
        .execute(url)
        .await;

    match resp {
        Ok(oss_data) => {
            println!("{:#?}", oss_data.headers);
        }
        Err(oss_err) => {
            println!("{}", oss_err);
        }
    }
}

#[allow(unused)]
async fn get_buckets() {
    let resp = OSSRequest::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .execute("https://oss-cn-shanghai.aliyuncs.com")
        .await
        .unwrap();

    println!("status code: {}", resp.status);
    println!("headers: {:#?}", resp.headers);
    let data = String::from_utf8_lossy(&resp.data);
    println!("data: {}", data);
}

#[allow(unused)]
async fn get_regions() {
    let resp = OSSRequest::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .execute("https://oss-cn-shanghai.aliyuncs.com/?regions")
        .await;

    match resp {
        Ok(oss_data) => {
            println!("status code: {}", oss_data.status);
            println!("headers: {:#?}", oss_data.headers);
            let data = String::from_utf8_lossy(&oss_data.data);
            println!("data: {}", data);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

#[allow(unused)]
async fn create_bcuket() {
    let mut headers = HeaderMap::new();
    // headers.insert("x-oss-resource-group-id", "bababa".parse().unwrap());
    headers.insert("x-oss-acl", "private".parse().unwrap());

    let config = CreateBucketConfiguration {
        storage_class: StorageClass::Standard,
        data_redundancy_type: None
    };

    let data = serde_xml_rs::to_string(&config).unwrap();
    println!("{}", data);
    let data = Bytes::from(serde_xml_rs::to_string(&config).unwrap());

    let resp = OSSRequest::new()
        .access_key_id("LTAI5tCpYAHHsoasDTH7hfXW")
        .access_key_secret("k0JAQGp6NURoVSDuxR7BORorlejGmj")
        .method(Method::PUT)
        .headers(headers)
        .body(data)
        .execute("https://xuetube-t3.oss-cn-shanghai.aliyuncs.com/")
        .await;

    match resp {
        Ok(oss_data) => {
            println!("status code: {}", oss_data.status);
            println!("headers: {:#?}", oss_data.headers);
            let data = String::from_utf8_lossy(&oss_data.data);
            println!("data: {}", data);
        }
        Err(err) => {
            println!("{:#?}", err);
            println!("{}", err);
        }
    }
}

#[tokio::main]
async fn main() {
    // get_file_stat().await;
    // get_file_head().await;
    // get_buckets().await
    // get_regions().await;
    create_bcuket().await;
}
