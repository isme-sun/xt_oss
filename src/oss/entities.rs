use crate::oss;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

pub(crate) mod inner {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Serialize, Deserialize)]
    pub struct RefererList {
        #[serde(rename = "Referer")]
        pub referer: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RefererBlacklist {
        #[serde(rename = "Referer")]
        pub referer: Option<Vec<String>>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct RefererConfiguration {
        #[serde(rename = "AllowEmptyReferer")]
        pub allow_empty_referer: bool,
        #[serde(rename = "AllowTruncateQueryString")]
        pub allow_truncate_query_string: bool,
        #[serde(rename = "TruncatePath")]
        pub truncate_path: bool,
        #[serde(rename = "RefererList")]
        pub referer_list: Option<RefererList>,
        #[serde(rename = "RefererBlacklist")]
        pub referer_blacklist: Option<RefererBlacklist>,
    }
}

pub mod cname;
pub mod cors;
pub mod encryption;
pub mod lifecycle;
pub mod private;
pub mod style;
pub mod tag;
pub mod version;

#[derive(Debug, Serialize, Deserialize)]
pub struct WormConfiguration {
    #[serde(rename = "WormId")]
    pub worm_id: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i32,
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
}

/// 指定存储空间的数据容灾类型
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "DataRedundancyType")]
pub enum DataRedundancyType {
    /// 本地冗余LRS将您的数据冗余存储在同一个可用区的不同存储设备上，可支持两个存储设备并发损坏时，仍维持数据不丢失，可正常访问
    #[default]
    LRS,
    /// 同城冗余ZRS采用多可用区（AZ）内的数据冗余存储机制，将用户的数据冗余存储在同一地域（Region）的多个可用区。当某个可用区不可用时，仍然能够保障数据的正常访问
    ZRS,
}

#[derive(Debug, Default)]
pub enum OssAcl {
    PublicReadWrite,
    #[default]
    PublicRead,
    Private,
}

impl Display for OssAcl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
            Self::Private => "private",
        };
        write!(f, "{}", desc)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ObjectACL {
    #[default]
    Default,
    PublicReadWrite,
    PublicRead,
    Private,
}

impl Display for ObjectACL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Default => "default",
            Self::PublicReadWrite => "public-read-write",
            Self::PublicRead => "public-read",
            Self::Private => "private",
        };
        write!(f, "{}", value)
    }
}

/// OSS 存储类型
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum StorageClass {
    /// 标准存储
    #[default]
    Standard,
    /// 低频访问存储
    IA,
    /// 归档存储
    Archive,
}

impl Display for StorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Archive => "Archive",
                Self::IA => "IA",
                Self::Standard => "STANDARD",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum ServerSideEncryption {
    //使用OSS完全托管密钥进行加解密（SSE-OSS）。
    #[default]
    AES256,
    // 使用KMS托管密钥进行加解密。
    KMS,
    // 国密SM4算法。
    SM4,
}

#[derive(Default)]
pub enum ContentDisposition<'a> {
    #[default]
    INLINE,
    ATTACHMENT(Option<&'a str>),
}

impl<'a> Display for ContentDisposition<'a> {
    // TODO 协议完善
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            Self::INLINE => "inline".to_string(),
            Self::ATTACHMENT(Some(filename)) => format!("attachment;filename={}", filename),
            Self::ATTACHMENT(None) => "attachment".to_string(),
        };
        write!(f, "{}", content)
    }
}

#[derive(Default)]
pub enum ContentEncoding {
    #[default]
    IDENTITY,
    GZIP,
    COMPRESS,
    DEFLATE,
    BR,
}

