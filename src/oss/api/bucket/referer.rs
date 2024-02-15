use crate::oss;

use self::builders::{GetBucketRefererBuilder, PutBucketRefererBuilder};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::referer::RefererConfiguration,
        http,
    };

    #[derive(Debug)]
    pub struct PutBucketRefererBuilder<'a> {
        client: &'a oss::Client<'a>,
        config: RefererConfiguration,
    }

    impl<'a> PutBucketRefererBuilder<'a> {
        pub fn new(cilent: &'a oss::Client) -> Self {
            Self {
                client: cilent,
                config: RefererConfiguration::default(),
            }
        }

        pub fn with_config(mut self, value: RefererConfiguration) -> Self {
            self.config = value;
            self
        }

        fn config(&self) -> String {
            quick_xml::se::to_string(&self.config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "referer");
            let url = format!("{}/?{}", self.client.base_url(), "referer");
            let config = self.config();
            let data = oss::Bytes::from(config);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::PUT)
                .with_body(data)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct GetBucketRefererBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketRefererBuilder<'a> {
        pub fn new(cilent: &'a oss::Client) -> Self {
            Self { client: cilent }
        }

        pub async fn execute(&self) -> api::ApiResult<RefererConfiguration> {
            let res = format!("/{}/?{}", self.client.options.bucket, "referer");
            let url = format!("{}?{}", self.client.options.base_url(), "referer");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

/// # 防盗链（Referer）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutBucketReferer接口设置存储空间（Bucket）级别的Referer访问白名单以及黑名单
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketreferer)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_referer_put.rs)
    pub fn PutBucketReferer(&self) -> PutBucketRefererBuilder {
        PutBucketRefererBuilder::new(self)
    }

    /// GetBucketReferer接口用于查看存储空间（Bucket）的防盗链（Referer）相关配置。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketreferer)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_referer_get.rs)
    pub fn GetBucketReferer(&self) -> GetBucketRefererBuilder {
        GetBucketRefererBuilder::new(self)
    }
}
