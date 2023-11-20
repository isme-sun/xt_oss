#[derive(Debug, Default)]
pub struct RegionInfo {
    pub region: String,
    pub internet_endpoint: String,
    pub internal_endpoint: String,
    pub accelerate_endpoint: String,
}

pub type RegionInfoList = Vec<RegionInfo>;
