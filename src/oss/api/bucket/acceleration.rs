#[allow(unused)]
use crate::oss::{self, api, entities::acceleration::TransferAccelerationConfiguration};

use self::builder::{GetBucketTransferAccelerationBuilder, PutBucketTransferAccelerationBuilder};

pub mod builder {

  use crate::oss::{
    self,
    api::{self, ApiResultFrom},
    entities::acceleration::TransferAccelerationConfiguration,
    http,
  };

  pub struct PutBucketTransferAccelerationBuilder<'a> {
    client: &'a oss::Client<'a>,
    timeout: Option<u64>,
    enabled: Option<bool>,
  }

  impl<'a> PutBucketTransferAccelerationBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, value: bool) -> Self {
      Self {
        client,
        timeout: None,
        enabled: Some(value),
      }
    }

    pub fn with_timeout(mut self, value: u64) -> Self {
      self.timeout = Some(value);
      self
    }

    fn timeout(&self) -> u64 {
      self.timeout.unwrap_or(self.client.options.timeout)
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let base_url = self.client.options.base_url();
      let bucket = self.client.options.bucket;
      let url = format!("{}/?transferAcceleration", base_url);
      let config = TransferAccelerationConfiguration {
        enabled: self.enabled.unwrap(),
      };
      let data = oss::Bytes::from(quick_xml::se::to_string(&config).unwrap());

      let res = format!("/{}/?transferAcceleration", bucket);

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_resource(&res)
        .with_method(http::Method::PUT)
        .with_body(data)
        .execute_timeout(self.timeout())
        .await;

      ApiResultFrom(resp).to_empty().await
    }
  }

  //----------------------------------------------
  pub struct GetBucketTransferAccelerationBuilder<'a> {
    client: &'a oss::Client<'a>,
    timeout: Option<u64>,
  }

  impl<'a> GetBucketTransferAccelerationBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
      Self {
        client,
        timeout: None,
      }
    }

    pub fn with_timeout(mut self, value: u64) -> Self {
      self.timeout = Some(value);
      self
    }

    fn timeout(&self) -> u64 {
      self.timeout.unwrap_or(self.client.options.timeout)
    }

    pub async fn execute(&self) -> api::ApiResult<TransferAccelerationConfiguration> {
      let base_url = self.client.options.base_url();
      let bucket = self.client.options.bucket;
      let url = format!("{}/?transferAcceleration", base_url);
      let res = format!("/{}/?transferAcceleration", bucket);

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_resource(&res)
        .execute_timeout(self.timeout())
        .await;

      ApiResultFrom(resp).to_type().await
    }
  }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// 接口用于为存储空间（Bucket）配置传输加速。开启传输加速后，可提升全球各地用户对OSS的访问速度，适用于远距离数据传输、GB或TB级大文件上传和下载的场景。
  pub fn PutBucketTransferAcceleration(
    &self,
    value: bool,
  ) -> PutBucketTransferAccelerationBuilder<'_> {
    PutBucketTransferAccelerationBuilder::new(&self, value)
  }

  /// 接口用于获取目标存储空间（Bucket）的传输加速配置
  pub async fn GetBucketTransferAcceleration(&self) -> GetBucketTransferAccelerationBuilder {
    GetBucketTransferAccelerationBuilder::new(&self)
  }
}