impl Display for ContentEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IDENTITY => "identity",
                Self::GZIP => "gzip",
                Self::COMPRESS => "compress",
                Self::DEFLATE => "deflate",
                Self::BR => "br",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Buckets {
    #[serde(rename(deserialize = "Bucket"))]
    pub bucket: Vec<Bucket>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListAllMyBucketsResult {
    #[serde(rename(deserialize = "Owner"))]
    pub owner: Owner,
    #[serde(rename(deserialize = "Buckets"))]
    pub buckets: Buckets,
    // #[serde(rename(deserialize = "Buckets"))]
    // pub buckets: Vec<Bucket>
}

impl From<oss::Bytes> for ListAllMyBucketsResult {
    fn from(data: oss::Bytes) -> Self {
        let content = String::from_utf8_lossy(&data);
        quick_xml::de::from_str::<Self>(&content).unwrap()
    }
}

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
pub struct Object {
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
pub struct ListBucketResult2 {
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
    pub contents: Option<Vec<Object>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
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
    #[serde(rename = "AccessMonitor")]
    pub access_monitor: Option<String>,
    #[serde(rename = "CreationDate", with = "private::serde_date::utc")]
    pub creation_date: DateTime<Utc>,
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: String,
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: String,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "TransferAcceleration")]
    pub transfer_acceleration: Option<String>,
    #[serde(rename = "CrossRegionReplication")]
    pub cross_region_replication: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: Option<String>,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: Option<AccessControlList>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "BucketPolicy")]
    pub bucket_policy: Option<BucketPolicy>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BucketInfo {
    #[serde(rename(deserialize = "Bucket"))]
    pub bucket: Bucket,
}

