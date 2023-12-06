use std::fmt::{self, Display};

// use reqwest::header::HeaderMap;
use crate::oss;
use serde::{Deserialize, Serialize};

/// 指定存储空间的存储类型
///
/// - `Standard` 标准存储
/// - `IA` 低频访问
/// - `Archive` 归档存储
/// - `DeepColdArchive` 深度冷归档存储
///
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "StorageClass")]
pub enum StorageClass {
    #[default]
    Standard,
    IA,
    Archive,
    ColdArchive,
    DeepColdArchive,
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
    // #[serde(rename = "public-read-write")]
    PublicReadWrite,
    #[default]
    // #[serde(rename = "public-read")]
    PublicRead,
    // #[serde(rename = "private")]
    Private,
}

impl fmt::Display for OssAcl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::PublicRead => "public-read",
            Self::PublicReadWrite => "public-read-write",
            Self::Private => "private",
        };
        write!(f, "{}", desc)
    }
}

pub trait OSSQuery {
    fn to_query(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObject2Query<'a> {
    #[serde(rename = "list-type")]
    pub list_type: i32,
    pub delimiter: Option<&'a str>,
    #[serde(rename = "start-after")]
    pub start_after: Option<&'a str>,
    #[serde(rename = "continuation-token")]
    pub continuation_token: Option<&'a str>,
    #[serde(rename = "max-keys")]
    pub max_keys: Option<i32>,
    pub prefix: Option<String>,
    #[serde(rename = "encoding-type")]
    pub encoding_type: Option<&'a str>,
    #[serde(rename = "fetch-owner")]
    pub fetch_owner: Option<bool>,
}

impl<'a> Default for ListObject2Query<'a> {
    fn default() -> Self {
        ListObject2Query {
            list_type: 2,
            delimiter: None,
            start_after: None,
            continuation_token: None,
            max_keys: Some(100),
            prefix: None,
            encoding_type: Some("url"),
            fetch_owner: None,
        }
    }
}

impl<'a> fmt::Display for ListObject2Query<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_qs::to_string(self).unwrap())
    }
}

#[derive(Debug, Default)]
pub struct DescribeRegionsQuery {
    pub regions: Option<String>,
}

impl Display for DescribeRegionsQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(region) = &self.regions {
            write!(f, "regions={}", region)
        } else {
            write!(f, "regions")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListBucketsQuery {
    /// 限定此次返回Bucket的最大个数。
    pub prefix: Option<String>,
    /// 设定结果从marker之后按字母排序的第一个开始返回。如果不设定，则从头开始返回数据。
    pub marker: Option<String>,
    #[serde(rename = "max-keys")]
    /// 限定返回的Bucket名称必须以prefix作为前缀。如果不设定，则不过滤前缀信息。
    pub max_keys: Option<i32>,
}

impl OSSQuery for ListBucketsQuery {
    fn to_query(&self) -> String {
        serde_qs::to_string(&self).unwrap()
    }
}

#[derive(Debug, Default)]
pub struct CreateBucketParams<'a> {
    pub acl: Option<OssAcl>,
    pub group_id: Option<&'a str>,
    pub config: Option<CreateBucketConfiguration<'a>>,
}

impl<'a> CreateBucketParams<'a> {

    pub fn headers(&self) -> oss::HeaderMap {
        let mut headers = oss::HeaderMap::default();
        if let Some(acl) = &self.acl {
            headers.insert("x-oss-acl", acl.to_string().parse().unwrap());
        }
        if let Some(group_id) = &self.group_id {
            headers.insert("x-oss-resource-group-id", group_id.parse().unwrap());
        }
        headers
    }

    pub fn config(&self) -> oss::Bytes {

        oss::Bytes::from("ok")
    }

}

#[derive(Debug, Default)]
pub struct CreateBucketConfiguration<'a> {
    pub acl: Option<OssAcl>,
    pub group_id: Option<&'a str>,
    pub storage_class: Option<StorageClass>,
    pub data_redundancy_type: Option<DataRedundancyType>,
}

impl<'a> CreateBucketConfiguration<'a> {

    pub fn headers(&self) -> oss::HeaderMap {
        let mut headers = oss::HeaderMap::default();
        if let Some(acl) = &self.acl {
            headers.insert("x-oss-acl", acl.to_string().parse().unwrap());
        }
        if let Some(group_id) = &self.group_id {
            headers.insert("x-oss-resource-group-id", group_id.parse().unwrap());
        }
        headers
    }

    pub fn config(&self) -> oss::Bytes {

        oss::Bytes::from("ok")
    }

}
