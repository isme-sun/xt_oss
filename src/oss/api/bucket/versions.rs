use crate::oss;
use crate::oss::{
  api::{self, ApiResponseFrom},
  entities::version::{VersioningConfiguration, VersioningStatus},
};

use self::builders::PutBucketVersioningBuilder;

pub mod builders {
  use crate::oss::{
    self,
    api::{self, ApiResponseFrom},
    entities::version::{VersioningConfiguration, VersioningStatus},
    http,
  };

  pub struct PutBucketVersioningBuilder<'a> {
    client: &'a oss::Client<'a>,
    status: VersioningStatus,
  }

  impl<'a> PutBucketVersioningBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, status: VersioningStatus) -> Self {
      Self { client, status }
    }

    pub async fn execute(&self) -> api::ApiResult {
      let res = format!("/{}/?{}", self.client.options.bucket, "versioning");
      let url = format!("{}/?{}", self.client.options.base_url(), "versioning");

      let config = VersioningConfiguration {
        status: Some(self.status.to_owned()),
      };

      let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

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
}

pub struct GetBucketVersioningBuilder<'a> {
  client: &'a oss::Client<'a>,
}

impl<'a> GetBucketVersioningBuilder<'a> {
  pub(crate) fn new(client: &'a oss::Client) -> Self {
    Self { client }
  }
  pub async fn execute(&self) -> api::ApiResult<VersioningConfiguration> {
    let res = format!("/{}/?{}", self.client.options.bucket, "versioning");
    let url = format!("{}/?{}", self.client.options.base_url(), res);
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

/// # 版本控制（Versioning）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// ## 调用PutBucketVersioning设置指定存储空间（Bucket）的版本控制状态。
  pub fn PutBucketVersioning(&self, status: VersioningStatus) -> PutBucketVersioningBuilder {
    PutBucketVersioningBuilder::new(self, status)
  }
  /// ## 接口用于获取指定Bucket的版本控制状态。
  pub fn GetBucketVersioning(&self) -> GetBucketVersioningBuilder {
    GetBucketVersioningBuilder::new(self)
  }

  /// ## 接口用于列出Bucket中包括删除标记（Delete Marker）在内的所有Object的版本信息
  pub fn ListObjectVersions() {
    todo!()
  }
}
