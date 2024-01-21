use crate::oss::entities::tag::Tagging;
#[allow(unused)]
use crate::oss::{self, Client, Data, Method, Result};

use self::builder::{DeleteBucketTagsBuilder, PutBucketTagsBuilder};

pub mod builder {
    use std::collections::HashMap;

    use crate::oss::{
        self,
        entities::tag::{Tag, TagSet, Tagging},
    };

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

        pub fn delete_key(mut self, value: &'a str) -> Self {
            self.keys.push(value);
            self
        }

        pub fn delete_keys(mut self, value: Vec<&'a str>) -> Self {
            self.keys.extend(value);
            self
        }

        pub async fn send(&self) -> oss::Result<()> {
            let res = "tagging";
            let query = if !self.keys.is_empty() {
                let keys = self.keys.join(",");
                format!("{}={}", res, keys)
            } else {
                res.to_string()
            };
            let url = { format!("{}/?{}", self.client.options.base_url(), query) };

            println!("{}", url);

            let resp = self
                .client
                .request
                .task()
                .url(&url)
                .method(oss::Method::DELETE)
                .resourse(&query)
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
            let url = format!("{}?{}", self.client.options.base_url(), res);

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
}

#[allow(non_snake_case)]
impl<'a> Client<'a> {
    pub fn PutBucketTags(&self) -> PutBucketTagsBuilder {
        PutBucketTagsBuilder::new(self)
    }

    pub async fn GetBucketTags(&self) -> oss::Result<Tagging> {
        let res = "tagging";
        let url = format!("{}?{}", self.options.base_url(), res);

        let resp = self.request.task().url(&url).resourse(res).send().await?;

        let content = String::from_utf8_lossy(&resp.data);
        let tagging: Tagging = quick_xml::de::from_str(&content).unwrap();

        let result = oss::Data {
            status: resp.status,
            headers: resp.headers,
            data: tagging,
        };
        Ok(result)
    }

    pub fn DeleteBucketTags(&self) -> DeleteBucketTagsBuilder {
        DeleteBucketTagsBuilder::new(self)
    }
}
