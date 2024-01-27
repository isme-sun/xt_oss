use crate::oss::Client;

use self::builder::DescribeRegionsBuilder;

pub mod builder {
    use crate::oss::{
        self,
        api::{self, ApiResultFrom},
        entities::region::RegionInfoList,
    };

    pub struct DescribeRegionsBuilder<'a> {
        client: &'a oss::Client<'a>,
        region: Option<&'a str>,
        timeout: Option<u64>,
    }

    impl<'a> DescribeRegionsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                region: None,
                timeout: None,
            }
        }

        pub fn with_region(mut self, value: &'a str) -> Self {
            self.region = Some(value);
            self
        }

        pub fn with_timeout(mut self, value: u64) -> Self {
            self.timeout = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult<RegionInfoList> {
            let base_url = format!(
                "{}://{}.{}",
                self.client.options.schema(),
                oss::DEFAULT_REGION,
                oss::BASE_URL
            );

            let url = match self.region {
                Some(region) => format!("{}/?regions={}", base_url, region),
                None => format!("{}/?regions", base_url),
            };

            let task = self.client.request.task().with_url(&url).with_resource("/");

            let resp = match self.timeout {
                Some(timeout) => task.execute_timeout(timeout).await,
                None => task.execute().await,
            };

            ApiResultFrom(resp).to_type().await
        }
    }
}

#[allow(non_snake_case)]
/// 关于Service操作
impl<'a> Client<'a> {
    pub fn DescribeRegions(&self) -> DescribeRegionsBuilder {
        DescribeRegionsBuilder::new(self)
    }
}
