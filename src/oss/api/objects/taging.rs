use crate::oss;

use builders::{DeleteObjectTaggingBuilder, PutObjectTaggingBuilder};

pub mod builders {
  use std::collections::HashMap;

  use crate::oss::{
    self,
    api::{self, ApiResultFrom},
    entities::tag::{Tag, TagSet, Tagging},
    http,
  };

  #[allow(unused)]
  pub struct PutObjectTaggingBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    tags: HashMap<&'a str, &'a str>,
  }

  #[allow(unused)]
  impl<'a> PutObjectTaggingBuilder<'a> {
    pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
      Self {
        client,
        object,
        tags: HashMap::new(),
      }
    }

    pub fn with_tags(mut self, tags: HashMap<&'a str, &'a str>) -> Self {
      self.tags = tags;
      self
    }

    pub fn tagging(&self) -> Tagging {
      Tagging {
        tag_set: TagSet {
          tag: Some(
            self
              .tags
              .iter()
              .map(|entry| Tag {
                key: entry.0.to_string(),
                value: entry.1.to_string(),
              })
              .collect(),
          ),
        },
      }
    }

    pub fn tagging_xml(&self) -> String {
      quick_xml::se::to_string(&self.tagging()).unwrap()
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let res = format!(
        "/{}/{}/?{}",
        self.client.options.bucket, self.object, "tagging"
      );
      let url = format!("{}/{}?{}", self.client.options.base_url(), self.object, res);

      let data = oss::Bytes::from(self.tagging_xml());

      ApiResultFrom(
        self
          .client
          .request
          .task()
          .with_url(&url)
          .with_method(http::Method::PUT)
          .with_resource(&res)
          .with_body(data)
          .execute()
          .await,
      )
      .to_empty()
      .await
    }
  }

  #[allow(unused)]
  pub struct DeleteObjectTaggingBuilder<'a> {
    client: &'a oss::Client<'a>,
    object: &'a str,
    version_id: Option<&'a str>,
  }

  #[allow(unused)]
  impl<'a> DeleteObjectTaggingBuilder<'a> {
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
      let res = format!(
        "/{}/{}/?{}",
        self.client.options.bucket, self.object, "tagging"
      );
      let url = if let Some(version_id) = self.version_id {
        format!(
          "{}{}&versionId={}",
          self.client.options.base_url(),
          res,
          version_id
        )
      } else {
        format!("{}{}", self.client.options.base_url(), res)
      };

      let resp = self
        .client
        .request
        .task()
        .with_url(&url)
        .with_method(http::Method::DELETE)
        .with_resource(&res)
        .execute()
        .await;

      ApiResultFrom(resp).to_empty().await
    }
  }
}

/// 基础操作
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  /// 使用Multipart Upload模式传输数据前，您必须先调用InitiateMultipartUpload接口来通知OSS
  /// 初始化一个Multipart Upload事件
  pub fn PutObjectTagging(&self, object: &'a str) -> PutObjectTaggingBuilder {
    PutObjectTaggingBuilder::new(self, object)
  }

  /*
  /// 初始化一个MultipartUpload后，调用UploadPart接口根据指定的Object名和uploadId来分块（Part）
  /// 上传数据
  pub async fn GetObjectTagging(&self, object: &'a str) -> oss::Result<Tagging> {
      let res = "tagging";
      let url = format!("{}/{}?{}", self.options.base_url(), object, res);

      let resp = self.request.task().url(&url).resourse(res).send().await?;

      let content = String::from_utf8_lossy(&resp.data);
      println!("{}", content);
      let tagging: Tagging = quick_xml::de::from_str(&content).unwrap();

      let result = oss::Data {
          status: resp.status,
          headers: resp.headers,
          data: tagging,
      };
      Ok(result)
  }
  */

  /// 调用DeleteObjectTagging接口删除指定对象（Object）的标签（Tagging）信息。
  pub fn DeleteObjectTagging(&self, object: &'a str) -> DeleteObjectTaggingBuilder {
      DeleteObjectTaggingBuilder::new(self, object)
  }
}
