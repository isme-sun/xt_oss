#![doc = include_str! ("../README.md")]

pub(crate) mod api;
pub mod arguments;
pub mod entities;
pub(crate) mod util;

use bytes::Bytes;
use reqwest::{
    header::{self, HOST},
    header::{HeaderMap, HeaderValue},
    StatusCode, Url,
};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use std::time::Duration;
use std::{env, str::FromStr};
use util::{Authorization, RequestOptions};

/// *阿里云OSS服务地址*
pub const OSS_BASE_URL: &'static str = "aliyuncs.com";
/// *默认区域*
pub const DEFAULT_REGION: &'static str = "oss-cn-hangzhou";

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
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

#[derive(Debug, Default)]
pub struct OssData<T> {
    pub headers: HeaderMap,
    pub data: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OssOptions<'a> {
    pub access_key_id: &'a str,
    pub access_key_secret: &'a str,
    pub sts_token: &'a str,
    pub bucket: &'a str,
    pub endpoint: &'a str,
    pub region: &'a str,
    pub internal: bool,
    pub cname: bool,
    pub is_request_pay: bool,
    pub secure: bool,
    pub timeout: u64,
}

impl<'a> Default for OssOptions<'a> {
    fn default() -> Self {
        Self {
            access_key_id: Default::default(),
            access_key_secret: Default::default(),
            sts_token: Default::default(),
            bucket: Default::default(),
            endpoint: OSS_BASE_URL,
            region: DEFAULT_REGION,
            internal: false,
            cname: false,
            is_request_pay: false,
            secure: true,
            timeout: 60,
        }
    }
}

#[allow(dead_code)]
impl<'a> OssOptions<'a>{

    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_env(mut self) -> Self {
        if let Ok(access_key_id) = env::var("OSS_ACCESS_KEY_ID") {
            self.access_key_id = access_key_id.as_str::<'a>();
        }
        if let Ok(_access_key_secret) = env::var("OSS_ACCESS_KEY_SECRET") {
            self.access_key_secret = "123123";
        }
        if let Ok(_sts_token) = env::var("OSS_STS_TOKEN") {
            self.sts_token = "123123";
        }
        if let Ok(_oss_bucket) = env::var("OSS_BUCKET") {
            self.bucket = "asdsa";
        }
        if let Ok(_oss_region) = env::var("OSS_REGION") {
            self.region = "2134234342";
        }
        if let Ok(value) = env::var("OSS_INTERNAL") {
            self.internal = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_CNAME") {
            self.cname = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_IS_REQUEST_PAY") {
            self.is_request_pay = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_SECURE") {
            self.secure = value.parse::<bool>().unwrap_or(false);
        }
        if let Ok(value) = env::var("OSS_TIMEOUT") {
            self.timeout = value.parse::<u64>().unwrap_or(60);
        }

        self
    }


    // pub fn from_env() -> Self {
    //     let mut options = OssOptions::default();

    //     if let Ok(access_key_id) = env::var("OSS_ACCESS_KEY_ID") {
    //         options.access_key_id = &*access_key_id
    //     }
    //     if let Ok(access_key_secret) = env::var("OSS_ACCESS_KEY_SECRET") {
    //         options.access_key_secret = &*access_key_secret;
    //     }
    //     if let Ok(sts_token) = env::var("OSS_STS_TOKEN") {
    //         options.sts_token = &*sts_token;
    //     }
    //     if let Ok(oss_bucket) = env::var("OSS_BUCKET") {
    //         options.bucket = &*oss_bucket;
    //     }
    //     if let Ok(oss_region) = env::var("OSS_REGION") {
    //         options.region = &*oss_region;
    //     }
    //     if let Ok(value) = env::var("OSS_INTERNAL") {
    //         options.internal = value.parse::<bool>().unwrap_or(false);
    //     }
    //     if let Ok(value) = env::var("OSS_CNAME") {
    //         options.cname = value.parse::<bool>().unwrap_or(false);
    //     }
    //     if let Ok(value) = env::var("OSS_IS_REQUEST_PAY") {
    //         options.is_request_pay = value.parse::<bool>().unwrap_or(false);
    //     }
    //     if let Ok(value) = env::var("OSS_SECURE") {
    //         options.secure = value.parse::<bool>().unwrap_or(false);
    //     }
    //     if let Ok(value) = env::var("OSS_TIMEOUT") {
    //         options.timeout = value.parse::<u64>().unwrap_or(60);
    //     }
    //     options
    // }

