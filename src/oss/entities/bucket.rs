use super::{acl::AccessControlList, object::Object, oss, DataRedundancyType, StorageClass};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default, Clone)]
pub struct CreateBucketConfiguration {
  #[serde(rename = "StorageClass", skip_serializing_if = "Option::is_none")]
  pub storage_class: Option<StorageClass>,
  #[serde(
    rename = "data_redundancy_type",
    skip_serializing_if = "Option::is_none"
  )]
  pub data_redundancy_type: Option<DataRedundancyType>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Owner {
  #[serde(rename = "ID")]
  pub id: String,
  #[serde(rename = "DisplayName")]
  pub display_name: String,
}

/// Bucket所在的地域
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "$value")]
pub struct LocationConstraint(String);

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BucketPolicy {
  #[serde(rename(deserialize = "LogBucket"))]
  pub log_bucket: String,
  #[serde(rename(deserialize = "LogPrefix"))]
  pub log_prefix: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Bucket {
  #[serde(rename = "AccessMonitor")]
  pub access_monitor: Option<String>,
  #[serde(rename = "CreationDate", with = "super::private::serde_date::utc")]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Buckets {
  #[serde(rename(deserialize = "Bucket"))]
  pub bucket: Option<Vec<Bucket>>,
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
  pub storage: u64,
  /// Bucket中总的Object数量
  #[serde(rename(deserialize = "ObjectCount"))]
  pub object_count: u64,
  #[serde(rename(deserialize = "MultipartUploadCount"))]
  pub multipart_upload_count: u64,
  #[serde(rename(deserialize = "LiveChannelCount"))]
  pub live_channel_count: u64,
  #[serde(rename(deserialize = "LastModifiedTime"))]
  pub last_modified_time: u64,
  #[serde(rename(deserialize = "StandardStorage"))]
  pub standard_storage: u64,
  #[serde(rename(deserialize = "StandardObjectCount"))]
  pub standard_object_count: u64,
  #[serde(rename(deserialize = "InfrequentAccessStorage"))]
  pub infrequent_access_storage: u64,
  #[serde(rename(deserialize = "InfrequentAccessRealStorage"))]
  pub infrequent_access_real_storage: u64,
  #[serde(rename(deserialize = "InfrequentAccessObjectCount"))]
  pub infrequent_access_object_count: u64,
  #[serde(rename(deserialize = "ArchiveStorage"))]
  pub archive_storage: u64,
  #[serde(rename(deserialize = "ArchiveRealStorage"))]
  pub archive_real_storage: u64,
  #[serde(rename(deserialize = "ArchiveObjectCount"))]
  pub archive_object_count: u64,
  #[serde(rename(deserialize = "ColdArchiveStorage"))]
  pub cold_archive_storage: u64,
  #[serde(rename(deserialize = "ColdArchiveRealStorage"))]
  pub cold_archive_real_storage: u64,
  #[serde(rename(deserialize = "ColdArchiveObjectCount"))]
  pub cold_archive_object_count: u64,
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

#[cfg(test)]
pub mod test {

  use super::*;

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
    let obj: ListAllMyBucketsResult = quick_xml::de::from_str(xml).unwrap();
    println!("{:#?}", &obj);
  }
}
