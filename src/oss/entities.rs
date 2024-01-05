use std::fmt::{self, Display};

use crate::oss::{self, inner::option_datetime_format};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

//----------------------------------------------------------------

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

//----------------------------------------------------------------

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

//----------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub enum SSEAlgorithm {
    KMS,
    #[default]
    AES256,
    SM4,
}

impl Display for SSEAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::KMS => "KMS",
            Self::AES256 => "AES256",
            Self::SM4 => "SM4",
        };
        write!(f, "{}", value)
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
}

impl From<oss::Bytes> for ListAllMyBucketsResult {
    fn from(data: oss::Bytes) -> Self {
        let content = String::from_utf8_lossy(&data);
        quick_xml::de::from_str::<Self>(&content).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListCnameResult {
    #[serde(rename(deserialize = "Bucket"))]
    pub bucket: String,
    #[serde(rename(deserialize = "Owner"))]
    pub owner: String,
    #[serde(rename(deserialize = "Cname"))]
    pub cname: Option<Cname>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[serde(rename(deserialize = "Type"))]
    pub r#type: String,
    #[serde(rename(deserialize = "CertId"))]
    pub cert_id: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "CreationDate"))]
    pub creation_date: String,
    #[serde(rename(deserialize = "Fingerprint"))]
    pub fingerprint: String,
    #[serde(rename(deserialize = "ValidStartDate"))]
    pub valid_start_date: String,
    #[serde(rename(deserialize = "ValidEndDate"))]
    pub valid_end_date: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cname {
    #[serde(rename(deserialize = "Domain"))]
    pub domain: String,
    #[serde(rename(deserialize = "LastModified"))]
    pub last_modified: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "Certificate"))]
    pub certificate: Option<Certificate>,
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
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
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
    pub comment: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagSet {
    #[serde(rename = "Tag")]
    pub(crate) tag: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tagging {
    #[serde(rename = "TagSet")]
    pub tag_set: TagSet,
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

// -----------------------------------------------------------------

//------------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Style {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "Category")]
    pub category: Option<String>,
    #[serde(
        rename = "CreateTime",
        skip_serializing_if = "Option::is_none",
        with = "option_datetime_format"
    )]
    pub create_time: Option<DateTime<Utc>>,
    #[serde(
        rename = "LastModifyTime",
        skip_serializing_if = "Option::is_none",
        with = "option_datetime_format"
    )]
    pub last_modify_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StyleList {
    #[serde(rename = "Style")]
    pub style: Vec<Style>,
}

// ----------------------------------------------------------

/*
<ServerSideEncryptionRule>
  <ApplyServerSideEncryptionByDefault>
    <SSEAlgorithm>KMS</SSEAlgorithm>
    <KMSDataEncryption>SM4</KMSDataEncryption>
    <KMSMasterKeyID></KMSMasterKeyID>
  </ApplyServerSideEncryptionByDefault>
</ServerSideEncryptionRule>
*/

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ApplyServerSideEncryptionByDefault {
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: SSEAlgorithm,
    #[serde(rename = "KMSDataEncryption", skip_serializing_if = "Option::is_none")]
    pub kms_data_encryption: Option<String>,
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    pub(crate) apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault,
}

// ------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CORSRule {
    #[serde(rename = "AllowedOrigin")]
    pub allowed_origin: Vec<String>,
    #[serde(rename = "AllowedMethod")]
    pub allowed_method: Vec<String>,
    #[serde(rename = "AllowedHeader")]
    pub allowed_header: Option<Vec<String>>,
    #[serde(rename = "ExposeHeader")]
    pub expose_header: Option<Vec<String>>,
    #[serde(rename = "MaxAgeSeconds")]
    pub max_age_seconds: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRule")]
    pub cors_rule: Vec<CORSRule>,
    #[serde(rename = "ResponseVary", skip_serializing_if = "Option::is_none")]
    pub response_vary: Option<bool>,
}

// ----------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VersioningStatus {
    Enabled,
    Suspended,
}

