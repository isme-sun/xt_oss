use crate::oss;

use self::builders::{DeleteBucketCorsBuilder, GetBucketCorsBuilder, PutBucketCorsBuilder};

pub mod builders {
  use crate::oss::{
    self,
    api::{self, ApiResultFrom},
    entities::cors::CORSConfiguration,
    http,
  };

  #[allow(unused)]
  pub struct PutBucketCorsBuilder<'a> {
    client: &'a oss::Client<'a>,
    config: CORSConfiguration,
  }

  #[allow(unused)]
  impl<'a> PutBucketCorsBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
      Self {
        client,
        config: CORSConfiguration::default(),
      }
    }

    pub fn config(mut self, value: CORSConfiguration) -> Self {
      self.config = value;
      self
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let url = format!("{}/?{}", self.client.options.base_url(), "cors");
      let res = format!("/{}/?{}", self.client.options.bucket, "cors");
      let content = quick_xml::se::to_string(&self.config).unwrap();
      let data = oss::Bytes::from(content);
      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::PUT)
        .with_resource(&res)
        .with_body(data)
        .execute_timeout(self.client.options.timeout)
        .await;

      ApiResultFrom(resp).to_type().await
    }
  }

  pub struct GetBucketCorsBuilder<'a> {
    client: &'a oss::Client<'a>,
  }

  impl<'a> GetBucketCorsBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
      Self { client }
    }

    pub async fn execute(&self) -> api::ApiResult<CORSConfiguration> {
      let res = format!("/{}/?{}", self.client.options.bucket, "cors");
      let url = format!("{}/?{}", self.client.options.base_url(), "cors");

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

  pub struct DeleteBucketCorsBuilder<'a> {
    client: &'a oss::Client<'a>,
  }

  impl<'a> DeleteBucketCorsBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
      Self { client }
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let res = format!("/{}/?{}", self.client.options.bucket, "cors");
      let url = format!("{}/?{}", self.client.options.base_url(), "cors");

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_resource(&res)
        .with_method(http::Method::DELETE)
        .execute_timeout(self.client.options.timeout)
        .await;

      ApiResultFrom(resp).to_empty().await
    }
  }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  // 调用PutBucketCors接口为指定的存储空间（Bucket）设置跨域资源共享CORS（Cross-Origin Resource Sharing）规则
  pub fn PutBucketCors(&self) -> PutBucketCorsBuilder {
    PutBucketCorsBuilder::new(&self)
  }

  /// GetBucketCors接口用于获取指定存储空间（Bucket）当前的跨域资源共享CORS（Cross-Origin Resource Sharing）规则。
  pub async fn GetBucketCors(&self) -> GetBucketCorsBuilder {
    GetBucketCorsBuilder::new(&self)
  }

  /// DeleteBucketCors用于关闭指定存储空间（Bucket）对应的跨域资源共享CORS（Cross-Origin Resource Sharing）功能并清空所有规则
  pub async fn DeleteBucketCors(&self) -> DeleteBucketCorsBuilder {
    DeleteBucketCorsBuilder::new(&self)
  }
}
