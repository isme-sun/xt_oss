use crate::oss;

use self::builders::{
    DeleteBucketEncryptionBuilder, GetBucketEncryptionBuilder, PutBucketEncryptionBuilder,
};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::encryption::{
            ApplyServerSideEncryptionByDefault, SSEAlgorithm, ServerSideEncryptionRule,
        },
        http,
    };

    #[allow(unused)]
    pub struct PutBucketEncryptionBuilder<'a> {
        client: &'a oss::Client<'a>,
        algorithm: SSEAlgorithm,
        data_encryption: Option<&'a str>,
        master_key_id: Option<&'a str>,
    }

    #[allow(unused)]
    impl<'a> PutBucketEncryptionBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                algorithm: SSEAlgorithm::default(),
                data_encryption: None,
                master_key_id: None,
            }
        }

        pub fn with_algorithm(mut self, value: SSEAlgorithm) -> Self {
            self.algorithm = value;
            self
        }

        pub fn with_data_encryption(mut self, value: &'a str) -> Self {
            self.data_encryption = Some(value);
            self
        }

        pub fn with_master_key_id(mut self, value: &'a str) -> Self {
            self.master_key_id = Some(value);
            self
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "encryption");
            let url = format!("{}/?{}", self.client.base_url(), "encryption");

            let mut content = ServerSideEncryptionRule {
                apply_server_side_encryption_by_default: ApplyServerSideEncryptionByDefault {
                    sse_algorithm: self.algorithm,
                    kms_data_encryption: self.data_encryption.map(|enc| enc.into()),
                    kms_master_key_id: self.master_key_id.map(|key_id| key_id.into()),
                },
            };

            let data = oss::Bytes::from(quick_xml::se::to_string(&content).unwrap());

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

    pub struct GetBucketEncryptionBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketEncryptionBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<ServerSideEncryptionRule> {
            let res = format!("/{}/?{}", self.client.bucket(), "encryption");
            let url = format!("{}/?{}", self.client.base_url(), "encryption");
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

    pub struct DeleteBucketEncryptionBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketEncryptionBuilder<'a> {
        pub fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.bucket(), "encryption");
            let url = format!("{}/?{}", self.client.base_url(), "encryption");
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

/// # 加密（Encryption）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketEncryption接口用于配置存储空间（Bucket）的加密规则。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketencryption)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_encryption_put.rs)
    pub fn PutBucketEncryption(&self) -> PutBucketEncryptionBuilder {
        PutBucketEncryptionBuilder::new(self)
    }

    /// GetBucketEncryption接口用于获取存储空间（Bucket）的加密规则。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketencryption)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_encryption_get.rs)
    pub fn GetBucketEncryption(&self) -> GetBucketEncryptionBuilder {
        GetBucketEncryptionBuilder::new(&self)
    }

    /// DeleteBucketEncryption接口用于删除Bucket加密规则。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deletebucketencryption)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_encryption_del.rs)
    pub fn DeleteBucketEncryption(&self) -> DeleteBucketEncryptionBuilder {
        DeleteBucketEncryptionBuilder::new(&self)
    }
}