impl fmt::Display for VersioningStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Enabled => "Enabled",
                Self::Suspended => "Suspended",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersioningConfiguration {
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<VersioningStatus>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeleteMarker {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "IsLatest")]
    pub is_latest: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "Owner")]
    pub owner: Owner,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Version {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "VersionId")]
    pub version_id: String,
    #[serde(rename = "IsLatest")]
    pub is_latest: bool,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(rename = "StorageClass", skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<StorageClass>,
    #[serde(rename = "Owner")]
    pub owner: Owner,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListVersionsResult {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "KeyMarker")]
    pub key_marker: String,
    #[serde(rename = "VersionIdMarker")]
    pub version_id_marker: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: u64,
    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "DeleteMarker", skip_serializing_if = "Option::is_none")]
    pub delete_marker: Option<Vec<DeleteMarker>>,
    #[serde(rename = "Version")]
    pub version: Vec<Version>,
}

// ----------------------------------------------------------------------

pub mod lifecycle {
    use super::{StorageClass, Tag};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Not {
        #[serde(rename = "Prefix")]
        pub prefix: String,
        #[serde(rename = "Tag")]
        pub tag: Tag,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Filter {
        #[serde(rename = "Not")]
        pub not: Not,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AbortMultipartUpload {
        #[serde(rename = "Days")]
        pub days: i32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct NoncurrentVersionTransition {
        #[serde(rename = "NoncurrentDays", skip_serializing_if = "Option::is_none")]
        pub noncurrent_days: Option<bool>,
        #[serde(rename = "StorageClass")]
        pub storage_class: StorageClass,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Transition {
        #[serde(rename = "Days")]
        pub days: Option<i32>,
        #[serde(rename = "StorageClass")]
        pub storage_class: StorageClass,
        #[serde(rename = "IsAccessTime", skip_serializing_if = "Option::is_none")]
        pub is_access_time: Option<bool>,
        #[serde(
            rename = "ReturnToStdWhenVisit",
            skip_serializing_if = "Option::is_none"
        )]
        pub return_to_std_when_visit: Option<bool>,
        #[serde(rename = "AllowSmallFile", skip_serializing_if = "Option::is_none")]
        pub allow_small_file: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Expiration {
        #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
        pub days: Option<i32>,
        #[serde(rename = "CreatedBeforeDate", skip_serializing_if = "Option::is_none")]
        pub created_before_date: Option<String>,
        #[serde(
            rename = "ExpiredObjectDeleteMarker",
            skip_serializing_if = "Option::is_none"
        )]
        pub expired_object_delete_marker: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct NoncurrentVersionExpiration {
        #[serde(rename = "NoncurrentDays")]
        pub noncurrent_days: i32,
    }

    #[derive(Debug, Serialize, Deserialize, Default, Clone)]
    pub struct Rule {
        #[serde(rename = "ID")]
        pub id: String,
        #[serde(rename = "Prefix")]
        pub prefix: String,
        #[serde(rename = "Status")]
        pub status: String,
        #[serde(rename = "Transition", skip_serializing_if = "Option::is_none")]
        pub transition: Option<Vec<Transition>>,
        #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
        pub filter: Option<Filter>,
        #[serde(rename = "Expiration", skip_serializing_if = "Option::is_none")]
        pub expiration: Option<Expiration>,
        #[serde(
            rename = "NoncurrentVersionExpiration",
            skip_serializing_if = "Option::is_none"
        )]
        pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
        #[serde(
            rename = "AbortMultipartUpload",
            skip_serializing_if = "Option::is_none"
        )]
        pub abort_multipart_upload: Option<AbortMultipartUpload>,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct LifecycleConfiguration {
        #[serde(rename = "Rule")]
        pub rule: Vec<Rule>,
    }

    pub mod builder {
        use crate::oss::entities::StorageClass;

        use super::{
            AbortMultipartUpload, Expiration, Filter, LifecycleConfiguration,
            NoncurrentVersionExpiration, Rule, Transition,
        };

        #[derive(Default)]
        #[allow(unused)]
        pub struct ExpirationBuilder {
            days: Option<i32>,
            created_before_date: Option<String>,
            expired_object_delete_marker: Option<bool>,
        }

        #[allow(unused)]
        impl ExpirationBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn days(mut self, value: i32) -> Self {
                self.days = Some(value);
                self
            }

            pub fn created_before_date(mut self, value: String) -> Self {
                self.created_before_date = Some(value);
                self
            }

            pub fn expired_object_delete_marker(mut self, value: bool) -> Self {
                self.expired_object_delete_marker = Some(value);
                self
            }

            pub fn builder(&self) -> Expiration {
                Expiration {
                    days: self.days,
                    created_before_date: self.created_before_date.clone(),
                    expired_object_delete_marker: self.expired_object_delete_marker,
                }
            }
        }

        #[derive(Default)]
        #[allow(unused)]
        pub struct TransitionBuilder {
            days: Option<i32>,
            storage_class: StorageClass,
            is_access_time: Option<bool>,
            return_to_std_when_visit: Option<bool>,
            allow_small_file: Option<bool>,
        }

        impl TransitionBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn days(mut self, value: i32) -> Self {
                self.days = Some(value);
                self
            }

            pub fn standard_storage(mut self, value: StorageClass) -> Self {
                self.storage_class = value;
                self
            }

            pub fn is_access_time(mut self, value: bool) -> Self {
                self.is_access_time = Some(value);
                self
            }

            pub fn return_to_std_when_visit(mut self, value: bool) -> Self {
                self.return_to_std_when_visit = Some(value);
                self
            }

            pub fn allow_small_file(mut self, value: bool) -> Self {
                self.allow_small_file = Some(value);
                self
            }

            pub fn builder(&self) -> Transition {
                Transition {
                    days: self.days,
                    storage_class: self.storage_class.clone(),
                    is_access_time: self.is_access_time,
                    return_to_std_when_visit: self.return_to_std_when_visit,
                    allow_small_file: self.allow_small_file,
                }
            }
        }

        #[derive(Default)]
        #[allow(unused)]
        pub struct RuleBuilder<'a> {
            id: &'a str,
            prefix: &'a str,
            status: &'a str,
            transition: Option<Vec<Transition>>,
            filter: Option<Filter>,
            expiration: Option<Expiration>,
            noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
            abort_multipart_upload: Option<AbortMultipartUpload>,
        }

        #[allow(unused)]
        impl<'a> RuleBuilder<'a> {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn id(mut self, value: &'a str) -> Self {
                self.id = value;
                self
            }

            pub fn prefix(mut self, value: &'a str) -> Self {
                self.prefix = value;
                self
            }

            pub fn status(mut self, value: &'a str) -> Self {
                self.status = value;
                self
            }

            pub fn add_transition(mut self, value: Transition) -> Self {
                let transitions = if let Some(mut transitions) = self.transition {
                    transitions.push(value);
                    transitions
                } else {
                    vec![value]
                };
                self.transition = Some(transitions);
                self
            }

            pub fn filter(mut self, value: Filter) -> Self {
                self.filter = Some(value);
                self
            }

            pub fn expiration(mut self, value: Expiration) -> Self {
                self.expiration = Some(value);
                self
            }

            pub fn noncurrent_version_expiration(
                mut self,
                value: NoncurrentVersionExpiration,
            ) -> Self {
                self.noncurrent_version_expiration = Some(value);
                self
            }

            pub fn abort_multipart_upload(mut self, value: i32) -> Self {
                self.abort_multipart_upload = Some(AbortMultipartUpload { days: value });
                self
            }

            pub fn builder(&self) -> Rule {
                Rule {
                    id: self.id.to_string(),
                    prefix: self.prefix.to_string(),
                    status: self.status.to_string(),
                    transition: self.transition.clone(),
                    filter: self.filter.clone(),
                    expiration: self.expiration.clone(),
                    noncurrent_version_expiration: self.noncurrent_version_expiration.clone(),
                    abort_multipart_upload: self.abort_multipart_upload.clone(),
                }
            }
        }

        #[derive(Default)]
        #[allow(unused)]
        pub struct LifecycleConfigurationBuilder {
            rules: Vec<Rule>,
        }

        #[allow(unused)]
        impl LifecycleConfigurationBuilder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn add_rule(mut self, value: Rule) -> Self {
                self.rules.push(value);
                self
            }

            pub fn builder(&self) -> LifecycleConfiguration {
                LifecycleConfiguration {
                    rule: self.rules.clone(),
                }
            }
        }
    }
}

