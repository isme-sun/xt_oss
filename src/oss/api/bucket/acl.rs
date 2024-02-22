use crate::oss::{
    self,
    api::{self, insert_custom_header, ApiResponseFrom},
    entities::{acl::AccessControlPolicy, OssAcl},
    http,
};

#[derive(Debug)]
pub struct PutBucketAclBuilder<'a> {
    client: &'a oss::Client<'a>,
    acl: OssAcl,
}

#[allow(unused)]
impl<'a> PutBucketAclBuilder<'a> {
    pub fn new(client: &'a oss::Client, acl: OssAcl) -> Self {
        Self { client, acl }
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
        let res = format!("/{}/?{}", self.client.bucket(), "acl");
        let url = { format!("{}/?{}", self.client.base_url(), "acl") };

        let mut headers = http::HeaderMap::new();
        insert_custom_header(&mut headers, "x-oss-acl", self.acl.to_string());

        let resp = self
            .client
            .request
            .task()
            .with_url(&url)
            .with_method(http::Method::PUT)
            .with_headers(headers)
            .with_resource(&res)
            .execute_timeout(self.client.timeout())
            .await?;

        Ok(ApiResponseFrom(resp).to_empty().await)
    }
}

pub struct GetBucketAclBuilder<'a> {
    client: &'a oss::Client<'a>,
}

impl<'a> GetBucketAclBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
        Self { client }
    }

    pub async fn execute(&self) -> api::ApiResult<AccessControlPolicy> {
        let res = format!("/{}/?{}", self.client.bucket(), "acl");
        let url = format!("{}/?{}", self.client.base_url(), "acl");

        let resp = self
            .client
            .request
            .task()
            .with_url(&url)
            .with_resource(&res)
            .execute_timeout(self.client.timeout())
            .await?;
        Ok(ApiResponseFrom(resp).to_type().await)
    }
}

/// # 权限控制`ACL``
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketAcl接口用于设置或修改存储空间（Bucket）的访问权限（ACL）。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketacl)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_acl_put.rs)
    pub fn PutBucketAcl(&self, acl: OssAcl) -> PutBucketAclBuilder {
        PutBucketAclBuilder::new(self, acl)
    }

    /// GetBucketAcl接口用于获取某个存储空间（Bucket）的访问权限（ACL）。
    /// 只有Bucket的拥有者才能获取Bucket的访问权限。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketacl)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_acl_get.rs)
    pub fn GetBucketAcl(&self) -> GetBucketAclBuilder {
        GetBucketAclBuilder::new(&self)
    }
}
