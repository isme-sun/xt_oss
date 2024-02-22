use crate::oss;

use self::builders::{DeleteBucketLifecycleBuilder, GetBucketLifecycleBuilder, PutBucketLifecycleBuilder};

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
            let res = format!("/{}/?{}", self.client.bucket(), "lifecycle");
            let url = format!("{}/?{}", self.client.base_url(), "lifecycle");

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
            Ok(ApiResponseFrom(resp).to_empty().await)
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
            let res = format!("/{}/?{}", self.client.bucket(), "lifecycle");
            let url = format!("{}/?{}", self.client.base_url(), "lifecycle");

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

    pub struct DeleteBucketLifecycleBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketLifecycleBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "lifecycle");
            let url = format!("{}/?{}", self.client.base_url(), "lifecycle");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::DELETE)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 生命周期`Lifecycle``
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 您可以基于最后一次修改时间以及最后一次访问时间的策略创建生命周期规则，定期将存储空间
    /// `Bucket`内的多个文件`Object`转储为指定存储类型,或者将过期的Object和碎片删除,
    /// 从而节省存储费用。本文为您介绍如何调用PutBucketLifecycle接口为存储空间`Bucket``
    /// 设置生命周期规则。
    ///
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketlifecycle)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_lifecycle_put.rs)
    pub fn PutBucketLifecycle(&self) -> PutBucketLifecycleBuilder {
        PutBucketLifecycleBuilder::new(&self)
    }

    /// 调用GetBucketLifecycle接口查看存储空间`Bucket`的生命周期规则`Lifecycle`。
    ///
    /// - [official docs](调用GetBucketLifecycle接口查看存储空间`Bucket`的生命周期规则`Lifecycle`。)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_lifecycle_get.rs)
    #[allow(non_snake_case)]
    pub fn GetBucketLifecycle(&self) -> GetBucketLifecycleBuilder {
        GetBucketLifecycleBuilder::new(&self)
    }

    /// DeleteBucketLifecycle接口用于删除指定存储空间`Bucket`的生命周期规则。
    /// 使用DeleteBucketLifecycle接口删除指定Bucket所有的生命周期规则后,
    /// 该Bucket中的文件`Object`不会被自动删除。只有Bucket的拥有者才能删除该Bucket
    /// 的生命周期规则。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlifecycle)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_lifecycle_del.rs)
    pub fn DeleteBucketLifecycle(&self) -> DeleteBucketLifecycleBuilder {
        DeleteBucketLifecycleBuilder::new(&self)
    }
}