pub mod builder {
    use crate::oss;

    use super::{CORSConfiguration, CORSRule};

    #[derive(Default, Debug)]
    #[allow(unused)]
    pub struct CORSRuleBuilder {
        pub rule: CORSRule,
    }

    impl<'a> CORSRuleBuilder {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn allowed_origin(mut self, value: &'a str) -> Self {
            self.rule.allowed_origin.push(value.to_string());
            self
        }

        pub fn allowed_method(mut self, value: oss::Method) -> Self {
            self.rule.allowed_method.push(value.to_string());
            self
        }

        pub fn allowed_header(mut self, value: oss::header::HeaderName) -> Self {
            if let Some(mut header_list) = self.rule.allowed_header {
                header_list.push(value.to_string());
                self.rule.allowed_header = Some(header_list)
            } else {
                let header_list = vec![value.to_string()];
                self.rule.allowed_header = Some(header_list);
            }
            self
        }

        pub fn expose_header(mut self, value: &'a str) -> Self {
            if let Some(mut expose_header_list) = self.rule.expose_header {
                expose_header_list.push(value.to_string());
                self.rule.expose_header = Some(expose_header_list)
            } else {
                let expose_header_list = vec![value.to_string()];
                self.rule.expose_header = Some(expose_header_list);
            }
            self
        }

        pub fn max_age_seconds(mut self, value: i32) -> Self {
            self.rule.max_age_seconds = Some(value);
            self
        }

        pub fn builder(self) -> CORSRule {
            self.rule
        }
    }

    pub struct CORSConfigurationBuilder {
        pub cors_configuration: CORSConfiguration,
    }

    impl CORSConfigurationBuilder {
        pub fn new() -> Self {
            Self {
                cors_configuration: CORSConfiguration::default(),
            }
        }

        pub fn add_rule(mut self, value: CORSRule) -> Self {
            self.cors_configuration.cors_rule.push(value);
            self
        }

        pub fn response_vary(mut self, value: bool) -> Self {
            self.cors_configuration.response_vary = Some(value);
            self
        }

        pub fn builder(self) -> CORSConfiguration {
            self.cors_configuration
        }
    }

    // ------------------------------------------------
}

