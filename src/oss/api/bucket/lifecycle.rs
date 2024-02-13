use crate::oss;

use self::builders::{
    DeleteBucketLifecycleBuilder, GetBucketLifecycleBuilder, PutBucketLifecycleBuilder,
};

pub mod builders {

    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::lifecycle::LifecycleConfiguration,
        http,
    };

    pub struct PutBucketLifecycleBuilder<'a> {
        client: &'a oss::Client<'a>,
        config: LifecycleConfiguration,
    }

    impl<'a> PutBucketLifecycleBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                config: LifecycleConfiguration::default(),
            }
        }

        pub fn with_config(mut self, value: LifecycleConfiguration) -> Self {
            self.config = value;
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "lifecycle");
            let url = format!("{}/?{}", self.client.options.base_url(), "lifecycle");

            let config = quick_xml::se::to_string(&self.config).unwrap();
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
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct GetBucketLifecycleBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketLifecycleBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<LifecycleConfiguration> {
            let res = format!("/{}/?{}", self.client.options.bucket, "lifecycle");
            let url = format!("{}/?{}", self.client.options.base_url(), "lifecycle");

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

    pub struct DeleteBucketLifecycleBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketLifecycleBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let res = format!("/{}/?{}", self.client.options.bucket, "lifecycle");
            let url = format!("{}/?{}", self.client.options.base_url(), "lifecycle");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::DELETE)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_type().await)
        }
    }
}

/// # 生命周期（Lifecycle）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 定的过期时间，自动转换与规则相匹配文件（Object）的存储类型或将其删除。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub async fn PutBucketLifecycle(&self) -> PutBucketLifecycleBuilder {
        PutBucketLifecycleBuilder::new(&self)
    }

    /// 调用GetBucketLifecycle接口查看存储空间（Bucket）的生命周期规则（Lifecycle）。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    #[allow(non_snake_case)]
    pub fn GetBucketLifecycle(&self) -> GetBucketLifecycleBuilder {
        GetBucketLifecycleBuilder::new(&self)
    }

    /// DeleteBucketLifecycle接口用于删除指定存储空间（Bucket）的生命周期规则。使用DeleteBucketLifecycle
    /// 接口删除指定Bucket所有的生命周期规则后，该Bucket中的文件（Object）不会被自动删除。只有Bucket的拥有者
    /// 才能删除该Bucket的生命周期规则。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteBucketLifecycle(&self) -> DeleteBucketLifecycleBuilder {
        DeleteBucketLifecycleBuilder::new(&self)
    }
}
