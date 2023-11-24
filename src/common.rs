use crate::DEFAULT_REGION;
use crate::OSS_BASE_URL;
use http::{header, HeaderMap, HeaderValue};
use reqwest::Request;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_xml_rs;
use std::env;
use std::fmt::{self, Display};

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
    pub request: Request,
    // pub response: Response,
    pub data: T,
}

/// OSS 返回结果
// pub type OssResult = Result<(), Box<dyn std::error::Error>>;
pub type OssResult<T> = Result<OssData<T>, OssError>;

#[derive(Debug, Default, Serialize, Deserialize)]
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
    pub last_modified_time: String,
    #[serde(rename(deserialize = "StandardStorage"))]
    pub standard_storage: String,
    #[serde(rename(deserialize = "StandardObjectCount"))]
    pub standard_object_count: String,
    #[serde(rename(deserialize = "InfrequentAccessStorage"))]
    pub infrequent_access_storage: String,
    #[serde(rename(deserialize = "InfrequentAccessRealStorage"))]
    pub infrequent_access_real_storage: String,
    #[serde(rename(deserialize = "InfrequentAccessObjectCount"))]
    pub infrequent_access_object_count: String,
    #[serde(rename(deserialize = "ArchiveStorage"))]
    pub archive_storage: String,
    #[serde(rename(deserialize = "ArchiveRealStorage"))]
    pub archive_real_storage: String,
    #[serde(rename(deserialize = "ArchiveObjectCount"))]
    pub archive_object_count: String,
    #[serde(rename(deserialize = "ColdArchiveStorage"))]
    pub cold_archive_storage: String,
    #[serde(rename(deserialize = "ColdArchiveRealStorage"))]
    pub cold_archive_real_storage: String,
    #[serde(rename(deserialize = "ColdArchiveObjectCount"))]
    pub cold_archive_object_count: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OSSObject {
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "LastModified"))]
    pub last_modified: String,
    #[serde(rename(deserialize = "ETag"))]
    pub etag: String,
    #[serde(rename(deserialize = "Size"))]
    pub size: i32,
    #[serde(rename(deserialize = "StorageClass"))]
    pub storage_class: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketResult {
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "Prefix"))]
    pub prefix: String,
    #[serde(rename(deserialize = "MaxKeys"))]
    pub max_keys: i32,
    #[serde(rename(deserialize = "EncodingType"))]
    pub encoding_type: String,
    #[serde(rename(deserialize = "IsTruncated"))]
    pub is_truncated: bool,
    #[serde(rename(deserialize = "KeyCount"))]
    pub key_count: i32,
    // #[serde(rename = "$value")]
    #[serde(rename(deserialize = "Contents"))]
    pub contents: Vec<OSSObject>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Owner {
    #[serde(rename(deserialize = "DisplayName"))]
    pub display_name: String,
    #[serde(rename(deserialize = "ID"))]
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BucketPolicy {
    #[serde(rename(deserialize = "LogBucket"))]
    pub log_bucket: String,
    #[serde(rename(deserialize = "LogPrefix"))]
    pub log_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccessControlList {
    #[serde(rename(deserialize = "Grant"))]
    pub grant: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bucket {
    #[serde(rename(deserialize = "AccessMonitor"))]
    pub access_monitor: String,
    #[serde(rename(deserialize = "CreationDate"))]
    pub creation_date: String,
    #[serde(rename(deserialize = "ExtranetEndpoint"))]
    pub extranet_endpoint: String,
    #[serde(rename(deserialize = "IntranetEndpoint"))]
    pub intranet_endpoint: String,
    #[serde(rename(deserialize = "Location"))]
    pub location: String,
    #[serde(rename(deserialize = "StorageClass"))]
    pub storage_class: StorageClass,
    #[serde(rename(deserialize = "TransferAcceleration"))]
    pub transfer_acceleration: String,
    #[serde(rename(deserialize = "CrossRegionReplication"))]
    pub cross_region_replication: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "ResourceGroupId"))]
    pub resource_group_id: String,
    #[serde(rename(deserialize = "Owner"))]
    pub owner: Owner,
    #[serde(rename(deserialize = "AccessControlList"))]
    pub access_control_list: AccessControlList,
    #[serde(rename(deserialize = "Comment"))]
    pub comment: String,
    #[serde(rename(deserialize = "BucketPolicy"))]
    pub bucket_policy: BucketPolicy,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BucketInfo {
    #[serde(rename(deserialize = "Bucket"))]
    pub bucket: Bucket,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum StorageClass {
    /// 标准存储
    #[serde(rename(deserialize = "Standard"))]
    #[default]
    Standard,
    /// 低频访问存储
    #[serde(rename(deserialize = "IA"))]
    IA,
    /// 归档存储
    #[serde(rename(deserialize = "Archive"))]
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
