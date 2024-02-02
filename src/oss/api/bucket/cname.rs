use crate::oss;
use builders::{CreateCnameTokenBuilder, GetCnameTokenBuilder, ListCnameBuilder, PutCnameBuilder};

use self::builders::DeleteCnameBuilder;

use super::stand::builders::DeleteBucketBuilder;

pub mod builders {
  use crate::oss::{
    self,
    api::{self, ApiResponseFrom},
    entities::cname::{BucketCnameConfiguration, CertificateConfiguration, Cname, CnameToken},
    http,
  };

  pub struct CreateCnameTokenBuilder<'a> {
    client: &'a oss::Client<'a>,
    cname: &'a str,
  }

  impl<'a> CreateCnameTokenBuilder<'a> {
    pub fn new(client: &'a oss::Client, cname: &'a str) -> Self {
      Self { client, cname }
    }

    fn config(&self) -> String {
      let mut config = BucketCnameConfiguration::default();
      config.cname.domain = self.cname.to_string();
      quick_xml::se::to_string(&config).unwrap()
    }

    pub async fn execute(&self) -> api::ApiResult<CnameToken> {
      let res = format!("/{}/?cname&comp=token", self.client.bucket());
      let url = format!("{}/?cname&comp=token", &self.client.base_url());

      let data = oss::Bytes::from(self.config());

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::POST)
        .with_resource(&res)
        .with_body(data)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_type().await)
    }
  }

  pub struct GetCnameTokenBuilder<'a> {
    client: &'a oss::Client<'a>,
    cname: &'a str,
  }

  impl<'a> GetCnameTokenBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, cname: &'a str) -> Self {
      Self { client, cname }
    }

    pub async fn execute(&self) -> api::ApiResult<CnameToken> {
      let res = format!("/{}/?cname={}&comp=token", self.client.bucket(), self.cname);
      let url = format!(
        "{}/?cname={}&comp==token",
        self.client.base_url(),
        self.cname
      );

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

  pub struct PutCnameBuilder<'a> {
    client: &'a oss::Client<'a>,
    bucket_cname_configuration: BucketCnameConfiguration,
  }

  impl<'a> PutCnameBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
      Self {
        client,
        bucket_cname_configuration: BucketCnameConfiguration {
          cname: Cname {
            certificate_configuration: Some(CertificateConfiguration::default()),
            ..Cname::default()
          },
        },
      }
    }

    pub fn config(&self) -> String {
      quick_xml::se::to_string(&self.bucket_cname_configuration).unwrap()
    }

    pub async fn execute(&self) -> api::ApiResult {
      let res = format!("/{}/?cname&comp=add", self.client.bucket());
      let url = format!("{}/?cname&comp=add", self.client.base_url());

      let data = oss::Bytes::from(self.config());

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::POST)
        .with_body(data)
        .with_resource(&res)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_empty().await)
    }
  }

  pub struct ListCnameBuilder<'a> {
    client: &'a oss::Client<'a>,
  }

  impl<'a> ListCnameBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client) -> Self {
      Self { client }
    }

    pub async fn execute(&self) -> api::ApiResult<BucketCnameConfiguration> {
      let res = format!("/{}/?cname&comp=add", self.client.bucket());
      let url = format!("{}/?cname&comp=add", self.client.base_url());

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

  pub struct DeleteCnameBuilder<'a> {
    client: &'a oss::Client<'a>,
    cname: &'a str,
  }

  impl<'a> DeleteCnameBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, cname: &'a str) -> Self {
      Self { client, cname }
    }

    fn config(&self) -> String {
      let mut config = BucketCnameConfiguration::default();
      config.cname.domain = self.cname.to_string();
      quick_xml::se::to_string(&config).unwrap()
    }

    pub async fn execute(&self) -> api::ApiResult<BucketCnameConfiguration> {
      let res = format!("/{}/?cname&comp=delete", self.client.bucket());
      let url = format!("{}/?cname&comp=delete", self.client.base_url());

      let data = oss::Bytes::from(self.config());

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_resource(&res)
        .with_body(data)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_type().await)
    }
  }
}

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// 调用CreateCnameToken接口创建域名所有权验证所需的CnameToken
  pub fn CreateCnameToken(&self, cname: &'a str) -> CreateCnameTokenBuilder<'_> {
    CreateCnameTokenBuilder::new(self, cname)
  }

  /// 调用GetCnameToken接口获取已创建的CnameToken
  pub fn GetCnameToken(&self, cname: &'a str) -> GetCnameTokenBuilder<'_> {
    GetCnameTokenBuilder::new(self, cname)
  }

  /// 调用PutCname接口为某个存储空间（Bucket）绑定自定义域名
  pub fn PutCname(&self) -> PutCnameBuilder<'_> {
    PutCnameBuilder::new(self)
  }

  /// 调用ListCname接口用于查询某个存储空间（Bucket）下绑定的所有的自定义域名（Cname）列表
  pub fn ListCname(&self) -> ListCnameBuilder<'_> {
    ListCnameBuilder::new(self)
  }

  /// 调用DeleteCname接口删除某个存储空间（Bucket）已绑定的Cname
  pub fn DeleteCname(&self, cname: &'a str) -> DeleteCnameBuilder<'_> {
    DeleteCnameBuilder::new(self, cname)
  }
}
