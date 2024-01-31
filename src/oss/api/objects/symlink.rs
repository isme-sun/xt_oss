use crate::oss;

use self::builders::{GetSymlinkBuilder, PutSymlinkBuilder};

pub mod builders {

  use crate::oss::{
    self,
    api::{self, ApiResponseFrom},
    entities::{ObjectACL, StorageClass},
    http,
  };

  #[derive(Debug)]
  pub struct PutSymlinkBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
    symlink_target: &'a str,
    forbid_overwrite: Option<bool>,
    object_acl: Option<ObjectACL>,
    storage_class: Option<StorageClass>,
  }

  impl<'a> PutSymlinkBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        version_id: None,
        symlink_target: Default::default(),
        forbid_overwrite: None,
        object_acl: None,
        storage_class: None,
      }
    }

    pub fn with_object(mut self, value: &'a str) -> Self {
      self.object = value;
      self
    }

    pub fn with_symlink_target(mut self, value: &'a str) -> Self {
      self.symlink_target = value;
      self
    }

    pub fn with_forbid_overwrite(mut self, value: bool) -> Self {
      self.forbid_overwrite = Some(value);
      self
    }

    pub fn with_object_acl(mut self, value: ObjectACL) -> Self {
      self.object_acl = Some(value);
      self
    }

    pub fn with_storage_class(mut self, value: StorageClass) -> Self {
      self.storage_class = Some(value);
      self
    }

    fn headers(&self) -> http::HeaderMap {
      let mut headers = http::HeaderMap::new();
      headers.insert("x-oss-symlink-target", self.symlink_target.parse().unwrap());
      headers
    }

    pub async fn execute(&self) -> api::ApiResult {
      let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "symlink");
      let mut url = { format!("{}?{}", self.client.object_url(self.object), res) };
      if let Some(version_id) = self.version_id {
        res = format!("{}&versionId={}", res, version_id);
        url = format!("{}&versionId={}", url, version_id);
      }

      let headers = self.headers();

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_headers(headers)
        .with_method(http::Method::PUT)
        .with_resource(&res)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_empty().await)
    }
  }

  pub struct GetSymlinkBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
  }

  impl<'a> GetSymlinkBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        version_id: None,
      }
    }

    pub fn with_object(mut self, value: &'a str) -> Self {
      self.object = value;
      self
    }

    pub fn with_version_id(mut self, value: &'a str) -> Self {
      self.version_id = Some(value);
      self
    }

    pub async fn execute(&self) -> api::ApiResult {
      let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "symlink");
      let mut url = { format!("{}?{}", self.client.object_url(self.object), res) };
      if let Some(version_id) = self.version_id {
        res = format!("{}&versionId={}", res, version_id);
        url = format!("{}&versionId={}", url, version_id);
      }

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_resource(&res)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_empty().await)
    }
  }
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
  /// 初始化一个Multipart Upload事件
  pub fn PutSymlink(&self, object: &'a str) -> PutSymlinkBuilder<'_> {
    PutSymlinkBuilder::new(self, object)
  }

  /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
  /// 上传数据
  pub fn GetSymlink(&self, object: &'a str) -> GetSymlinkBuilder<'_> {
    GetSymlinkBuilder::new(self, object)
  }
}