#[cfg(test)]
mod tests {

    use crate::oss::{
        self,
        entities::{
            inner,
            lifecycle::{
                builder::{
                    ExpirationBuilder, LifecycleConfigurationBuilder, RuleBuilder,
                    TransitionBuilder,
                },
                Expiration, LifecycleConfiguration, Rule, Transition,
            },
            ListVersionsResult, StorageClass, Style, Tag, TagSet, Tagging,
            TransferAccelerationConfiguration,
        },
    };

    use super::{
        builder::{CORSConfigurationBuilder, CORSRuleBuilder},
        ApplyServerSideEncryptionByDefault, CORSConfiguration, ServerSideEncryptionRule,
    };

    #[test]
    fn tagging() {
        let tag = Tag {
            key: "key1".to_string(),
            value: "value1".to_string(),
        };
        let tag1 = Tag {
            key: "key1".to_string(),
            value: "value1".to_string(),
        };

        let tag_set = TagSet {
            tag: Some(vec![tag, tag1]),
        };

        let tag_sets = Tagging { tag_set };
        let content = quick_xml::se::to_string(&tag_sets).unwrap();
        println!("{}", content);

        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Tagging>
	<TagSet>
		<Tag>
			<Key>key1</Key>
			<Value>value1</Value>
		</Tag>
		<Tag>
			<Key>key2</Key>
			<Value>value2</Value>
		</Tag>
	</TagSet>
</Tagging>"#;

        let c: Tagging = quick_xml::de::from_str(&xml).unwrap();
        println!("{:#?}", c);
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

    // xml转换
    #[test]
    fn lifecycle_configuration_1() {
        let xml_content = r#"<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log/</Prefix><Status>Enabled</Status><Transition><Days>30</Days><StorageClass>IA</StorageClass></Transition></Rule></LifecycleConfiguration>"#;
        let rule = Rule {
            id: "rule".to_string(),
            prefix: "log/".to_string(),
            status: "Enabled".to_string(),
            transition: Some(vec![Transition {
                days: Some(30),
                storage_class: StorageClass::IA,
                is_access_time: None,
                return_to_std_when_visit: None,
                allow_small_file: None,
            }]),
            expiration: None,
            filter: None,
            noncurrent_version_expiration: None,
            abort_multipart_upload: None,
        };

        let config = LifecycleConfiguration { rule: vec![rule] };
        let content = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(content, xml_content);
    }

    // xml转换
    #[test]
    fn lifecycle_configuration_2() {
        let xml_content = r#"<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log</Prefix><Status>Enabled</Status><Expiration><Days>90</Days></Expiration></Rule></LifecycleConfiguration>"#;

        let rule = Rule {
            id: "rule".to_string(),
            prefix: "log".to_string(),
            status: "Enabled".to_string(),
            transition: None,
            expiration: Some(Expiration {
                days: Some(90),
                created_before_date: None,
                expired_object_delete_marker: None,
            }),
            filter: None,
            noncurrent_version_expiration: None,
            abort_multipart_upload: None,
        };

        let config = LifecycleConfiguration { rule: vec![rule] };
        let content = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(content, xml_content);
    }

    // xml转换
    #[test]
    fn lifecycle_configuration_3() {
        let xml_content = r#"<LifecycleConfiguration><Rule><ID>rule</ID><Prefix>log/</Prefix><Status>Enabled</Status><Transition><Days>30</Days><StorageClass>IA</StorageClass></Transition><Transition><Days>60</Days><StorageClass>Archive</StorageClass></Transition><Expiration><Days>3600</Days></Expiration></Rule></LifecycleConfiguration>"#;

        let transition = vec![
            Transition {
                days: Some(30),
                storage_class: StorageClass::IA,
                is_access_time: None,
                return_to_std_when_visit: None,
                allow_small_file: None,
            },
            Transition {
                days: Some(60),
                storage_class: StorageClass::Archive,
                is_access_time: None,
                return_to_std_when_visit: None,
                allow_small_file: None,
            },
        ];

        let rule = Rule {
            id: "rule".to_string(),
            prefix: "log/".to_string(),
            status: "Enabled".to_string(),
            transition: Some(transition),
            expiration: Some(Expiration {
                days: Some(3600),
                created_before_date: None,
                expired_object_delete_marker: None,
            }),
            filter: None,
            noncurrent_version_expiration: None,
            abort_multipart_upload: None,
        };

        let config = LifecycleConfiguration { rule: vec![rule] };
        let content = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(content, xml_content);
    }

    #[test]
    fn lifecycle_configuration_builder() {
        let rule1 = RuleBuilder::new()
            .id("RuleID")
            .prefix("Prefix")
            .status("status")
            .expiration(
                ExpirationBuilder::new()
                    .days(23)
                    .expired_object_delete_marker(false)
                    .builder(),
            )
            .add_transition(
                TransitionBuilder::new()
                    .days(23)
                    .standard_storage(StorageClass::Archive)
                    .builder(),
            )
            .abort_multipart_upload(12)
            .builder();

        let config = LifecycleConfigurationBuilder::new()
            .add_rule(rule1)
            .builder();

        println!("{:#?}", config);

        println!("{}", quick_xml::se::to_string(&config).unwrap());
    }

    #[test]
    fn style() {
        let xml_origin = r#"<Style><Name>imagestyle</Name><Content>image/resize,p_50</Content><Category>image</Category><CreateTime>Wed, 20 May 2020 12:07:15 GMT</CreateTime><LastModifyTime>Wed, 20 May 2020 12:07:15 GMT</LastModifyTime></Style>"#;

        let style = Style {
            name: "imagestyle".to_string(),
            content: "image/resize,p_50".to_string(),
            category: Some("image".to_string()),
            create_time: None,
            last_modify_time: None,
        };

        let xml_gen = quick_xml::se::to_string(&style).unwrap();
        println!("{}", xml_gen);

        let style1 = quick_xml::de::from_str::<Style>(&xml_origin).unwrap();
        println!("{:#?}", style1);
    }

    #[test]
    fn server_side_encryption_rule1() {
        let xml_conrtent = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault> <SSEAlgorithm>KMS</SSEAlgorithm><KMSDataEncryption>SM4</KMSDataEncryption> <KMSMasterKeyID></KMSMasterKeyID></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;

        let object: ServerSideEncryptionRule = quick_xml::de::from_str(xml_conrtent).unwrap();
        assert_eq!(
            object
                .apply_server_side_encryption_by_default
                .kms_data_encryption,
            Some("SM4".to_string())
        )
    }

    #[test]
    fn server_side_encryption_rule2() {
        let object = ServerSideEncryptionRule {
            apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                sse_algorithm: super::SSEAlgorithm::KMS,
                kms_data_encryption: Some("9468da86-3509-4f8d-a61e-6eab1eac****".to_string()),
                kms_master_key_id: None,
            },
        };
        let left = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>KMS</SSEAlgorithm><KMSDataEncryption>9468da86-3509-4f8d-a61e-6eab1eac****</KMSDataEncryption><KMSMasterKeyID/></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;

        let right = quick_xml::se::to_string(&object).unwrap();
        assert_eq!(left, right)
    }

