use crate::oss;

use self::builders::{DeleteBucketTagsBuilder, GetBucketTagsBuilder, PutBucketTagsBuilder};

pub mod builders {
  use std::collections::HashMap;

  use crate::oss::{
    self,
    api::{self, ApiResultFrom},
    entities::tag::{Tag, TagSet, Tagging},
    http,
  };

  pub struct PutBucketTagsBuilder<'a> {
    client: &'a oss::Client<'a>,
    tags: HashMap<&'a str, &'a str>,
  }

  impl<'a> PutBucketTagsBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
      Self {
        client,
        tags: HashMap::new(),
      }
    }

    pub fn with_tags(mut self, tags: HashMap<&'a str, &'a str>) -> Self {
      self.tags = tags;
      self
    }

    fn tagging(&self) -> Tagging {
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
              .collect::<Vec<Tag>>(),
          ),
        },
      }
    }

    fn tagging_xml(&self) -> String {
      quick_xml::se::to_string(&self.tagging()).unwrap()
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let res = format!("/{}/?{}", self.client.options.bucket, "tagging");
      let url = format!("{}?{}", self.client.options.base_url(), "tagging");

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

  pub struct GetBucketTagsBuilder<'a> {
    client: &'a oss::Client<'a>,
  }

  impl<'a> GetBucketTagsBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
      Self { client }
    }

    pub async fn execute(&self) -> api::ApiResult<Tagging> {
      let res = format!("/{}/?{}", self.client.options.bucket, "tagging");
      let url = format!("{}?{}", self.client.options.base_url(), "tagging");
      ApiResultFrom(
        self
          .client
          .request
          .task()
          .with_url(&url)
          .with_resource(&res)
          .execute()
          .await,
      )
      .to_type()
      .await
    }
  }
  pub struct DeleteBucketTagsBuilder<'a> {
    client: &'a oss::Client<'a>,
    keys: Vec<&'a str>,
  }

  impl<'a> DeleteBucketTagsBuilder<'a> {
    pub fn new(client: &'a oss::Client) -> Self {
      Self {
        client,
        keys: Vec::new(),
      }
    }

    pub fn with_keys(mut self, keys: Vec<&'a str>) -> Self {
      self.keys = keys;
      self
    }

    pub async fn execute(&self) -> api::ApiResult<()> {
      let res = format!("/{}/?{}", self.client.options.bucket, "tagging");
      let url = format!("{}?{}", self.client.options.base_url(), "tagging");
      let url = if !self.keys.is_empty() {
        let keys = self.keys.join(",");
        format!("{}&{}", url, keys)
      } else {
        url
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

#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
  pub fn PutBucketTags(&self) -> PutBucketTagsBuilder {
    PutBucketTagsBuilder::new(self)
  }

  pub fn GetBucketTags(&self) -> GetBucketTagsBuilder {
    GetBucketTagsBuilder::new(self)
  }

  pub fn DeleteBucketTags(&self) -> DeleteBucketTagsBuilder {
    DeleteBucketTagsBuilder::new(self)
  }
}
