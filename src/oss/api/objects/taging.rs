use crate::oss::{self, entities::tag::Tagging};

use builder::{DeleteObjectTaggingBuilder, PutObjectTaggingBuilder};

pub mod builder {
    use std::collections::HashMap;

    use crate::oss::{
        self,
        entities::tag::{Tag, TagSet, Tagging},
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

        /// 添加tag
        pub fn add_tag(mut self, key: &'a str, value: &'a str) -> Self {
            self.tags.insert(key, value);
            self
        }

        /// 移除tag
        pub fn remove_tag(mut self, key: &'a str) -> Self {
            self.tags.remove(key);
            self
        }

        pub fn tagging(&self) -> Tagging {
            let mut tags: Vec<Tag> = Vec::new();
            for (key, value) in self.tags.clone() {
                tags.push(Tag {
                    key: String::from(key),
                    value: String::from(value),
                });
            }
            Tagging {
                tag_set: TagSet { tag: Some(tags) },
            }
        }

        pub fn tagging_xml(&self) -> String {
            quick_xml::se::to_string(&self.tagging()).unwrap()
        }

        pub async fn send(&self) -> oss::Result<()> {
            let res = "tagging";
            let url = format!("{}/{}?{}", self.client.options.base_url(), self.object, res);

            let data = oss::Bytes::from(self.tagging_xml());

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::PUT)
                .resourse(res)
                .body(data)
                .send()
                .await?;

            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: (),
            };
            Ok(result)
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

        pub fn version_id(mut self, value: &'a str) -> Self {
            self.version_id = Some(value);
            self
        }

        pub async fn send(&self) -> oss::Result<()> {
            let res = if let Some(version_id) = self.version_id {
                format!("tagging&versionId={}", version_id)
            } else {
                "tagging".to_string()
            };
            let url = { format!("{}/{}?{}", self.client.options.base_url(), self.object, res) };

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::DELETE)
                .resourse(&res)
                .send()
                .await?;

            let result = oss::Data {
                status: resp.status,
                headers: resp.headers,
                data: (),
            };
            Ok(result)
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

    /// 调用DeleteObjectTagging接口删除指定对象（Object）的标签（Tagging）信息。
    pub fn DeleteObjectTagging(&self, object: &'a str) -> DeleteObjectTaggingBuilder {
        DeleteObjectTaggingBuilder::new(self, object)
    }
}
