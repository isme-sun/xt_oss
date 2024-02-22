use crate::oss;

use builders::{DeleteObjectTaggingBuilder, GetObjectTaggingbuilder, PutObjectTaggingBuilder};

pub mod builders {
    use std::collections::HashMap;

    use crate::oss::{
        self,
        api::{self, ApiResponseFrom},
        entities::tag::{Tag, TagSet, Tagging},
        http,
    };

    pub struct PutObjectTaggingBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
        tags: HashMap<&'a str, &'a str>,
    }

    impl<'a> PutObjectTaggingBuilder<'a> {
        pub fn new(client: &'a oss::Client, object: &'a str) -> Self {
            Self {
                client,
                object,
                version_id: None,
                tags: HashMap::new(),
            }
        }

        pub fn with_version_id(mut self, value: &'a str) -> Self {
            self.version_id = Some(value);
            self
        }

        pub fn with_tag(mut self, key: &'a str, value: &'a str) -> Self {
            self.tags.insert(key, value);
            self
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
                            .map(|(key, value)| Tag {
                                key: key.to_string(),
                                value: value.to_string(),
                            })
                            .collect(),
                    ),
                },
            }
        }

        fn tagging_xml(&self) -> String {
            quick_xml::se::to_string(&self.tagging()).unwrap()
        }

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "tagging");
            let mut url = format!("{}?{}", self.client.object_url(self.object), "tagging");
            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, &version_id);
                url = format!("{}&versionId={}", url, &version_id);
            }

            let data = oss::Bytes::from(self.tagging_xml());

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

    pub struct GetObjectTaggingbuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

    impl<'a> GetObjectTaggingbuilder<'a> {
        pub(crate) fn new(client: &'a oss::Client, object: &'a str) -> Self {
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

        pub async fn execute(&self) -> api::ApiResult<Tagging> {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "tagging");
            let mut url = format!("{}?{}", self.client.object_url(self.object), "tagging");
            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, &version_id);
                url = format!("{}&versionId={}", url, &version_id);
            }

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

    pub struct DeleteObjectTaggingBuilder<'a> {
        client: &'a oss::Client<'a>,
        object: &'a str,
        version_id: Option<&'a str>,
    }

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

        pub async fn execute(&self) -> api::ApiResult {
            let mut res = format!("/{}/{}?{}", self.client.bucket(), self.object, "tagging");
            let mut url = format!("{}?{}", self.client.object_url(self.object), "tagging");
            if let Some(version_id) = self.version_id {
                res = format!("{}&versionId={}", res, &version_id);
                url = format!("{}&versionId={}", url, &version_id);
            }

            let resp = self
                .client
                .request
                .task()
                .with_url(&url)
                .with_method(http::Method::DELETE)
                .with_resource(&res)
                .execute_timeout(self.client.timeout())
                .await?;

            Ok(ApiResponseFrom(resp).to_empty().await)
        }
    }
}

/// # 标签`Tagging`
#[allow(non_snake_case)]
impl<'a> oss::Client<'a> {
    /// 调用PutObjectTagging接口设置或更新对象`Object`的标签`Tagging`信息。
    /// 对象标签使用一组键值对`Key-Value`标记对象。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/putobjecttagging)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_put.rs)
    pub fn PutObjectTagging(&self, object: &'a str) -> PutObjectTaggingBuilder {
        PutObjectTaggingBuilder::new(self, object)
    }

    /// 调用GetObjectTagging接口获取对象`Object`的标签`Tagging`信息。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/getobjecttagging)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_get.rs)
    pub fn GetObjectTagging(&self, object: &'a str) -> GetObjectTaggingbuilder {
        GetObjectTaggingbuilder::new(&self, object)
    }

    /// 调用DeleteObjectTagging接口删除指定对象`Object`的标签`Tagging`信息。
    ///
    /// - [official docs](https://help.aliyun.com/zh/oss/developer-reference/deleteobjecttagging)
    /// - [xtoss example](https://github.com/isme-sun/xt_oss/blob/main/examples/api_object_tag_del.rs)
    pub fn DeleteObjectTagging(&self, object: &'a str) -> DeleteObjectTaggingBuilder {
        DeleteObjectTaggingBuilder::new(self, object)
    }
}
