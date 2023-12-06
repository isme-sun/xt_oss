use crate::oss::{
    arguments::DescribeRegionsQuery,
    entities::{RegionInfo, RegionInfoList},
    Client, Data, Result,
};

#[allow(non_snake_case)]
/// 关于Service操作
impl<'a> Client<'a> {
    // 调用`ListBuckets（GetService）`接口列举请求者拥有的所有存储空间`（Bucket）`。您还可以通过设置
    // `prefix`、`marker`或者`max-keys`参数列举满足指定条件的存储空间。
    pub async fn DescribeRegions(&self, region: DescribeRegionsQuery) -> Result<Vec<RegionInfo>> {
        let url = {
            let base_url = self.options.root_url();
            let query_str = region.to_string();
            format!("{base_url}?{query_str}")
        };

        let resp = self.request.task().url(&url).send().await.unwrap();

        let content = String::from_utf8_lossy(&resp.data);
        let regoins: RegionInfoList = serde_xml_rs::from_str(&content).unwrap();
        let result = Data {
            status: resp.status,
            headers: resp.headers,
            data: regoins.region_info,
        };
        Ok(result)
    }
}
