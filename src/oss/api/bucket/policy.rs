use crate::oss;

use self::builders::{DeleteBucketPolicyBuilder, GetBucketPolicyBuilder, PutBucketPolicyBuilder};

pub mod builders {
    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        http, Bytes,
    };

    pub struct PutBucketPolicyBuilder<'a> {
        client: &'a oss::Client<'a>,
        policy: &'a str,
    }

    impl<'a> PutBucketPolicyBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self {
                client,
                policy: Default::default(),
            }
        }

        pub fn with_policy(mut self, value: &'a str) -> Self {
            self.policy = value;
            self
        }

        pub async fn execute(&self) -> api::ApiResult<()> {
            let res = format!("/{}/?{}", self.client.options.bucket, "policy");
            let url = format!("{}/?{}", self.client.options.base_url(), "policy");

            let data = oss::Bytes::from(self.policy.to_string());

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .with_body(data)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_empty().await)
        }
    }

    pub struct GetBucketPolicyBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> GetBucketPolicyBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult<Bytes> {
            let res = format!("/{}/?{}", self.client.options.bucket, "policy");
            let url = format!("{}/?{}", self.client.options.base_url(), "policy");

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;

            Ok(ApiResponseFrom(resp).as_bytes().await)
        }
    }

    pub struct DeleteBucketPolicyBuilder<'a> {
        client: &'a oss::Client<'a>,
    }

    impl<'a> DeleteBucketPolicyBuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client) -> Self {
            Self { client }
        }

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "policy");
            let url = format!("{}/?{}", self.client.options.base_url(), "policy");

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

/// # 授权策略（Policy）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {

    /// PutBucketPolicy接口用于为指定的存储空间（Bucket）设置授权策略（Policy)。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn PutBucketPolicy(&self) -> PutBucketPolicyBuilder {
        PutBucketPolicyBuilder::new(self)
    }

    /// GetBucketPolicy用于获取指定存储空间（Bucket）的权限策略（Policy）。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn GetBucketPolicy(&self) -> GetBucketPolicyBuilder {
        GetBucketPolicyBuilder::new(self)
    }

    /// DeleteBucketPolicy用于删除指定存储空间（Bucket）的权限策略（Policy）。
    /// 
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteBucketPolicy(&self) -> DeleteBucketPolicyBuilder {
        DeleteBucketPolicyBuilder::new(self)
    }
}