    #[test]
    fn server_side_encryption_rule3() {
        let object = ServerSideEncryptionRule {
            apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                sse_algorithm: super::SSEAlgorithm::SM4,
                kms_data_encryption: None,
                kms_master_key_id: None,
            },
        };
        let left = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>SM4</SSEAlgorithm><KMSMasterKeyID/></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;

        let right = quick_xml::se::to_string(&object).unwrap();
        assert_eq!(left, right)
    }

    #[test]
    fn server_side_encryption_rule4() {
        let object = ServerSideEncryptionRule {
            apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                sse_algorithm: super::SSEAlgorithm::KMS,
                kms_data_encryption: Some("SM4".to_string()),
                kms_master_key_id: None,
            },
        };
        let left = r#"<ServerSideEncryptionRule><ApplyServerSideEncryptionByDefault><SSEAlgorithm>KMS</SSEAlgorithm><KMSDataEncryption>SM4</KMSDataEncryption><KMSMasterKeyID/></ApplyServerSideEncryptionByDefault></ServerSideEncryptionRule>"#;
        let right = quick_xml::se::to_string(&object).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn cors_configuration1() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<CORSConfiguration>
  <CORSRule>
    <AllowedOrigin>*</AllowedOrigin>
    <AllowedMethod>PUT</AllowedMethod>
    <AllowedMethod>GET</AllowedMethod>
    <AllowedHeader>Authorization</AllowedHeader>
  </CORSRule>
  <CORSRule>
    <AllowedOrigin>http://example.com</AllowedOrigin>
    <AllowedOrigin>http://example.net</AllowedOrigin>
    <AllowedMethod>GET</AllowedMethod>
    <AllowedHeader> Authorization</AllowedHeader>
    <ExposeHeader>x-oss-test</ExposeHeader>
    <ExposeHeader>x-oss-test1</ExposeHeader>
    <MaxAgeSeconds>100</MaxAgeSeconds>
  </CORSRule>
  <ResponseVary>false</ResponseVary>
