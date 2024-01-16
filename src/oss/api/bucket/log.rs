use crate::oss::entities::log::LoggingEnabled;
#[allow(unused)]
use crate::oss::{self, entities::log::BucketLoggingStatus};
/*  日志管理（Logging） */

#[allow(unused)]
pub struct PutBucketLoggingBuilder<'a> {
    pub client: &'a oss::Client<'a>,
    pub bucket_logging_status: BucketLoggingStatus,
}

#[allow(unused)]
impl<'a> PutBucketLoggingBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
        Self {
            client,
            bucket_logging_status: BucketLoggingStatus::default(),
        }
    }

    pub fn with_enabled(mut self, value: bool) -> Self {
        if value == true {
            let bucket = self.client.options.bucket.to_string();
            let mut loggin_enabled = match self.bucket_logging_status.logging_enabled {
                Some(mut loggin_status) => {
                    loggin_status.target_bucket = Some(bucket);
                    loggin_status
                }
                None => LoggingEnabled {
                    target_bucket: Some(bucket),
                    target_prefix: Default::default(),
                },
            };
            self.bucket_logging_status.logging_enabled = Some(loggin_enabled);
        } else {
            self.bucket_logging_status = BucketLoggingStatus::default();
        }
        self
    }

    pub fn with_target_prefix(mut self, value: &'a str) -> Self {
        let bucket = self.client.options.bucket.to_string();
        self = self.with_enabled(true);
        let mut loggin_enabled = self.bucket_logging_status.logging_enabled.unwrap();
        loggin_enabled.target_prefix = Some(value.to_string());
        self.bucket_logging_status.logging_enabled = Some(loggin_enabled);
        self
    }

    pub(crate) fn config(&self) -> String {
        quick_xml::se::to_string(&self.bucket_logging_status).unwrap()
    }

    pub async fn send(&self) -> oss::Result<()> {
        let query = "logging";
        let url = format!("{}/?{}", self.client.options.base_url(), query);
        let config = self.config();

        let data = oss::Bytes::from(config);
        let resp = self
            .client
            .request
            .task()
            .url(&url)
            .method(oss::Method::PUT)
            .body(data)
            .resourse(query)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };
        Ok(result)
    }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketLogging接口用于为存储空间（Bucket）开启日志转存功能，
    /// 可将OSS的访问日志按照固定命名规则，以小时为单位生成日志文件写入您
    /// 指定的Bucket。
    pub fn PutBucketLogging(&self) -> PutBucketLoggingBuilder {
        PutBucketLoggingBuilder::new(&self)
    }

    /// GetBucketLogging接口用于查看存储空间（Bucket）的访问日志配置。
    /// 只有Bucket的拥有者才能查看Bucket的访问日志配置。
    pub async fn GetBucketLogging(&self) -> oss::Result<BucketLoggingStatus> {
        let query = "logging";
        let url = format!("{}/?{}", self.options.base_url(), query);

        let resp = self.request.task().url(&url).resourse(query).send().await?;

        let data = String::from_utf8_lossy(&resp.data);
        let loggin_status = quick_xml::de::from_str(&data).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: loggin_status,
        };

        Ok(result)
    }

    /// DeleteBucketLogging用于关闭存储空间（Bucket）的访问日志记录功能。
    /// 只有Bucket的拥有者才有权限关闭Bucket访问日志记录功能
    pub async fn DeleteBucketLogging(&self) -> oss::Result<()> {
        let query = "logging";
        let url = format!("{}/?{}", self.options.base_url(), query);

        let resp = self
            .request
            .task()
            .url(&url)
            .method(oss::Method::DELETE)
            .resourse(query)
            .send()
            .await?;

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: (),
        };

        Ok(result)
    }
}
