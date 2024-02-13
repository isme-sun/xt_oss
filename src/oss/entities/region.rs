use bytes::Bytes;
use serde::{Deserialize, Serialize};
/// OSS 区域信息
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegionInfoList {
    #[serde(rename = "RegionInfo")]
    pub region_info: Vec<RegionInfo>,
}

impl From<Bytes> for RegionInfoList {
    fn from(item: Bytes) -> Self {
        let content = String::from_utf8_lossy(&item);
        quick_xml::de::from_str(&content).unwrap()
    }
}
