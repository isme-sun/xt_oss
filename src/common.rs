use crate::OSS_BASE_URL;
use crate::{utls::hmac_sha1, DEFAULT_REGION};
use http::uri::Scheme;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::fmt::{Display, self};
use async_trait::async_trait;
use std::pin::Pin;
use std::future::Future;
use anyhow::Result as AnyResult;


/// OSS 返回结果
pub type OssResult = Result<(), Box<dyn std::error::Error> >;
/// OSS api 请求参数
pub struct OssParams {}

#[async_trait]
pub trait OssApiService {
    #[allow(non_snake_case)]
    async fn ListBuckets(&self, params: OssParams);
}
#[async_trait]
pub trait OssApiRegion {
    #[allow(non_snake_case)]
    async fn DescribeRegions(&self, params: OssParams) -> AnyResult<()>;
}
#[async_trait]
pub trait OssApiBucketRegion {}
#[async_trait]
pub trait OssApiObject {}
#[async_trait]
pub trait OssApiLiveChannel {}

////////////////////////////////////////////////////////////////////////////////////

/// *OSS HttpMethod描述*
#[derive(Debug)]
pub enum HttpMethod {
    PUT,
    GET,
    POST,
    HEAD,
    DELETE,
    OPTIONS,
}

/// OSS 存储类型
#[derive(Debug)]
pub enum StorageClass {
    /// 标准存储
    Standard,
    /// 低频访问存储
    IA,
    /// 归档存储
    Archive,
}

/// 客户端配置
///
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
    fn get_schema(&self) -> String {
        if self.secure == true {
            "https".to_string()
        } else {
            "http".to_string()
        }
    }

    pub fn host_url(&self) -> String {
        if self.internal == true {
            format!("{}-internal.{}", self.region, self.endpoint)
        } else {
            format!("{}.{}", self.region, self.endpoint)
        }
    }

    pub fn bucket_url(&self) -> String {
        let scheme = self.get_schema();
        let host_url = self.host_url();
        format!("{}://{}.{}", scheme, self.bucket, host_url)
    }

    #[allow(unused)]
    pub fn object_key_url(&self, object_key: String) -> String {
        let bucket_url = self.bucket_url();
        format!("{}/{}", bucket_url, object_key)
    }
}

/// *OSS Endpoint描述*
#[derive(Debug)]
pub struct Endpoint {
    pub region: String,
}

impl Endpoint {
    pub fn new(region: String) -> Endpoint {
        Endpoint { region }
    }
}

/**
# node sdk 返回例子
*OSS Bucket描述*
*/
#[derive(Debug)]
pub struct Bucket {
    pub name: String,
    pub region: String,
    pub creation_date: String,
    pub storage_class: StorageClass,
    pub tags: String,
}

/**
OSS Authorization 描述
*/
#[derive(Default, Debug)]
pub struct Authorization {
    pub access_key_id: String,
    pub signature: Signature,
}

// Authorization = "OSS " + AccessKeyId + ":" + Signature

impl Display for Authorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OSS{}:{}",
            &self.access_key_id,
            &self.signature.to_string()
        )
    }
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::GET
    }
}

impl Display for HttpMethod {
    #[allow(unused)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            HttpMethod::GET => "GET",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
        };
        write!(f, "{}", value)
    }
}

///
/// # 签章数据结构
///
/// ```ignore
/// Signature = base64(hmac-sha1(AccessKeySecret,
///             VERB + "\n"
///             + Content-MD5 + "\n"
///             + Content-Type + "\n"
///             + Date + "\n"
///             + CanonicalizedOSSHeaders
///             + CanonicalizedResource))
/// ```
///
#[derive(Debug, Default)]
pub struct Signature {
    pub access_key_secret: String,
    pub verb: HttpMethod,
    pub content_md5: String,
    pub content_type: String,
    pub date: String,
    pub canonicalized_oss_headers: String,
    pub canonicalized_resource: String,
}

impl Signature {
    pub fn new(access_key_secret: String) -> Self {
        Self {
            access_key_secret,
            ..Self::default()
        }
    }

    // 计算签章
    pub fn compute(&self) -> String {
        let content = format!(
            "{}\n{}\n{}\n{}\n{}{}",
            self.verb,
            self.content_md5,
            self.content_type,
            self.date,
            self.canonicalized_oss_headers,
            self.canonicalized_resource
        );
        hmac_sha1(&content, &self.access_key_secret)
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This Is Signature")
    }
}

/*
  <Code>AccessDenied</Code>
  <Message>Anonymous access is forbidden for this operation.</Message>
  <RequestId>65589C1147C61735372BA1F5</RequestId>
  <HostId>aliyuncs.com</HostId>
  <EC>0003-00001201</EC>
  <RecommendDoc>https://api.aliyun.com/troubleshoot?q=0003-00001201</RecommendDoc>
*/
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

impl fmt::Display for OssError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
     write!(f,"Sorry, something is wrong! Please Try Again!")
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use reqwest::{self, Response};
    use serde_json;
    use std::{env, error::Error};

    use crate::OssOptions;

    fn get_options() -> OssOptions {
        dotenv().ok();
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

    #[test]
    fn temp() {
    }

    #[test]
    fn is_work() {
        let options = get_options();
        let json_string = serde_json::to_string_pretty(&options).unwrap();
        println!("{}", json_string);
        println!("{}", options.bucket_url());
        println!("{}", options.host_url());
        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn is_work_async() {
        let res = reqwest::get("http://httpbin.org/get").await;

        if let Ok(response) = res {
            println!("Status: {}", response.status());
            println!("Headers:\n{:#?}", response.headers());
            let body = response.text().await;
            if let Ok(content) = body {
                println!("Body:\n{}", content);
            } else {
                println!("error")
            }
        } else {
            println!("error");
        }
    }
}
