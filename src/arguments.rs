use serde::{Deserialize, Serialize};


/// 指定存储空间的存储类型
/// 
/// - `Standard` 标准存储
/// - `IA` 低频访问
/// - `Archive` 归档存储
/// - `DeepColdArchive` 深度冷归档存储
/// 
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag ="StorageClass")]
pub enum StorageClass {
    Standard, 
    IA,
    Archive,
    ColdArchive,
    DeepColdArchive
}

/// 指定存储空间的数据容灾类型
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "DataRedundancyType")]
pub enum DataRedundancyType {
    /// 本地冗余LRS将您的数据冗余存储在同一个可用区的不同存储设备上，可支持两个存储设备并发损坏时，仍维持数据不丢失，可正常访问
    LRS,
    /// 同城冗余ZRS采用多可用区（AZ）内的数据冗余存储机制，将用户的数据冗余存储在同一地域（Region）的多个可用区。当某个可用区不可用时，仍然能够保障数据的正常访问
    ZRS,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OssAcl {
    #[serde(rename = "public-read-write")]
    PublicReadWrite,
    #[serde(rename = "public-read")]
    PublicRead,
    #[serde(rename = "private")]
    Private,
}

impl OssAcl {}

pub trait OSSQuery {
    fn to_query(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListObject2Query {
    #[serde(rename = "list-type")]
    pub list_type: i32,
    pub delimiter: Option<String>,
    #[serde(rename = "start-after")]
    pub start_after: Option<String>,
    #[serde(rename = "continuation-token")]
    pub continuation_token: Option<String>,
    #[serde(rename = "max-keys")]
    pub max_keys: Option<i32>,
    pub prefix: Option<String>,
    #[serde(rename = "encoding-type")]
    pub encoding_type: Option<String>,
    #[serde(rename = "fetch-owner")]
    pub fetch_owner: Option<bool>,
}

impl Default for ListObject2Query {
    fn default() -> Self {
        ListObject2Query {
            list_type: 2,
            delimiter: None,
            start_after: None,
            continuation_token: None,
            max_keys: Some(100),
            prefix: None,
            encoding_type: Some("url".to_string()),
            fetch_owner: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct DescribeRegionsQuery {
    pub regions: Option<String>,
}

impl OSSQuery for DescribeRegionsQuery {
    fn to_query(&self) -> String {
        if let Some(region) = &self.regions {
            format!("regions={}", region)
        } else {
            "regions".to_string()
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: DataRedundancyType
}
