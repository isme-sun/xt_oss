use crate::oss::Client;

use self::builders::DescribeRegionsBuilder;

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::region::RegionInfoList,
    };

    pub struct DescribeRegionsBuilder<'a> {
        client: &'a oss::Client<'a>,
        region: Option<&'a str>,
    }

    impl<'a> DescribeRegionsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                region: None,
            }
        }

        pub fn with_region(&mut self, value: &'a str) -> &Self {
            self.region = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<RegionInfoList> {
            let mut url = format!("{}/?regions", self.client.root_url());

            if let Some(region) = self.region {
                url = format!("{}={}", url, region);
            }

            let resp = self.client.request.task().with_url(&url).execute().await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

#[allow(non_snake_case)]
/// 关于Service操作
impl<'a> Client<'a> {
    /// 调用DescribeRegions接口查询所有支持地域或者指定地域对应的Endpoint信息，
    /// 包括外网Endpoint、内网Endpoint和传输加速Endpoint。
    pub fn DescribeRegions(&self) -> DescribeRegionsBuilder {
        DescribeRegionsBuilder::new(self)
    }
}
