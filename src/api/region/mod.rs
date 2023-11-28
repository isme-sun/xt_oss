use crate::{
    arguments::{DescribeRegionsQuery, OSSQuery},
    entities::{RegionInfo, RegionInfoList},
    util::Authorization,
    OssClient, OssData, OssResult,
};

#[allow(non_snake_case)]
/// 关于Service操作
impl OssClient {
    /// 调用`ListBuckets（GetService）`接口列举请求者拥有的所有存储空间`（Bucket）`。您还可以通过设置
    /// `prefix`、`marker`或者`max-keys`参数列举满足指定条件的存储空间。
    pub async fn DescribeRegions(
        &self,
        region: DescribeRegionsQuery,
    ) -> OssResult<Vec<RegionInfo>> {
        let url = {
            let base_url = self.options.root_url();
            let query_str = region.to_query();
            format!("{base_url}?{query_str}")
        };

        let auth = Authorization::default();
        let (_status, headers, data) = self.request(url, auth).await?;

        let content = String::from_utf8_lossy(&data);
        let regoins: RegionInfoList = serde_xml_rs::from_str(&content).unwrap();
        let result = OssData {
            headers,
            data: regoins.region_info,
        };
        Ok(result)
    }
}
