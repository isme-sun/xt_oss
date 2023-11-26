use serde::{Deserialize, Serialize};
// use serde_qs as qs;

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
