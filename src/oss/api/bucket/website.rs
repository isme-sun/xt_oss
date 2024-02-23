use crate::oss;

use self::builders::{
    DeleteBucketWebsiteBuilder, GetBucketWebsiteBuilder, PutBucketWebsiteBuilder,
};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::website::WebsiteConfiguration,
        http,
    };

    pub struct PutBucketWebsiteBuilder<'a> {
        client: &'a oss::Client<'a>,
        config: WebsiteConfiguration,
    }

    impl<'a> PutBucketWebsiteBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                config: WebsiteConfiguration::default(),
            }
        }

        pub fn with_config(mut self, config: WebsiteConfiguration) -> Self {
            self.config = config;
            self
        }

        pub fn config(&self) -> String {
            quick_xml::se::to_string(&self.config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let res = format!("/{}/?{}", self.client.bucket(), "website");
            let url = format!("{}/?{}", self.client.base_url(), "website");

            let config = self.config();
            let data = oss::Bytes::from(config);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_resource(&res)
                .with_body(data)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }

    pub struct GetBucketWebsiteBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketWebsiteBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<WebsiteConfiguration> {
            let res = format!("/{}/?{}", self.client.bucket(), "website");
            let url = format!("{}/?{}", self.client.base_url(), "website");
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

    pub struct DeleteBucketWebsiteBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketWebsiteBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "website");
            let url = format!("{}/?{}", self.client.base_url(), "website");
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
}

/// # 静态网站`Website``
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutBucketWebsite接口将存储空间`Bucket`设置为静态网站托管模式并设置跳
    /// 转规则`RoutingRule`。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketwebsite)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_website_put.rs)
    pub fn PutBucketWebsite(&self) -> PutBucketWebsiteBuilder {
        PutBucketWebsiteBuilder::new(self)
    }

    /// 调用GetBucketWebsite接口查看存储空间`Bucket`的静态网站托管状态以及跳转规则
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketwebsite)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_website_get.rs)
    pub fn GetBucketWebsite(&self) -> GetBucketWebsiteBuilder {
        GetBucketWebsiteBuilder::new(self)
    }

    /// DeleteBucketWebsite接口用于关闭存储空间`Bucket`的静态网站托管模式以及
    /// 跳转规则。只有Bucket的拥有者才能关闭Bucket的静态网站托管模式。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketwebsite)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_website_del.rs)
    pub fn DeleteBucketWebsite(&self) -> DeleteBucketWebsiteBuilder {
        DeleteBucketWebsiteBuilder::new(self)
    }
}
