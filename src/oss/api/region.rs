use crate::oss::Client;

use self::builder::DescribeRegionsBuilder;

pub mod builder {

    #[allow(unused)]
    use bytes::Bytes;

    #[allow(unused)]
    use crate::oss::{
        self,
        api::{self, into_api_result, Data},
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

            let _result = into_api_result(resp).await;
            // let result = if let Ok(api::ApiResponse::SUCCESS(data)) = result {
            //     let content = String::from_utf8_lossy(&data.content());
            //     let content: RegionInfoList = quick_xml::de::from_str(&content).unwrap();
            //     let target = Data {
            //         url: data.url().clone(),
            //         status: data.status().clone(),
            //         headers: data.headers().clone(),
            //         content
            //     };
            //     Ok(api::ApiResponse::SUCCESS(target))
            // };
            todo!()
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
