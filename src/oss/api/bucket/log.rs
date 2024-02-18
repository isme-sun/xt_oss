use crate::oss;

use self::builders::{DeleteBucketLoggingBuilder, GetBucketLoggingBuilder, PutBucketLoggingBuilder};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::log::{BucketLoggingStatus, LoggingEnabled},
        http,
    };

    pub struct PutBucketLoggingBuilder<'a> {
        client: &'a oss::Client<'a>,
        enabled: Option<bool>,
        bucket: Option<&'a str>,
        target_prefix: Option<&'a str>,
    }

    impl<'a> PutBucketLoggingBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                enabled: None,
                bucket: None,
                target_prefix: None,
            }
        }

        pub fn with_enabled(mut self, value: bool) -> Self {
            self.enabled = Some(value);
            self
        }

        pub fn with_bucket(mut self, value: &'a str) -> Self {
            self.bucket = Some(value);
            self
        }

        pub fn with_target_prefix(mut self, value: &'a str) -> Self {
            self.target_prefix = Some(value);
            self
        }

        // todo 确认生成方式时候合理
        pub(crate) fn config(&self) -> String {
            let config = if self.enabled == Some(true) {
                BucketLoggingStatus {
                    logging_enabled: Some(LoggingEnabled {
                        target_bucket: self.bucket.or(Some(self.client.bucket())).map(|s| s.to_string()),
                        target_prefix: self.target_prefix.map(|s| s.to_string()),
                    }),
                }
            } else {
                BucketLoggingStatus { logging_enabled: None }
            };
            dbg!(&config);
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
            Ok(ApiResponseFrom(resp).to_empty().await)
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
            let res = format!("/{}/?{}", self.client.bucket(), "logging");
            let url = format!("{}/?{}", self.client.base_url(), "logging");

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

    pub struct DeleteBucketLoggingBuilder<'a> {
        pub client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketLoggingBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "logging");
            let url = format!("{}/?{}", self.client.base_url(), "logging");

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

/// # 日志管理（Logging）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketLogging接口用于为存储空间（Bucket）开启日志转存功能，
    /// 可将OSS的访问日志按照固定命名规则，以小时为单位生成日志文件写入您
    /// 指定的Bucket。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketlogging)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_logging_put.rs)
    pub fn PutBucketLogging(&self) -> PutBucketLoggingBuilder {
        PutBucketLoggingBuilder::new(self)
    }

    /// GetBucketLogging接口用于查看存储空间（Bucket）的访问日志配置。
    /// 只有Bucket的拥有者才能查看Bucket的访问日志配置。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketlogging)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_logging_get.rs)
    pub fn GetBucketLogging(&self) -> GetBucketLoggingBuilder {
        GetBucketLoggingBuilder::new(&self)
    }

    /// DeleteBucketLogging用于关闭存储空间（Bucket）的访问日志记录功能。
    /// 只有Bucket的拥有者才有权限关闭Bucket访问日志记录功能
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketlogging)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_logging_del.rs)
    pub fn DeleteBucketLogging(&self) -> DeleteBucketLoggingBuilder {
        DeleteBucketLoggingBuilder::new(&self)
    }
}
