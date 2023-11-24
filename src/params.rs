use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_qs as qs;

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
            fetch_owner: None
        }
    }
}

#[derive(Debug, Default)]
pub struct DescribeRegionsQuery {
    regions: Option<String>,
}

impl DescribeRegionsQuery {
    pub fn to_query(&self) -> String {
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Regions {
    pub regions: Option<String>,
}

impl Regions {
    pub const ALL: Regions = Regions { regions: None };
}

#[allow(dead_code)]
impl Display for Regions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = if let Some(_) = self.regions {
            qs::to_string(&self).unwrap()
        } else {
            "regions".to_string()
        };
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::Regions;
    // use serde_qs as qs;

    #[test]
    fn params_regions() {
        println!("{}\n", "-".repeat(60));
        // let region = "oss-cn-hangzhou".to_string();
        // let params = Regions{regions: Some(region)};
        let params = Regions { regions: None };
        println!("{}", params);
        println!(
            "{}",
            Regions {
                regions: Some("cn-zhanghao".to_string())
            }
        );
        println!("\n{}", "-".repeat(60));
    }
}