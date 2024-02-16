use crate::oss;

use self::builders::{
    AbortBucketWormBuilder, CompleteBucketWormBuilder, ExtendBucketWormBuilder,
    GetBucketWormBuilder, InitiateBucketWormBuilder,
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
            let res = format!("/{}/?{}", self.client.bucket(), "worm");
            let url = format!("{}/?{}", self.client.base_url(), "worm");

            let data = oss::Bytes::from(self.config());
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_resource(&res)
                .with_body(data)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct AbortBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> AbortBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }
        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "worm");
            let url = format!("{}/?{}", self.client.base_url(), "worm");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::DELETE)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct ExtendBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
        worm_id: &'a str,
        days: u32,
    }

    impl<'a> ExtendBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client, worm_id: &'a str) -> Self {
            Self {
                client,
                days: 1,
                worm_id,
            }
        }

        pub fn with_days(mut self, value: u32) -> Self {
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
                self.client.bucket(),
                "wormExtend",
                self.worm_id
            );
            let url = format!(
                "{}/?{}&wormId={}",
                self.client.base_url(),
                "wormExtend",
                self.worm_id
            );
            let config = self.config();
            let data = oss::Bytes::from(config);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_body(data)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
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
            let res = format!("/{}/?wormId={}", self.client.bucket(), self.worm_id);
            let url = format!("{}/?wormId={}", self.client.base_url(), self.worm_id);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::POST)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetBucketWormBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketWormBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<WormConfiguration> {
            let res = format!("/{}/?{}", self.client.bucket(), "worm");
            let url = format!("{}/?{}", self.client.base_url(), "worm");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
        }
    }
}

/// # 合规保留策略（WORM）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用InitiateBucketWorm接口新建一条合规保留策略。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/initiatebucketworm)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_init.rs)
    #[allow(non_snake_case)]
    pub fn InitiateBucketWorm(&self) -> InitiateBucketWormBuilder {
        InitiateBucketWormBuilder::new(self)
    }

    /// 调用InitiateBucketWorm接口新建一条合规保留策略。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/abortbucketworm)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_abort.rs)
    #[allow(non_snake_case)]
    pub fn AbortBucketWorm(&self) -> AbortBucketWormBuilder {
        AbortBucketWormBuilder::new(self)
    }

    /// AbortBucketWorm用于删除未锁定的合规保留策略。
    /// CompleteBucketWorm用于锁定合规保留策略。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/completebucketworm)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_complete.rs)
    #[allow(non_snake_case)]
    pub fn CompleteBucketWorm(&self, worm_id: &'a str) -> CompleteBucketWormBuilder {
        CompleteBucketWormBuilder::new(self, worm_id)
    }

    /// ExtendBucketWorm用于延长已锁定的合规保留策略对应Bucket中Object的保留天数。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/extendbucketworm)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_extend.rs)
    pub fn ExtendBucketWorm(&self, worm_id: &'a str) -> ExtendBucketWormBuilder {
        ExtendBucketWormBuilder::new(self, worm_id)
    }

    /// GetBucketWorm用于获取指定存储空间（Bucket）的合规保留策略信息。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketworm)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_worm_get.rs)
    pub fn GetBucketWorm(&self) -> GetBucketWormBuilder {
        GetBucketWormBuilder::new(self)
    }
}
