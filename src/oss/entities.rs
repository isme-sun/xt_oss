use std::fmt::Display;

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
                Self::BR => "br"
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagSet {
    #[serde(rename = "Tag")]
    pub(crate) tag: Vec<Tag>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Not {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Tag")]
    pub tag: Tag,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    #[serde(rename = "Not")]
    pub not: Not,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbortMultipartUpload {
    #[serde(rename = "Days")]
    pub days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "NoncurrentDays", skip_serializing_if = "Option::is_none")]
    pub noncurrent_days: Option<bool>,
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Transition {
    #[serde(rename = "Days")]
    pub days: i32,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Expiration {
    #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(
        rename = "ExpiredObjectDeleteMarker",
        skip_serializing_if = "Option::is_none"
    )]
    pub expired_object_delete_marker: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoncurrentVersionExpiration {
    #[serde(rename = "NoncurrentDays")]
    pub noncurrent_days: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rule")]
    pub rule: Vec<Rule>,
}
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

#[cfg(test)]
mod tests {

    use crate::oss::entities::{
        inner, Expiration, LifecycleConfiguration, StorageClass, Style, Tag, TagSet, Tagging,
        TransferAccelerationConfiguration, Transition,
    };

    use super::Rule;

    #[test]
    fn temp() {}

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
            tag: vec![tag, tag1],
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
                days: 30,
                storage_class: StorageClass::IA,
                is_access_time: None,
                return_to_std_when_visit: None,
                allow_small_file: None,
            }]),
            expiration: None,
            filter: None,
            noncurrent_version_expiration: None,
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
                expired_object_delete_marker: None,
            }),
            filter: None,
            noncurrent_version_expiration: None,
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
                days: 30,
                storage_class: StorageClass::IA,
                is_access_time: None,
                return_to_std_when_visit: None,
                allow_small_file: None,
            },
            Transition {
                days: 60,
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
                expired_object_delete_marker: None,
            }),
            filter: None,
            noncurrent_version_expiration: None,
        };

        let config = LifecycleConfiguration { rule: vec![rule] };
        let content = quick_xml::se::to_string(&config).unwrap();
        assert_eq!(content, xml_content);
    }

    // xml转换
    // #[test]
    // fn lifecycle_configuration_4() {
    //     let xml_content = r#"<LifecycleConfiguration><Rule><ID>rule</ID><Prefix></Prefix><Status>Enabled</Status><Expiration><ExpiredObjectDeleteMarker>true</ExpiredObjectDeleteMarker></Expiration><NoncurrentVersionExpiration><NoncurrentDays>5</NoncurrentDays></NoncurrentVersionExpiration></Rule></LifecycleConfiguration>"#;

    //     let rule = Rule {
    //         id: "rule".to_string(),
    //         prefix: "".to_string(),
    //         status: "Enabled".to_string(),
    //         transition: None,
    //         expiration: Some(Expiration {
    //             days: None,
    //             expired_object_delete_marker: Some(true),
    //         }),
    //         filter: None,
    //         noncurrent_version_expiration: Some(NoncurrentVersionExpiration { noncurrent_days: 5 }),
    //     };

    //     let config = LifecycleConfiguration { rule: vec![rule] };
    //     let content = quick_xml::se::to_string(&config).unwrap();
    //     assert_eq!(content, xml_content);
    // }

    // #[test]
    // fn lifecycle_configuration_5() {
    //     let xml_origin = r#"<LifecycleConfiguration><Rule><ID>rule</ID><Prefix></Prefix><Status>Enabled</Status><Filter> <Not><Prefix>log</Prefix><Tag><Key>key1</Key><Value>value1</Value></Tag></Not></Filter><Transition> <Days>30</Days><StorageClass>Archive</StorageClass></Transition><Expiration><Days>100</Days></Expiration></Rule></LifecycleConfiguration>"#;

    //     let rule = Rule {
    //         id: String::from("rule"),
    //         prefix: String::from(""),
    //         status: String::from("Enabled"),
    //         filter: Some(Filter {
    //             not: Not {
    //                 prefix: String::from("log"),
    //                 tag: Tag {
    //                     key: String::from("key1"),
    //                     value: String::from("value1"),
    //                 },
    //             },
    //         }),
    //         transition: Some(vec![Transition {
    //             days: 30,
    //             storage_class: StorageClass::Archive,
    //             is_access_time: None,
    //             return_to_std_when_visit: None,
    //             allow_small_file: None,
    //         }]),
    //         expiration: Some(Expiration {
    //             days: Some(100),
    //             expired_object_delete_marker: None,
    //         }),
    //         noncurrent_version_expiration: None,
    //     };

    //     let config = LifecycleConfiguration { rule: vec![rule] };

    //     let xml_gen = format!("{}", quick_xml::se::to_string(&config).unwrap());

    //     assert_eq!(xml_gen, xml_origin);
    // }

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
}