/// OSS 区域信息
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RegionInfo {
    #[serde(rename = "AccelerateEndpoint")]
    pub accelerate_endpoint: String,
    #[serde(rename = "InternalEndpoint")]
    pub internal_endpoint: String,
    #[serde(rename = "InternetEndpoint")]
    pub internet_endpoint: String,
    #[serde(rename = "Region")]
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionInfoList {
    #[serde(rename = "RegionInfo")]
    pub region_info: Vec<RegionInfo>,
}

/// Bucket所在的地域
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "$value")]
pub struct LocationConstraint(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessControlPolicy {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "AccessControlList")]
    pub access_control_list: AccessControlList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonPrefixes {
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contents {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub etag: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Size")]
    pub size: i32,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker")]
    pub marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
    #[serde(rename = "Contents")]
    pub contents: Option<Vec<Contents>>,
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<Vec<CommonPrefixes>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefererConfiguration {
    #[serde(rename = "AllowEmptyReferer")]
    pub allow_empty_referer: bool,
    #[serde(rename = "AllowTruncateQueryString")]
    pub allow_truncate_query_string: bool,
    #[serde(rename = "TruncatePath")]
    pub truncate_path: bool,
    #[serde(rename = "RefererList")]
    pub referer_list: Vec<String>,
    #[serde(rename = "RefererBlacklist")]
    pub referer_blacklist: Vec<String>,
}

impl Default for RefererConfiguration {
    fn default() -> Self {
        Self {
            allow_empty_referer: false,
            allow_truncate_query_string: true,
            truncate_path: true,
            referer_list: Default::default(),
            referer_blacklist: Default::default(),
        }
    }
}

#[allow(unused)]
impl RefererConfiguration {
    pub(crate) fn from_inner(config: inner::RefererConfiguration) -> Self {
        let mut referer_list: Vec<String> = Vec::new();
        let mut referer_blacklist: Vec<String> = Vec::new();

        if let Some(inner_referer_list) = config.referer_list {
            if let Some(referer) = inner_referer_list.referer {
                for url in referer {
                    referer_list.push(url);
                }
            }
        }

        if let Some(inner_referer_blacklist) = config.referer_blacklist {
            if let Some(referer) = inner_referer_blacklist.referer {
                for url in referer {
                    referer_blacklist.push(url);
                }
            }
        }

        let config = RefererConfiguration {
            allow_empty_referer: config.allow_empty_referer,
            allow_truncate_query_string: config.allow_truncate_query_string,
            truncate_path: config.truncate_path,
            referer_list,
            referer_blacklist,
        };
        config
    }

    pub(crate) fn to_inner(&self) -> inner::RefererConfiguration {
        let referer_list = {
            if self.referer_list.len() > 0 {
                Some(inner::RefererList {
                    referer: Some({
                        let mut referer: Vec<String> = Vec::new();
                        for url in &self.referer_list {
                            referer.push(url.to_string())
                        }
                        referer
                    }),
                })
            } else {
                None
            }
        };
        let referer_blacklist = {
            if self.referer_blacklist.len() > 0 {
                Some(inner::RefererBlacklist {
                    referer: Some({
                        let mut referer: Vec<String> = Vec::new();
                        for url in &self.referer_blacklist {
                            referer.push(url.to_string())
                        }
                        referer
                    }),
                })
            } else {
                None
            }
        };
        let config = inner::RefererConfiguration {
            allow_empty_referer: self.allow_empty_referer,
            allow_truncate_query_string: self.allow_truncate_query_string,
            truncate_path: self.truncate_path,
            referer_list,
            referer_blacklist,
        };
        config
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAccelerationConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use chrono::{
        DateTime::{self},
        FixedOffset, Local, NaiveDateTime, TimeZone,
        Utc::{self},
    };

    use crate::oss::entities::{inner, TransferAccelerationConfiguration};

    use super::ListAllMyBucketsResult;

    #[test]
    fn bucket() {
        // include!()
        let xml = r#"
<ListAllMyBucketsResult>
<Owner>
    <ID>512**</ID>
    <DisplayName>51264</DisplayName>
</Owner>
<Buckets>
    <Bucket>
        <CreationDate>2014-02-17T18:12:43.000Z</CreationDate>
        <ExtranetEndpoint>oss-cn-shanghai.aliyuncs.com</ExtranetEndpoint>
        <IntranetEndpoint>oss-cn-shanghai-internal.aliyuncs.com</IntranetEndpoint>
        <Location>oss-cn-shanghai</Location>
        <Name>app-base-oss</Name>
        <Region>cn-shanghai</Region>
        <StorageClass>Standard</StorageClass>
    </Bucket>
    <Bucket>
        <CreationDate>2014-02-25T11:21:04.000Z</CreationDate>
        <ExtranetEndpoint>oss-cn-hangzhou.aliyuncs.com</ExtranetEndpoint>
        <IntranetEndpoint>oss-cn-hangzhou-internal.aliyuncs.com</IntranetEndpoint>
        <Location>oss-cn-hangzhou</Location>
        <Name>mybucket</Name>
        <Region>cn-hangzhou</Region>
        <StorageClass>IA</StorageClass>
    </Bucket>
</Buckets>
</ListAllMyBucketsResult>
"#;
        let obj: ListAllMyBucketsResult = quick_xml::de::from_str(&xml).unwrap();
        println!("{:#?}", &obj);
    }

    #[test]
    fn referer_configuration() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<RefererConfiguration>
  <AllowEmptyReferer>false</AllowEmptyReferer>
  <AllowTruncateQueryString>true</AllowTruncateQueryString>
  <TruncatePath>true</TruncatePath>
  <RefererList>
    <Referer>http://www.aliyun.com</Referer>
    <Referer>https://www.aliyun.com</Referer>
    <Referer>http://www.*.com</Referer>
    <Referer>https://www.?.aliyuncs.com</Referer>
  </RefererList>
  <RefererBlacklist>
    <Referer>http://www.refuse.com</Referer>
    <Referer>https://*.hack.com</Referer>
    <Referer>http://ban.*.com</Referer>
    <Referer>https://www.?.deny.com</Referer>
  </RefererBlacklist>
</RefererConfiguration>"#;

        let object: inner::RefererConfiguration = quick_xml::de::from_str(&content).unwrap();
        println!("{:#?}", object);
    }

    #[test]
    fn transfer_acceleration_configuration() {
        let xml = r#"<TransferAccelerationConfiguration>
  <Enabled>true</Enabled>
</TransferAccelerationConfiguration>"#;
        let object1: TransferAccelerationConfiguration = quick_xml::de::from_str(&xml).unwrap();

        let object2 = TransferAccelerationConfiguration { enabled: true };

        assert_eq!(object1.enabled, object2.enabled)
    }
}
