use crate::oss;

use self::builders::{
    CompleteBucketWormBuilder, ExtendBucketWormBuilder, GetBucketWormBuilder,
    InitiateBucketWormBuilder,
};

pub mod builders {

    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::worm::{ExtendWormConfiguration, InitiateWormConfiguration, WormConfiguration},
        http,
    };

    pub struct InitiateBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
        days: i32,
    }

    impl<'a> InitiateBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self { client, days: 1 }
        }

        pub fn with_days(mut self, value: i32) -> Self {
            self.days = value;
            self
        }

        fn config(&self) -> String {
            let config = InitiateWormConfiguration {
                retention_period_in_days: self.days,
            };
            quick_xml::se::to_string(&config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "worm");
            let url = format!("{}/?{}", self.client.options.base_url(), res);

            let config = self.config();

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_body(oss::Bytes::from(config))
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct ExtendBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
        worm_id: Option<&'a str>,
        days: i32,
    }

    impl<'a> ExtendBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                days: 1,
                worm_id: None,
            }
        }

        pub fn with_worm_id(mut self, value: &'a str) -> Self {
            self.worm_id = Some(value);
            self
        }

        pub fn with_days(mut self, value: i32) -> Self {
            self.days = value;
            self
        }

        fn config(&self) -> String {
            let config = ExtendWormConfiguration {
                retention_period_in_days: self.days,
            };
            quick_xml::se::to_string(&config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!(
                "/{}/?{}&wormId={}",
                self.client.options.bucket,
                "wormExtend",
                self.worm_id.unwrap_or_default()
            );
            let url = { format!("{}/?{}", self.client.options.base_url(), res) };
            let config = self.config();

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_body(oss::Bytes::from(config))
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct CompleteBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
        worm_id: &'a str,
    }

    impl<'a> CompleteBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client, worm_id: &'a str) -> Self {
            Self { client, worm_id }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?wormId={}", self.client.options.bucket, self.worm_id);
            let url = format!(
                "{}/?wormId={}",
                self.client.options.base_url(),
                self.worm_id
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct GetBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
        worm_id: &'a str,
    }

    impl<'a> GetBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client, worm_id: &'a str) -> Self {
            Self { client, worm_id }
        }

        pub async fn execute(&self) -> api::ApiResult<WormConfiguration> {
            let res = format!("/{}/?wormId={}", self.client.options.bucket, self.worm_id);
            let url = format!(
                "{}/?wormId={}",
                self.client.options.base_url(),
                self.worm_id
            );

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用InitiateBucketWorm接口新建一条合规保留策略。
    #[allow(non_snake_case)]
    pub fn InitiateBucketWorm(&self) -> InitiateBucketWormBuilder {
        InitiateBucketWormBuilder::new(self)
    }

    /*
    /// AbortBucketWorm用于删除未锁定的合规保留策略。
    pub fn AbortBucketWorm(&self) -> oss::Result<()> {
    }*/

    /// CompleteBucketWorm用于锁定合规保留策略。
    #[allow(non_snake_case)]
    pub fn CompleteBucketWorm(&self, worm_id: &'a str) -> CompleteBucketWormBuilder {
        CompleteBucketWormBuilder::new(self, worm_id)
    }

    /// ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    pub fn ExtendBucketWorm(&self) -> ExtendBucketWormBuilder {
        ExtendBucketWormBuilder::new(self)
    }

    /// GetBucketWorm用于获取指定存储空间（Bucket）的合规保留策略信息。
    pub fn GetBucketWorm(&self, worm_id: &'a str) -> GetBucketWormBuilder {
        GetBucketWormBuilder::new(self, worm_id)
    }
}