</CORSConfiguration>"#;

        let object = quick_xml::de::from_str::<CORSConfiguration>(&xml_content).unwrap();
        assert_eq!(object.cors_rule[0].allowed_origin[0], "*");
    }

    #[test]
    fn cors_configuration2() {
        let rule1 = CORSRuleBuilder::new()
            .allowed_origin("*")
            .allowed_method(oss::Method::PUT)
            .allowed_method(oss::Method::GET)
            .allowed_header(oss::header::AUTHORIZATION)
            .builder();

        let rule2 = CORSRuleBuilder::new()
            .allowed_origin("http://example.com")
            .allowed_origin("http://example.net")
            .allowed_method(oss::Method::GET)
            .allowed_header(oss::header::AUTHORIZATION)
            .expose_header("x-oss-test")
            .expose_header("x-oss-test1")
            .builder();

        let config = CORSConfigurationBuilder::new()
            .add_rule(rule1)
            .add_rule(rule2)
            .response_vary(false)
            .builder();

        let left = format!("{}", quick_xml::se::to_string(&config).unwrap());

        let right = r#"<CORSConfiguration><CORSRule><AllowedOrigin>*</AllowedOrigin><AllowedMethod>PUT</AllowedMethod><AllowedMethod>GET</AllowedMethod><AllowedHeader>authorization</AllowedHeader><ExposeHeader/><MaxAgeSeconds/></CORSRule><CORSRule><AllowedOrigin>http://example.com</AllowedOrigin><AllowedOrigin>http://example.net</AllowedOrigin><AllowedMethod>GET</AllowedMethod><AllowedHeader>authorization</AllowedHeader><ExposeHeader>x-oss-test</ExposeHeader><ExposeHeader>x-oss-test1</ExposeHeader><MaxAgeSeconds/></CORSRule><ResponseVary>false</ResponseVary></CORSConfiguration>"#;

        assert_eq!(left, right)
        // println!("{}", quick_xml::se::to_string(&config).unwrap())
    }

    #[test]
    fn list_versions_result_1() {
        let xml_content = r#"<ListVersionsResult>
<Name>examplebucket-1250000000</Name>
<Prefix/>
<KeyMarker/>
<VersionIdMarker/>
<MaxKeys>1000</MaxKeys>
<IsTruncated>false</IsTruncated>
<Version>
    <Key>example-object-1.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-5T12:03:10.000Z</LastModified>
    <ETag>5B3C1A2E053D763E1B669CC607C5A0FE1****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner>
        <ID>1250000000</ID>
        <DisplayName>1250000000</DisplayName>
    </Owner>
</Version>
<Version>
    <Key>example-object-2.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-9T12:03:09.000Z</LastModified>
    <ETag>5B3C1A2E053D763E1B002CC607C5A0FE1****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner>
        <ID>1250000000</ID>
        <DisplayName>1250000000</DisplayName>
    </Owner>
</Version>
<Version>
    <Key>example-object-3.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-10T12:03:08.000Z</LastModified>
    <ETag>4B3F1A2E053D763E1B002CC607C5AGTRF****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner>
        <ID>1250000000</ID>
        <DisplayName>1250000000</DisplayName>
    </Owner>
</Version>
</ListVersionsResult>"#;

        let object = quick_xml::de::from_str::<ListVersionsResult>(&xml_content).unwrap();
        let left = "example-object-1.jpg";
        let right = object.version[0].key.to_string();
        assert_eq!(left, right);
    }

    #[test]
    fn list_versions_result_2() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListVersionsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Name>oss-example</Name>
    <Prefix></Prefix>
    <KeyMarker>example</KeyMarker>
    <VersionIdMarker>CAEQMxiBgICbof2D0BYiIGRhZjgwMzJiMjA3MjQ0ODE5MWYxZDYwMzJlZjU1****</VersionIdMarker>
    <MaxKeys>100</MaxKeys>
    <Delimiter></Delimiter>
    <IsTruncated>false</IsTruncated>
    <DeleteMarker>
        <Key>example</Key>
        <VersionId>CAEQMxiBgICAof2D0BYiIDJhMGE3N2M1YTI1NDQzOGY5NTkyNTI3MGYyMzJm****</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2019-04-09T07:27:28.000Z</LastModified>
        <Owner>
            <ID>1234512528586****</ID>
            <DisplayName>12345125285864390</DisplayName>
        </Owner>
    </DeleteMarker>
    <Version>
        <Key>example</Key>
        <VersionId>CAEQMxiBgMDNoP2D0BYiIDE3MWUxNzgxZDQxNTRiODI5OGYwZGMwNGY3MzZjN****</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2019-04-09T07:27:28.000Z</LastModified>
        <ETag>"250F8A0AE989679A22926A875F0A2****"</ETag>
        <Type>Normal</Type>
        <Size>93731</Size>
        <StorageClass>Standard</StorageClass>
        <Owner>
            <ID>1234512528586****</ID>
            <DisplayName>12345125285864390</DisplayName>
        </Owner>
    </Version>
    <Version>
        <Key>pic.jpg</Key>
        <VersionId>CAEQMxiBgMCZov2D0BYiIDY4MDllOTc2YmY5MjQxMzdiOGI3OTlhNTU0ODIx****</VersionId>
        <IsLatest>true</IsLatest>
        <LastModified>2019-04-09T07:27:28.000Z</LastModified>
        <ETag>"3663F7B0B9D3153F884C821E7CF4****"</ETag>
        <Type>Normal</Type>
        <Size>574768</Size>
        <StorageClass>Standard</StorageClass>
        <Owner>
            <ID>1234512528586****</ID>
            <DisplayName>12345125285864390</DisplayName>
        </Owner>
    </Version>