    pub fn common_headers(&self) -> HeaderMap {
        let mut headers = header::HeaderMap::new();
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

    fn schema(&self) -> String {
        match self.secure {
            true => "https".to_string(),
            false => "http".to_string(),
        }
    }

    fn host(&self) -> String {
        match self.internal {
            true => {
                format!("{}-internal.{}", self.region, self.endpoint,)
            }
            false => {
                format!("{}.{}", self.region, self.endpoint,)
            }
        }
    }

    pub fn root_url(&self) -> String {
        format!("{}://{}", self.schema(), self.host()).to_string()
    }

    pub fn base_url(&self) -> String {
        format!("{}://{}.{}", self.schema(), self.bucket, self.host()).to_string()
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct OssClient<'a> {
    pub options: OssOptions<'a>,
    client: reqwest::Client,
}
// *----------------------------------------------------------------------------------
/// 初始化，私有方法
impl<'a> OssClient<'a> {
    #[allow(dead_code)]
    pub fn builder(options: OssOptions) -> Self {
        let common_headers = options.common_headers();
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(options.timeout))
            .default_headers(common_headers)
            .build()
            .unwrap();
        OssClient { options, client }
    }

    async fn general_request(
        &self,
        options: RequestOptions,
    ) -> Result<(StatusCode, HeaderMap, Bytes), OssError> {
        let RequestOptions {
            url,
            auth,
            headers,
            data,
        } = options;

        let url = Url::from_str(&url[..]).unwrap();
        let host = url.host().unwrap();

        let value = auth
            .to_value(&self.options.access_key_id, &self.options.access_key_secret)
            .to_string();

        let builder = self
            .client
            .request(auth.verb, url.to_string())
            .header(HOST, host.to_string())
            .header(header::DATE, crate::util::get_gmt_date(&auth.date))
            .header(header::AUTHORIZATION, value);

        let builder = builder.headers(headers.unwrap());
        let builder = builder.body(data.unwrap());

        let response = builder.send().await.unwrap();

        let status = response.status();
        let headers = response.headers().clone();
        let data = response.bytes().await.unwrap();

        if !status.is_success() {
            let content = String::from_utf8_lossy(&data);
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        } else {
            Ok((status, headers, data))
        }
    }

    async fn request(
        &self,
        url: String,
        auth: Authorization,
    ) -> Result<(StatusCode, HeaderMap, Bytes), OssError> {
        let value = auth
            .to_value(&self.options.access_key_id, &self.options.access_key_secret)
            .to_string();
        let request = self
            .client
            .request(auth.verb, url)
            .header(header::DATE, crate::util::get_gmt_date(&auth.date))
            .header(header::AUTHORIZATION, value);

        let response = request.send().await.unwrap();

        let status = response.status();
        let headers = response.headers().clone();

        let data = response.bytes().await.unwrap();

        if !status.is_success() {
            let content = String::from_utf8_lossy(&data);
            let oss_error: OssError = serde_xml_rs::from_str(&content).unwrap();
            Err(oss_error)
        } else {
            Ok((status, headers, data))
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{arguments as args, util::Authorization};
    use reqwest::header::HeaderMap;

    #[test]
    fn test_create_bucket_configuration() {
        let cfg = args::CreateBucketConfiguration {
            storage_class: args::StorageClass::Standard,
            data_redundancy_type: args::DataRedundancyType::LRS,
        };
        let rs = serde_xml_rs::to_string(&cfg).unwrap();
        println!("{}", "-".repeat(60));
        println!("{}", rs);
        println!("{}", "-".repeat(60));
        assert!(true)
    }

    #[test]
    fn test_http_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-oss-acl", "private".parse().unwrap());
        headers.insert(
            "x-oss-resource-group-id",
            "rg-aek27tc********".parse().unwrap(),
        );

        let _auth = Authorization {
            ..Default::default()
        };

        println!("{:#?}", headers);
    }
}
