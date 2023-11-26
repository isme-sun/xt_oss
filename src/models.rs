use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_xml_rs;

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListCnameResult {
    #[serde(rename(deserialize = "Bucket"))]
    pub bucket: String,
    #[serde(rename(deserialize = "Owner"))]
    pub owner: String,
    #[serde(rename(deserialize = "Cname"))]
    pub cname: Cname,
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
    pub access_monitor: Option<String>,
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
    pub transfer_acceleration: Option<String>,
    #[serde(rename(deserialize = "CrossRegionReplication"))]
    pub cross_region_replication: Option<String>,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "ResourceGroupId"))]
    pub resource_group_id: Option<String>,
    #[serde(rename(deserialize = "Owner"))]
    pub owner: Option<Owner>,
    #[serde(rename(deserialize = "AccessControlList"))]
    pub access_control_list: Option<AccessControlList>,
    #[serde(rename(deserialize = "Comment"))]
    pub comment: String,
    #[serde(rename(deserialize = "BucketPolicy"))]
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
    #[serde(rename(serialize = "AccelerateEndpoint", deserialize = "AccelerateEndpoint"))]
    pub accelerate_endpoint: String,
    #[serde(rename(serialize = "InternalEndpoint", deserialize = "InternalEndpoint"))]
    pub internal_endpoint: String,
    #[serde(rename(serialize = "InternetEndpoint", deserialize = "InternetEndpoint"))]
    pub internet_endpoint: String,
    #[serde(rename(serialize = "Region", deserialize = "Region"))]
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegionInfoList {
    #[serde(rename(serialize = "RegionInfo", deserialize = "RegionInfo"))]
    pub region_info: Vec<RegionInfo>,
}