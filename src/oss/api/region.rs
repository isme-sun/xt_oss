use crate::oss::Client;

use self::builder::DescribeRegionsBuilder;

pub mod builder {
    use std::fmt;

    use crate::oss::{
        self,
        entities::region::{RegionInfo, RegionInfoList},
    };

    #[derive(Debug, Default)]
    struct DescribeRegionsQuery<'a> {
        pub regions: Option<&'a str>,
    }

    impl<'a> fmt::Display for DescribeRegionsQuery<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(region) = &self.regions {
                write!(f, "regions={}", region)
            } else {
                write!(f, "regions")
            }
        }
    }

    pub struct DescribeRegionsBuilder<'a> {
        client: &'a oss::Client<'a>,
        query: DescribeRegionsQuery<'a>,
    }

    impl<'a> DescribeRegionsBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                query: DescribeRegionsQuery::default(),
            }
        }

        pub fn regions(mut self, value: &'a str) -> Self {
            self.query.regions = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<Vec<RegionInfo>> {
            let url = {
                let base_url = self.client.options.root_url();
                let query_str = self.query.to_string();
                format!("{base_url}?{query_str}")
            };

            let resp = self.client.request.task().url(&url).send().await.unwrap();

            let content = String::from_utf8_lossy(&resp.data);
            let regoins: RegionInfoList = quick_xml::de::from_str(&content).unwrap();
            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: regoins.region_info,
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
