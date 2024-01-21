use serde::{Deserialize, Serialize};
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
