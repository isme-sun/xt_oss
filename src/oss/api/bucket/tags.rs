use crate::oss;

use self::builders::{DeleteBucketTagsBuilder, GetBucketTagsBuilder, PutBucketTagsBuilder};

pub mod builders {
    use std::collections::HashMap;

    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
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
                        self.tags
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

        pub async fn execute(&self) -> api::ApiResult {
            let res = format!("/{}/?{}", self.client.options.bucket, "tagging");
            let url = format!("{}?{}", self.client.options.base_url(), "tagging");
            let data = oss::Bytes::from(self.tagging_xml());
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
            Ok(ApiResponseFrom(resp).to_empty().await)
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
            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_resource(&res)
                .execute()
                .await?;
            Ok(ApiResponseFrom(resp).to_type().await)
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
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 标签`Tags``
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// PutBucketTags接口用来给某个存储空间`Bucket`添加或修改标签。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn PutBucketTags(&self) -> PutBucketTagsBuilder {
        PutBucketTagsBuilder::new(self)
    }

    /// GetBucketTags用于获取存储空间`Bucket`的标签信息。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn GetBucketTags(&self) -> GetBucketTagsBuilder {
        GetBucketTagsBuilder::new(self)
    }

    /// DeleteBucketTags接口用于删除存储空间`Bucket`标签。
    ///
    /// - [official docs]()
    /// - [xtoss example]()
    pub fn DeleteBucketTags(&self) -> DeleteBucketTagsBuilder {
        DeleteBucketTagsBuilder::new(self)
    }
}
