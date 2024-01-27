use crate::oss::{
  self,
  api::{self, ApiResultFrom},
  entities::acl::AccessControlPolicy,
  http,
};

#[derive(Debug)]
pub struct PutBucketAclBuilder<'a> {
  client: &'a oss::Client<'a>,
  acl: oss::entities::OssAcl,
}

#[allow(unused)]
impl<'a> PutBucketAclBuilder<'a> {
  pub fn new(client: &'a oss::Client) -> Self {
    Self {
      client,
      acl: oss::entities::OssAcl::Private,
    }
  }

  pub fn acl(mut self, value: oss::entities::OssAcl) -> Self {
    self.acl = value;
    self
  }

  pub async fn execute(&self) -> api::ApiResult<()> {
    let bucket = self.client.options.bucket;
    let url = { format!("{}/?{}", self.client.options.base_url(), "acl") };
    let res = format!("/{}/?{}", bucket, "acl");

    let mut headers = http::HeaderMap::new();
    headers.insert("x-oss-acl", self.acl.to_string().parse().unwrap());

    let resp = self
      .client
      .request
      .task()
      .with_url(&url)
      .with_method(http::Method::PUT)
      .with_headers(headers)
      .with_resource(&res)
      .execute_timeout(self.client.options.timeout)
      .await;

    ApiResultFrom(resp).to_empty().await
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
    let url = format!("{}/?{}", self.client.options.base_url(), "acl");
    let res = format!("/{}/?{}", self.client.options.bucket, "acl");

    let resp = self
      .client
      .request
      .task()
      .with_url(&url)
      .with_resource(&res)
      .execute_timeout(self.client.options.timeout)
      .await;

    ApiResultFrom(resp).to_type().await
  }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// PutBucketAcl接口用于设置或修改存储空间（Bucket）的访问权限（ACL）。
  pub fn PutBucketAcl(&self) -> PutBucketAclBuilder {
    PutBucketAclBuilder::new(self)
  }

  /// GetBucketAcl接口用于获取某个存储空间（Bucket）的访问权限（ACL）。只有Bucket的拥有者才能获取Bucket的访问权限。
  pub async fn GetBucketAcl(&self) -> GetBucketAclBuilder {
    GetBucketAclBuilder::new(&self)
  }
}
