use crate::oss::{self, entities::version::VersioningStatus};

use self::builders::{GetBucketVersioningBuilder, ListObjectVersionsBuilder, PutBucketVersioningBuilder};

pub mod builders {
  use std::fmt;

  use serde::{Deserialize, Serialize};

  use crate::oss::{
    self,
    api::{self, ApiResponseFrom},
    entities::version::{ListVersionsResult, VersioningConfiguration, VersioningStatus},
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
      let res = format!("/{}/?{}", self.client.bucket(), "versioning");
      let url = format!("{}/?{}", self.client.base_url(), "versioning");

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
        .execute_timeout(self.client.timeout())
        .await?;
      Ok(ApiResponseFrom(resp).to_empty().await)
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
      let res = format!("/{}/?{}", self.client.bucket(), "versioning");
      let url = format!("{}/?{}", self.client.base_url(), "versioning");
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

  #[derive(Debug, Serialize, Deserialize, Default)]
  pub(crate) struct ListObjectVersionsQuery<'a> {
    pub delimiter: Option<&'a str>,
    #[serde(rename = "key-marker")]
    pub key_marker: Option<&'a str>,
    #[serde(rename = "version_id_marker")]
    pub version_id_marker: Option<&'a str>,
    #[serde(rename = "max-keys")]
    pub max_keys: Option<u32>,
    pub prefix: Option<&'a str>,
    #[serde(rename = "encoding-type")]
    pub encoding_type: Option<&'a str>,
  }

  impl<'a> fmt::Display for ListObjectVersionsQuery<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", serde_qs::to_string(self).unwrap())
    }
  }

  pub struct ListObjectVersionsBuilder<'a> {
    client: &'a oss::Client<'a>,
    query: ListObjectVersionsQuery<'a>,
  }

  impl<'a> ListObjectVersionsBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
      Self {
        client,
        query: ListObjectVersionsQuery::default(),
      }
    }

    pub fn with_delimiter(mut self, value: &'a str) -> Self {
      self.query.delimiter = Some(value);
      self
    }

    pub fn with_key_marker(mut self, value: &'a str) -> Self {
      self.query.key_marker = Some(value);
      self
    }

    pub fn with_max_keys(mut self, value: u32) -> Self {
      self.query.max_keys = Some(value);
      self
    }

    pub fn with_prefix(mut self, value: &'a str) -> Self {
      self.query.prefix = Some(value);
      self
    }

    pub fn with_version_id_marker(mut self, value: &'a str) -> Self {
      self.query.version_id_marker = Some(value);
      self
    }

    pub fn with_encoding_type(mut self, value: &'a str) -> Self {
      self.query.encoding_type = Some(value);
      self
    }

    pub async fn execute(&self) -> api::ApiResult<ListVersionsResult> {
      let query = self.query.to_string();
      let res = format!("/{}/?versions", self.client.bucket());
      let url = format!("{}/?versions&{}", self.client.base_url(), query);

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
}

/// # 版本控制（Versioning）
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// 调用PutBucketVersioning设置指定存储空间（Bucket）的版本控制状态。
  ///
  /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putbucketversioning)
  /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_version_put.rs)
  pub fn PutBucketVersioning(&self, status: VersioningStatus) -> PutBucketVersioningBuilder {
    PutBucketVersioningBuilder::new(self, status)
  }
  /// 接口用于获取指定Bucket的版本控制状态。
  ///
  /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getbucketversioning)
  /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_bucket_version_get.rs)
  pub fn GetBucketVersioning(&self) -> GetBucketVersioningBuilder {
    GetBucketVersioningBuilder::new(self)
  }

  /// 接口用于列出Bucket中包括删除标记（Delete Marker）在内的所有Object
  /// 的版本信息
  ///
  /// - [official docs]()
  /// - [xtoss example]()
  pub fn ListObjectVersions(&self) -> ListObjectVersionsBuilder {
    ListObjectVersionsBuilder::new(self)
  }
}
