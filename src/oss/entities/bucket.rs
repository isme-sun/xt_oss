use super::{acl::AccessControlList, object::Object, oss, DataRedundancyType, StorageClass};
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
pub struct LocationConstraint(pub String);

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
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "MaxKeys")]
    pub max_keys: i32,
    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "KeyCount")]
    pub key_count: Option<u32>,
    #[serde(rename = "NextContinuationToken")]
    pub next_continuation_token: Option<String>,
    #[serde(rename = "Contents")]
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
        let left = "2014-02-17T18:12:43.000Z";
        let right = &obj.buckets.bucket.unwrap()[0].creation_date;
        assert_eq!(left, right);
    }

    #[test]
    pub fn list_object_v2() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <ListBucketResult>
      <Name>xtoss-ex11</Name>
      <Prefix></Prefix>
      <MaxKeys>5</MaxKeys>
      <Delimiter></Delimiter>
      <IsTruncated>true</IsTruncated>
      <NextContinuationToken>ChlpbWFnZXMvSlBHSW1hZ2VfMm1ibWIuanBnEAA-</NextContinuationToken>
      <Contents>
        <Key>excel/Spreadsheet-1000-rows.xls</Key>
        <LastModified>2024-02-09T12:11:40.000Z</LastModified>
        <ETag>"B6DF06A19E3A3AF4F39EBD2E14C64F28"</ETag>
        <Type>Normal</Type>
        <Size>217088</Size>
        <StorageClass>Standard</StorageClass>
      </Contents>
      <Contents>
        <Key>excel/Spreadsheet-5000-rows.xls</Key>
        <LastModified>2024-02-09T12:11:39.000Z</LastModified>
        <ETag>"F97C47A00070BC0B945268A26FC8C14A"</ETag>
        <Type>Normal</Type>
        <Size>925696</Size>
        <StorageClass>Standard</StorageClass>
      </Contents>
      <Contents>
        <Key>images/JPGImage_100kbmb.jpg</Key>
        <LastModified>2024-02-09T12:11:39.000Z</LastModified>
        <ETag>"12EA14D362611F6CCAB9C66CA0A3FAEF"</ETag>
        <Type>Normal</Type>
        <Size>102796</Size>
        <StorageClass>Standard</StorageClass>
      </Contents>
      <Contents>
        <Key>images/JPGImage_15mbmb.jpg</Key>
        <LastModified>2024-02-09T12:11:31.000Z</LastModified>
        <ETag>"AD9D4461988B2D82D53F0DA31CAFEAA5"</ETag>
        <Type>Normal</Type>
        <Size>15882755</Size>
        <StorageClass>Standard</StorageClass>
      </Contents>
      <Contents>
        <Key>images/JPGImage_2mbmb.jpg</Key>
        <LastModified>2024-02-09T12:11:39.000Z</LastModified>
        <ETag>"EA5EFC10C2873F1713FDB368E4D25DD7"</ETag>
        <Type>Normal</Type>
        <Size>2101546</Size>
        <StorageClass>Standard</StorageClass>
      </Contents>
      <KeyCount>5</KeyCount>
    </ListBucketResult>"#;

        let entity: ListBucketResult2 = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "excel/Spreadsheet-1000-rows.xls";
        let right = &entity.contents.unwrap()[0].key;
        assert_eq!(&left, &right);
    }
}
