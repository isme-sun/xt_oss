use builders::{DeleteObjectBuilder, GetObjectBuilder, PutObjectBuilder};

use crate::oss::{self, api::objects::stand::builders::GetObjectMetaBuilder};

use self::builders::{
  AppendObjectBuilder, CopyObjectBuilder, HeadObjectBuilder, RestoreObjectBuilder,
};

pub mod builders {

  use chrono::{DateTime, Utc};
  use serde::{Deserialize, Serialize};
  use urlencoding;

  use crate::oss::{
    self,
    api::{self, ApiResponseFrom},
    entities::{
      object::{JobParameters, MetadataDirective, RestoreRequest, TaggingDirective, Tier},
      tag::Tagging,
      ObjectACL, ServerSideEncryption, StorageClass,
    },
    http, Bytes, GMT_DATE_FMT,
  };

  pub struct PutObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    headers: Option<http::HeaderMap>,
    content: oss::Bytes,
  }

  impl<'a> PutObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        content: oss::Bytes::new(),
        headers: None,
      }
    }

    pub fn with_content(mut self, content: oss::Bytes) -> Self {
      self.content = content;
      self
    }

    pub fn with_headers(mut self, headers: http::HeaderMap) -> Self {
      self.headers = Some(headers);
      self
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let url = self.client.object_url(self.object);

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::PUT)
        .with_headers(self.headers.as_ref().unwrap().clone())
        .with_body(self.content.to_owned())
        .execute()
        .await?;
      Ok(ApiResponseFrom(resp).as_empty().await)
    }
  }

  #[derive(Debug, Default)]
  struct CopyObjectBuilderArguments<'a> {
    source_version_id: Option<&'a str>,
    version_id: Option<&'a str>,
    forbid_overwrite: Option<bool>,
    if_match: Option<&'a str>,
    if_none_match: Option<&'a str>,
    if_unmodified_since: Option<DateTime<Utc>>,
    if_modified_since: Option<DateTime<Utc>>,
    metadata_directive: Option<MetadataDirective>,
    encryption: Option<ServerSideEncryption>,
    enc_key_id: Option<&'a str>,
    object_acl: Option<ObjectACL>,
    storage_class: Option<StorageClass>,
    oss_tagging: Option<Tagging>,
    tagging_directive: Option<TaggingDirective>,
  }

  #[derive(Debug)]
  pub struct CopyObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    copy_source: &'a str,
    arguments: CopyObjectBuilderArguments<'a>,
  }

  impl<'a> CopyObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        copy_source: Default::default(),
        arguments: CopyObjectBuilderArguments::default(),
      }
    }

    pub fn with_source_version_id(mut self, value: &'a str) -> Self {
      self.arguments.source_version_id = Some(value);
      self
    }

    pub fn with_version_id(mut self, value: &'a str) -> Self {
      self.arguments.version_id = Some(value);
      self
    }

    pub fn with_forbid_overwrite(mut self, value: bool) -> Self {
      self.arguments.forbid_overwrite = Some(value);
      self
    }

    pub fn with_if_match(mut self, value: &'a str) -> Self {
      self.arguments.if_match = Some(value);
      self
    }

    pub fn with_if_none_match(mut self, value: &'a str) -> Self {
      self.arguments.if_none_match = Some(value);
      self
    }

    pub fn with_if_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
      self.arguments.if_unmodified_since = Some(value);
      self
    }

    pub fn with_if_modified_since(mut self, value: DateTime<Utc>) -> Self {
      self.arguments.if_modified_since = Some(value);
      self
    }

    pub fn with_metadata_directive(mut self, value: MetadataDirective) -> Self {
      self.arguments.metadata_directive = Some(value);
      self
    }

    pub fn with_encryption(mut self, value: ServerSideEncryption) -> Self {
      self.arguments.encryption = Some(value);
      self
    }

    pub fn with_enc_key_id(mut self, value: &'a str) -> Self {
      self.arguments.enc_key_id = Some(value);
      self
    }

    pub fn with_object_acl(mut self, value: ObjectACL) -> Self {
      self.arguments.object_acl = Some(value);
      self
    }

    pub fn with_storage_class(mut self, value: StorageClass) -> Self {
      self.arguments.storage_class = Some(value);
      self
    }

    pub fn with_oss_tagging(mut self, value: Tagging) -> Self {
      self.arguments.oss_tagging = Some(value);
      self
    }

    pub fn with_tagging_directive(mut self, value: TaggingDirective) -> Self {
      self.arguments.tagging_directive = Some(value);
      self
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      self.client;
      self.object;
      self.copy_source;
      todo!()
    }
  }

  #[derive(Debug, Default)]
  struct AppendObjectBuilderArguments {
    cache_control: Option<String>,
    content_disposition: Option<String>,
    content_encoding: Option<String>,
    // content_md5: Option<String>,
    expires: Option<DateTime<Utc>>,
    server_side_encryption: Option<ServerSideEncryption>,
    object_acl: Option<ObjectACL>,
    storage_class: Option<StorageClass>,
    // meta: Option<Vec<String>>,
    // tagging: Option<Tagging>,
  }

  pub struct AppendObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: String,
    position: u64,
    arguments: AppendObjectBuilderArguments,
  }

  impl<'a> AppendObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object: object.to_string(),
        position: 0,
        arguments: AppendObjectBuilderArguments::default(),
      }
    }

    pub fn position(mut self, value: u64) -> Self {
      self.position = value;
      self
    }

    pub fn cache_control(mut self, value: &'a str) -> Self {
      self.arguments.cache_control = Some(value.to_string());
      self
    }
    pub fn content_disposition(mut self, value: &'a str) -> Self {
      self.arguments.content_disposition = Some(value.to_string());
      self
    }
    pub fn content_encoding(mut self, value: &str) -> Self {
      self.arguments.content_encoding = Some(value.to_string());
      self
    }

    pub fn expires(mut self, value: DateTime<Utc>) -> Self {
      self.arguments.expires = Some(value);
      self
    }

    pub fn server_side_encryption(mut self, value: ServerSideEncryption) -> Self {
      self.arguments.server_side_encryption = Some(value);
      self
    }

    pub fn object_acl(mut self, value: ObjectACL) -> Self {
      self.arguments.object_acl = Some(value);
      self
    }

    pub fn storage_class(mut self, value: StorageClass) -> Self {
      self.arguments.storage_class = Some(value);
      self
    }

    // pub fn metas(mut self) -> Self {
    //   self
    // }

    // pub fn add_meta(mut self) -> Self {
    //   self
    // }

    // pub fn tagging(mut self) -> Self {
    //   self
    // }

    // pub fn add_tag(mut self) -> Self {
    //   self
    // }

    pub async fn execute(&self) -> api::ApiResult<()> {
      self.client;
      let _ = self.object;
      todo!()
    }
  }

  #[derive(Debug, Default, Serialize, Deserialize)]
  pub struct GetObjectBuilderQuery<'a> {
    #[serde(
      rename = "response-cache-control",
      skip_serializing_if = "Option::is_none"
    )]
    cache_control: Option<&'a str>,
    #[serde(
      rename = "response-content-disposition",
      skip_serializing_if = "Option::is_none"
    )]
    content_disposition: Option<&'a str>,
    #[serde(
      rename = "response-content-encoding",
      skip_serializing_if = "Option::is_none"
    )]
    content_encoding: Option<&'a str>,
    #[serde(
      rename = "response-content-language",
      skip_serializing_if = "Option::is_none"
    )]
    content_language: Option<&'a str>,
    #[serde(
      rename = "response-content-type",
      skip_serializing_if = "Option::is_none"
    )]
    content_type: Option<&'a str>,
    #[serde(rename = "response-expires", skip_serializing_if = "Option::is_none")]
    expires: Option<&'a str>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<&'a str>,
  }

  #[derive(Debug)]
  pub struct GetObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    range: Option<(u64, u64)>,
    modified_since: Option<DateTime<Utc>>,
    unmodified_since: Option<DateTime<Utc>>,
    r#match: Option<&'a str>,
    none_match: Option<&'a str>,
    accept_encoding: Option<&'a str>,
    query: GetObjectBuilderQuery<'a>,
  }

  #[allow(unused)]
  impl<'a> GetObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        range: None,
        r#match: None,
        modified_since: None,
        unmodified_since: None,
        none_match: None,
        accept_encoding: None,
        query: GetObjectBuilderQuery::default(),
      }
    }

    pub fn with_version_id(mut self, value: &'a str) -> Self {
      self.query.version_id = Some(value);
      self
    }

    pub fn with_content_type(mut self, value: &'a str) -> Self {
      self.query.content_type = Some(value);
      self
    }

    pub fn with_content_language(mut self, value: &'a str) -> Self {
      self.query.content_language = Some(value);
      self
    }

    pub fn with_expires(mut self, value: &'a str) -> Self {
      self.query.expires = Some(value);
      self
    }

    pub fn with_cache_control(mut self, value: &'a str) -> Self {
      self.query.cache_control = Some(value);
      self
    }

    pub fn with_content_disposition(mut self, value: &'a str) -> Self {
      self.query.content_disposition = Some(value);
      self
    }

    pub fn with_content_encoding(mut self, value: &'a str) -> Self {
      self.query.content_encoding = Some(value);
      self
    }

    pub fn with_range(mut self, value: (u64, u64)) -> Self {
      self.range = Some(value);
      self
    }

    pub fn with_modified_since(mut self, value: DateTime<Utc>) -> Self {
      self.modified_since = Some(value);
      self
    }

    pub fn with_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
      self.unmodified_since = Some(value);
      self
    }

    pub fn with_match(mut self, value: &'a str) -> Self {
      self.r#match = Some(value);
      self
    }

    pub fn with_none_match(mut self, value: &'a str) -> Self {
      self.none_match = Some(value);
      self
    }

    pub fn with_accept_encoding(mut self, value: &'a str) -> Self {
      self.accept_encoding = Some(value);
      self
    }

    pub(crate) fn query(&self) -> String {
      // dbg!(println!("{:#?}", &self.query));
      serde_qs::to_string(&self.query).unwrap()
    }

    pub(crate) fn headers(&self) -> http::HeaderMap {
      let mut headers = http::HeaderMap::new();
      if let Some(modified_since) = self.modified_since {
        let dt = modified_since.format(oss::GMT_DATE_FMT).to_string();
        headers.append(http::header::IF_MODIFIED_SINCE, dt.parse().unwrap());
      }
      if let Some(unmodified_since) = self.unmodified_since {
        let dt = unmodified_since.format(oss::GMT_DATE_FMT).to_string();
        headers.append(http::header::IF_UNMODIFIED_SINCE, dt.parse().unwrap());
      }
      if let Some(r#match) = self.r#match {
        headers.append(http::header::IF_MATCH, r#match.parse().unwrap());
      }
      if let Some(none_match) = self.none_match {
        headers.append(http::header::IF_MATCH, none_match.parse().unwrap());
      }
      if let Some(accept_encoding) = self.accept_encoding {
        headers.append(
          http::header::ACCEPT_ENCODING,
          accept_encoding.parse().unwrap(),
        );
      }
      headers
    }

    pub async fn execute(&self) -> api::ApiResult<Bytes> {
      let query = self.query();
      let url = if !query.is_empty() {
        format!(
          "{}/{}?{}",
          self.client.options.base_url(),
          self.object,
          query
        )
      } else {
        format!("{}/{}", self.client.options.base_url(), self.object)
      };

      let headers = self.headers();

      let query_origin = urlencoding::decode(&query).unwrap();
      // println!("{}", query_origin);
      let mut task = self
        .client
        .request
        .task()
        .with_resource(&query_origin)
        .with_url(&url);

      let task = if !headers.is_empty() {
        task.with_headers(headers)
      } else {
        task
      };

      let resp = task.execute().await?;

      Ok(ApiResponseFrom(resp).as_bytes().await)
    }
  }

  #[allow(unused)]
  #[derive(Debug)]
  pub struct DeleteObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
  }

  #[allow(unused)]
  impl<'a> DeleteObjectBuilder<'a> {
    pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        version_id: None,
      }
    }

    pub fn with_version_id(mut self, value: &'a str) -> Self {
      self.version_id = Some(value);
      self
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let base_res = format!("/{}/{}", self.client.bucket(), self.object);
      let base_url = self.client.object_url(self.object);
      let version_param = if let Some(version_id) = self.version_id {
        format!("?versionId={}", version_id)
      } else {
        String::new()
      };
      let res = format!("{}{}", base_res, version_param);
      let url = format!("{}{}", base_url, version_param);

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

  #[allow(unused)]
  pub struct HeadObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
    modified_since: Option<DateTime<Utc>>,
    unmodified_since: Option<DateTime<Utc>>,
    r#match: Option<&'a str>,
    none_match: Option<&'a str>,
  }

  #[allow(unused)]
  impl<'a> HeadObjectBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        version_id: None,
        modified_since: None,
        unmodified_since: None,
        r#match: None,
        none_match: None,
      }
    }

    pub fn with_version_id(mut self, version_id: &'a str) -> Self {
      self.version_id = Some(version_id);
      self
    }

    pub fn with_modified_since(mut self, value: DateTime<Utc>) -> Self {
      self.modified_since = Some(value);
      self
    }

    pub fn with_unmodified_since(mut self, value: DateTime<Utc>) -> Self {
      self.unmodified_since = Some(value);
      self
    }

    pub fn with_match(mut self, value: &'a str) -> Self {
      self.r#match = Some(value);
      self
    }

    pub fn with_none_match(mut self, value: &'a str) -> Self {
      self.none_match = Some(value);
      self
    }

    fn headers(&self) -> http::HeaderMap {
      let mut headers = http::HeaderMap::new();
      if let Some(modified_since) = self.modified_since {
        headers.insert(
          "If-Modified-Since",
          modified_since
            .format(GMT_DATE_FMT)
            .to_string()
            .parse()
            .unwrap(),
        );
      }

      if let Some(unmodified_since) = self.unmodified_since {
        headers.insert(
          "If-Modified-Since",
          unmodified_since
            .format(GMT_DATE_FMT)
            .to_string()
            .parse()
            .unwrap(),
        );
      }
      if let Some(r#match) = self.r#match {
        headers.insert("If-Match", r#match.parse().unwrap());
      }
      if let Some(none_match) = self.none_match {
        headers.insert("If-None-Match", none_match.parse().unwrap());
      }
      headers
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let mut res = format!("/{}/{}", self.client.bucket(), self.object);
      let mut url = format!("{}", self.client.object_url(self.object));
      if let Some(version_id) = self.version_id {
        res = format!("{}?versionId={}", res, version_id);
        url = format!("{}?versionId={}", url, version_id);
      };

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::HEAD)
        .with_headers(self.headers())
        .with_resource(&res)
        .execute()
        .await?;

      Ok(ApiResponseFrom(resp).as_empty().await)
    }
  }

  pub struct GetObjectMetaBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
  }

  impl<'a> GetObjectMetaBuilder<'a> {
    pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        version_id: None,
      }
    }

    pub fn with_version_id(mut self, version_id: &'a str) -> Self {
      self.version_id = Some(version_id);
      self
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let mut res = format!(
        "/{}/{}?{}",
        self.client.options.bucket, self.object, "objectMeta"
      );

      let mut url = format!("{}?{}", self.client.object_url(self.object), "objectMeta");

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

  pub struct RestoreObjectBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
    days: Option<u8>,
    tier: Option<Tier>,
  }

  impl<'a> RestoreObjectBuilder<'a> {
    pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        version_id: None,
        days: None,
        tier: None,
      }
    }

    pub fn with_days(mut self, days: u8) -> Self {
      self.days = Some(days);
      self
    }

    pub fn with_tier(mut self, tier: Tier) -> Self {
      self.tier = Some(tier);
      self
    }

    fn config(&self) -> Option<String> {
      let days = self.days?;
      let request = RestoreRequest {
        days,
        job_parameters: self
          .tier
          .as_ref()
          .map(|tier| JobParameters { tier: tier.clone() }),
      };
      quick_xml::se::to_string(&request).ok()
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "restore");
      let mut url = format!("{}?{}", self.client.object_url(self.object), "restore");
      if let Some(version_id) = self.version_id {
        res = format!("{}&versionId={}", res, version_id);
        url = format!("{}&versionId={}", url, version_id);
      };

      let config = Bytes::from(self.config().unwrap_or("".to_string()));

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::POST)
        .with_body(config)
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
  /// 调用PutObject接口上传文件（Object）
  pub fn PutObject(&self, object: &'a str) -> PutObjectBuilder {
    PutObjectBuilder::new(self, object)
  }

  /// GetObject接口用于获取某个文件（Object）。此操作需要对此Object具有读权限
  pub fn GetObject(&self, object: &'a str) -> GetObjectBuilder {
    GetObjectBuilder::new(self, object)
  }

  /// 调用CopyObject接口拷贝同一地域下相同或不同存储空间（Bucket）之间的文件（Object）
  pub fn CopyObject(&self, object: &'a str) -> CopyObjectBuilder {
    CopyObjectBuilder::new(self, object)
  }

  /// 调用AppendObject接口用于以追加写的方式上传文件（Object）。通过AppendObject操
  /// 作创建的Object类型为Appendable Object，而通过PutObject上传的Object是Normal Object。
  pub fn AppendObject(&self, object: &'a str) -> AppendObjectBuilder {
    AppendObjectBuilder::new(self, object)
  }

  /// 调用DeleteObject删除某个文件（Object）
  ///
  /// ### 注意事项
  ///
  /// - 要删除文件，您必须有oss:DeleteObject权限。要删除文件指定版本，您必须具有
  /// oss:DeleteObjectVersion权限。
  /// - 文件删除后无法恢复，请谨慎操作。关于删除文件的更多信息，请参见删除文件。
  /// - 无论要删除的Object是否存在，删除成功后均会返回204状态码。
  /// - 如果Object类型为软链接，使用DeleteObject接口只会删除该软链接。
  pub fn DeleteObject(&self, object: &'a str) -> DeleteObjectBuilder {
    DeleteObjectBuilder::new(self, object)
  }

  /// DeleteMultipleObjects接口用于删除同一个存储空间（Bucket）中的多个文件（Object）
  pub fn DeleteMultipleObjects() {
    todo!()
  }

  /// HeadObject接口用于获取某个文件（Object）的元信息
  pub fn HeadObject(&self, object: &'a str) -> HeadObjectBuilder {
    HeadObjectBuilder::new(self, object)
  }

  /// 调用GetObjectMeta接口获取一个文件（Object）的元数据信息
  ///
  /// 包括该Object的ETag、Size、LastModified信息，并且不返回该Object的内容。
  pub fn GetObjectMeta(&self, object: &'a str) -> GetObjectMetaBuilder {
    GetObjectMetaBuilder::new(self, object)
  }

  /// 调用RestoreObject接口解冻归档类型、冷归档、深度冷归档类型的文件（Object）
  pub fn RestoreObject(&self, object: &'a str) -> RestoreObjectBuilder {
    RestoreObjectBuilder::new(self, object)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::oss;
  use chrono::Utc;
  #[test]
  fn get_object_builder_arugments() {
    let option = oss::Options::default();
    let client = oss::Client::new(option);
    let builder = GetObjectBuilder::new(&client, "example/ex1.txt")
      .with_version_id("version123")
      .with_content_type("text/plain")
      .with_content_language("zh")
      .with_expires("expires")
      .with_cache_control("cache")
      .with_content_disposition("dis")
      .with_content_encoding("GZIP")
      .with_range((0, 100))
      .with_modified_since(Utc::now())
      .with_unmodified_since(Utc::now())
      .with_match("etag")
      .with_none_match("etag")
      .with_accept_encoding("text/plain");

    println!("  query: {}", builder.query());
    println!("headers: {:#?}", builder.headers());
  }
}