</ListVersionsResult>"#;

        let object: ListVersionsResult = quick_xml::de::from_str(&xml_content).unwrap();

        let left = "example";
        let right = object.delete_marker.unwrap()[0].key.to_string();

        assert_eq!(left, right);
    }

    #[test]
    fn list_versions_result_3() {
        let xml_content = r#"<ListVersionsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
<Name>oss-example</Name>
<Prefix></Prefix>
<KeyMarker>example</KeyMarker>
<VersionIdMarker>CAEQMxiBgICbof2D0BYiIGRhZjgwMzJiMjA3MjQ0ODE5MWYxZDYwMzJlZjU1****</VersionIdMarker>
<MaxKeys>100</MaxKeys>
<Delimiter></Delimiter>
<IsTruncated>false</IsTruncated>
<Version>
    <Key>exampleobject1.txt</Key>
    <VersionId>CAEQMxiBgICAof2D0BYiIDJhMGE3N2M1YTI1NDQzOGY5NTkyNTI3MGYyMzJm****</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <Owner>
        <ID>1234512528586****</ID>
        <DisplayName>12345125285864390</DisplayName>
    </Owner>
    </Version>
<Version>
    <Key>exampleobject2.txt</Key>
    <VersionId>CAEQMxiBgMDNoP2D0BYiIDE3MWUxNzgxZDQxNTRiODI5OGYwZGMwNGY3MzZjN****</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <ETag>"250F8A0AE989679A22926A875F0A2****"</ETag>
    <Type>Normal</Type>
    <Size>93731</Size>
    <StorageClass>Standard</StorageClass>
    <RestoreInfo>ongoing-request="true"</RestoreInfo>
    <Owner>
        <ID>1234512528586****</ID>
        <DisplayName>12345125285864390</DisplayName>
    </Owner>
    </Version>
<Version>
    <Key>exampleobject3.txt</Key>
    <VersionId>CAEQMxiBgMCZov2D0BYiIDY4MDllOTc2YmY5MjQxMzdiOGI3OTlhNTU0ODIx****</VersionId>
    <IsLatest>true</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <ETag>"3663F7B0B9D3153F884C821E7CF4****"</ETag>
    <Type>Normal</Type>
    <Size>574768</Size>
    <StorageClass>Standard</StorageClass>
    <RestoreInfo>ongoing-request="false", expiry-date="Thr, 24 Mon 2020 12:40:33 GMT"</RestoreInfo>
    <Owner>
        <ID>1234512528586****</ID>
        <DisplayName>12345125285864390</DisplayName>
    </Owner>
    </Version>
</ListVersionsResult>"#;

        let object: ListVersionsResult = quick_xml::de::from_str(&xml_content).unwrap();

        println!("{:#?}", object);
    }
}
