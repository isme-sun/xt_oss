use crate::oss::Client;

use self::builder::DescribeRegionsBuilder;

pub mod builder {
    use crate::oss::{
        self,
        entities::region::{RegionInfo, RegionInfoList},
        http,
    };

    pub struct DescribeRegionsBuilder<'a> {
        client: &'a oss::Client<'a>,
        region: Option<&'a str>,
        // timeout: Option<u32>
    }

    impl<'a> DescribeRegionsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                region: None,
                // timeout: None,
            }
        }

        pub fn with_region(mut self, value: &'a str) -> Self {
            self.region = Some(value);
            self
        }

        pub async fn execute(&self) -> oss::Result<Vec<RegionInfo>> {
            let url = match self.region {
                Some(region) => format!(
                    "{}://{}.{}/?regions={}",
                    self.client.options.schema(),
                    oss::DEFAULT_REGION,
                    oss::BASE_URL,
                    region
                ),
                None => format!(
                    "{}://{}.{}/?regions",
                    self.client.options.schema(),
                    oss::DEFAULT_REGION,
                    oss::BASE_URL
                ),
            };

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::GET)
                .with_resource("/")
                .with_timeout(self.client.options.timeout)
                .execute()
                .await?;

            let body = String::from_utf8_lossy(&resp.body);
            let regoins: RegionInfoList = quick_xml::de::from_str(&body).unwrap();
            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                body: regoins.region_info,
            };
            Ok(result)
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
