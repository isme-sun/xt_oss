use crate::oss;

use self::builders::{
    DeleteBucketLoggingBuilder, GetBucketLoggingBuilder, PutBucketLoggingBuilder,
};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::log::{BucketLoggingStatus, LoggingEnabled},
        http,
    };

    pub struct PutBucketLoggingBuilder<'a> {
        pub client: &'a oss::Client<'a>,
        pub enabled: Option<bool>,
        pub target_prefix: Option<&'a str>,
    }

    impl<'a> PutBucketLoggingBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                enabled: None,
                target_prefix: None,
            }
        }

        pub fn with_enabled(mut self, value: bool) -> Self {
            self.enabled = Some(value);
            self
        }

        pub fn with_target_prefix(mut self, value: &'a str) -> Self {
            self.target_prefix = Some(value);
            self
        }

        // todo 确认生成方式时候合理
        pub(crate) fn config(&self) -> String {
            let config = BucketLoggingStatus {
                logging_enabled: Some(LoggingEnabled {
                    target_bucket: Some(self.client.options.bucket.to_string()),
                    target_prefix: Some(self.target_prefix.unwrap().to_string()),
                }),
            };
            quick_xml::se::to_string(&config).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "logging");
            let url = format!("{}/?{}", self.client.options.base_url(), "logging");
            let config = self.config();
            let data = oss::Bytes::from(config);

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::PUT)
                .with_body(data)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct GetBucketLoggingBuilder<'a> {
        pub client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketLoggingBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<BucketLoggingStatus> {
            let res = format!("{}/?{}", self.client.options.bucket, "logging");
            let url = format!("{}/?{}", self.client.options.base_url(), "logging");

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

    pub struct DeleteBucketLoggingBuilder<'a> {
        pub client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketLoggingBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("{}/?{}", self.client.options.bucket, "logging");
            let url = format!("{}/?{}", self.client.options.base_url(), "logging");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_method(http::Method::DELETE)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }
}

/// # 日志管理（Logging）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketLogging接口用于为存储空间（Bucket）开启日志转存功能，
    /// 可将OSS的访问日志按照固定命名规则，以小时为单位生成日志文件写入您
    /// 指定的Bucket。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn PutBucketLogging(&self) -> PutBucketLoggingBuilder {
        PutBucketLoggingBuilder::new(self)
    }

    /// GetBucketLogging接口用于查看存储空间（Bucket）的访问日志配置。
    /// 只有Bucket的拥有者才能查看Bucket的访问日志配置。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn GetBucketLogging(&self) -> GetBucketLoggingBuilder {
        GetBucketLoggingBuilder::new(&self)
    }

    /// DeleteBucketLogging用于关闭存储空间（Bucket）的访问日志记录功能。
    /// 只有Bucket的拥有者才有权限关闭Bucket访问日志记录功能
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteBucketLogging(&self) -> DeleteBucketLoggingBuilder {
        DeleteBucketLoggingBuilder::new(&self)
    }
}
