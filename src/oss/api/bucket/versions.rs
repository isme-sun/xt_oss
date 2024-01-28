use crate::oss;
use crate::oss::{
  api::{self, ApiResultFrom},
  entities::version::{VersioningConfiguration, VersioningStatus},
};

use self::builders::PutBucketVersioningBuilder;

pub mod builders {
  use crate::oss::{
    self,
    api::{self, ApiResultFrom},
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

    pub async fn execute(&self) -> api::ApiResult<()> {
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
        .await;
      ApiResultFrom(resp).to_empty().await
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
      .await;
    ApiResultFrom(resp).to_type().await
  }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  pub fn PutBucketVersioning(&self, status: VersioningStatus) -> PutBucketVersioningBuilder {
    PutBucketVersioningBuilder::new(self, status)
  }

  pub fn GetBucketVersioning(&self) -> GetBucketVersioningBuilder {
    GetBucketVersioningBuilder::new(self)
  }

  pub fn ListObjectVersions() {
    todo!()
  }
}
