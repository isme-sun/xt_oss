use crate::OSS_BASE_URL;
use crate::{utils::hmac_sha1, DEFAULT_REGION};
use http::{header, HeaderMap, HeaderValue};
use reqwest::Request;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_xml_rs;
use std::env;
use std::fmt::{self, Display};



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
        write!(f, "Sorry, something is wrong! Please Try Again!")
    }
}

#[derive(Debug)]
pub struct OssData<T> {
    pub request:  Request,
    // pub response: Response,
    pub data: T
}

/// OSS 返回结果
// pub type OssResult = Result<(), Box<dyn std::error::Error>>;
pub type OssResult<T> = Result<OssData<T>, OssError>;

// {
//     "server": "AliyunOSS",
//     "date": "Wed, 22 Nov 2023 14:21:30 GMT",
//     "content-type": "application/xml",
//     "content-length": "8502",
//     "connection": "keep-alive",
//     "x-oss-request-id": "655E0E6A6D612F3735A8B674",
//     "x-oss-server-time": "12",
// }

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct Headers {
//     server: String,
//     date: String,
//     #[serde(rename = "content-type")]
//     content_type: String,
//     #[serde(rename = "content_length")]
//     content_length: i32,
//     connection: String,
//     #[serde(rename = "x-oss-request-id")]
//     x_oss_request_id: String,
//     #[serde(rename = "x-oss-server-time")]
//     x_oss_server_time: i32,
// }

/*
<BucketStat>
  <Storage>1600</Storage>
  <ObjectCount>230</ObjectCount>
  <MultipartUploadCount>40</MultipartUploadCount>
  <LiveChannelCount>4</LiveChannelCount>
  <LastModifiedTime>1643341269</LastModifiedTime>
  <StandardStorage>430</StandardStorage>
  <StandardObjectCount>66</StandardObjectCount>
  <InfrequentAccessStorage>2359296</InfrequentAccessStorage>
  <InfrequentAccessRealStorage>360</InfrequentAccessRealStorage>
  <InfrequentAccessObjectCount>54</InfrequentAccessObjectCount>
  <ArchiveStorage>2949120</ArchiveStorage>
  <ArchiveRealStorage>450</ArchiveRealStorage>
  <ArchiveObjectCount>74</ArchiveObjectCount>
  <ColdArchiveStorage>2359296</ColdArchiveStorage>
  <ColdArchiveRealStorage>360</ColdArchiveRealStorage>
  <ColdArchiveObjectCount>36</ColdArchiveObjectCount>
</BucketStat>

*/

#[derive(Debug,Default, Serialize, Deserialize)]
pub struct BucketStat {
  /// Bucket的总存储量，单位字节。
  #[serde(rename(deserialize = "Storage"))]
  pub storage: String,
  /// Bucket中总的Object数量
  #[serde(rename(deserialize = "ObjectCount"))]
  pub object_count: String,
  #[serde(rename(deserialize = "MultipartUploadCount"))]
  pub multipart_upload_count: String,
  #[serde(rename(deserialize = "LiveChannelCount"))]
  pub live_channel_count: String,
  #[serde(rename(deserialize = "LastModifiedTime"))]
  pub last_modified_time:String,
  #[serde(rename(deserialize = "StandardStorage"))]
  pub standard_storage:String,
  #[serde(rename(deserialize = "StandardObjectCount"))]
  pub standard_object_count:String,
  #[serde(rename(deserialize = "InfrequentAccessStorage"))]
  pub infrequent_access_storage:String,
  #[serde(rename(deserialize = "InfrequentAccessRealStorage"))]
  pub infrequent_access_real_storage:String,
  #[serde(rename(deserialize = "InfrequentAccessObjectCount"))]
  pub infrequent_access_object_count:String,
  #[serde(rename(deserialize = "ArchiveStorage"))]
  pub archive_storage:String,
  #[serde(rename(deserialize = "ArchiveRealStorage"))]
  pub archive_real_storage:String,
  #[serde(rename(deserialize = "ArchiveObjectCount"))]
  pub archive_object_count:String,
  #[serde(rename(deserialize = "ColdArchiveStorage"))]
  pub cold_archive_storage:String,
  #[serde(rename(deserialize = "ColdArchiveRealStorage"))]
  pub cold_archive_real_storage:String,
  #[serde(rename(deserialize = "ColdArchiveObjectCount"))]
  pub cold_archive_object_count:String
}

/// OSS 区域信息
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RegionInfo {
    #[serde(rename(serialize = "AccelerateEndpoint", deserialize = "AccelerateEndpoint"))]
    pub accelerate_endpoint: String,
    #[serde(rename(serialize = "InternalEndpoint", deserialize = "InternalEndpoint"))]
    pub internal_endpoint: String,
    #[serde(rename(serialize = "InternetEndpoint", deserialize = "InternetEndpoint"))]
    pub internet_endpoint: String,
    #[serde(rename(serialize = "Region", deserialize = "Region"))]
    pub region: String,
}

// AccelerateEndpoint
pub struct RegionInfoResult {
    pub headers: HeaderMap,
    pub region_info_list: Vec<RegionInfo>,
}

// pub type RegionInfoList = Vec<RegionInfo>;


/// OSS api 请求参数
pub struct OssParams {}

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

    pub fn common_headers(&self) -> http::HeaderMap {
        let mut headers = header::HeaderMap::new();

        // let host = self.get_host().parse().unwrap();
        // headers.insert(header::HOST, host);
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );

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
    pub verb: http::Method,
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
    pub fn compute(&self) -> [u8; 20] {
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


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;

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
    fn temp() {}

    #[test]
    fn is_work() {
        let options = get_options();
        println!("{:?}", options.common_headers());
        assert_eq!(1, 1);
    }
}
